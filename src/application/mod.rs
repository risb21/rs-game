use glium::{glutin::surface::WindowSurface, winit::{
    application::ApplicationHandler, event::{DeviceEvent, WindowEvent}, event_loop::{self, ActiveEventLoop, EventLoop}, window::{Window, WindowId}
}, Surface, uniform};
use glium::backend::glutin::Display;

use crate::geometry::Vertex;

pub struct Application {
    count: u32,
    window: Window,
    display: Display<WindowSurface>,
    vert_shader: &'static str,
    frag_shader: &'static str,
    
    t: f32,
}

impl Application {
    pub fn new<T>(event_loop: &EventLoop<T>, window: Window, display: Display<WindowSurface>) -> Self {
        // glium::texture:
        Self {
            count: 0,
            window: window,
            display: display,
            vert_shader: r#"
                #version 140

                in vec2 position;
                in vec3 colour;
                out vec3 vertex_colour;

                uniform mat4 matrix;
                
                void main() {
                    vertex_colour = colour;
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            "#,
            frag_shader: r#"
                #version 140

                in vec3 vertex_colour;
                out vec4 color;
                
                void main() {
                    color = vec4(vertex_colour, 1.0);
                }
            "#,
            t: 0.0,
        }
    }
}

impl ApplicationHandler<()> for Application {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::CursorEntered { .. } => {
                self.count += 1;
                println!("#{} Cursor has entered!", self.count);
            },
            WindowEvent::RedrawRequested => {
                self.t += 0.05;

                let uniform = uniform! {
                    matrix: [
                        [ self.t.cos(), self.t.sin(), 0.0, 0.0    ],
                        [-self.t.sin(), self.t.cos(), 0.0, 0.0    ],
                        [     0.0     ,     0.0     , 1.0, 0.0    ],
                        [     0.0     ,     0.0     , 0.0, 1.0f32 ],
                    ]
                };

                let triangle = vec![
                    Vertex { 
                        position: [-0.5, -0.5 ], 
                        colour:   [ 1.0,  0.0,  0.0 ],
                    },
                    Vertex { 
                        position: [ 0.5, -0.5 ],
                        colour:   [ 0.0,  1.0,  0.0 ],
                    },
                    Vertex { 
                        position: [ 0.0,  0.5 ],
                        colour:   [ 0.0,  0.0,  1.0 ],
                    },
                ];


                let vert_buffer = glium::VertexBuffer::new(&self.display, &triangle).unwrap();
                let indices = glium::index::NoIndices(
                    glium::index::PrimitiveType::TrianglesList
                );

                let program = glium::Program::from_source(&self.display, self.vert_shader, self.frag_shader, None).unwrap();

                let mut target = self.display.draw();
                target.clear_color(0.75, 0.0, 0.75, 1.0);
                target.draw(
                    &vert_buffer,
                    &indices,
                    &program,
                    &uniform,
                    &Default::default()
                ).unwrap();
                
                target.finish().unwrap();
            },
            _ => (),
            
        }
    }
    
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        
    }
}

