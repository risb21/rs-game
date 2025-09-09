extern crate glium;

use rs_game::{application::Application, geometry::Vertex};
use glium::{winit, Surface};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.5, 1.0);

    // let triangle = vec![
    //     Vertex { position: [-0.5, -0.5 ] },
    //     Vertex { position: [ 0.0,  0.5 ] },
    //     Vertex { position: [ 0.5, -0.25] },
    // ];

    // let vert_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();
    // let indices = glium::index::NoIndices(
    //     glium::index::PrimitiveType::TrianglesList
    // );

    // let vert_shader_src = r#"
    //     #version 140

    //     in vec2 position;
    //     void main() {
    //         gl_Position = vec4(position, 0.0, 1.0);
    //     }
    // "#;

    // let frag_shader_src = r#"
    //     #version 140

    //     out vec4 color;
    //     void main() {
    //         color = vec4(1.0, 0.0, 0.0, 1.0);
    //     }
    // "#;

    // let program = glium::Program::from_source(&display, vert_shader_src, frag_shader_src, None).unwrap();
    // target.draw(&vert_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    // target.finish().unwrap();

    let mut app = Application::new(&event_loop, window, display);
    let _ = event_loop.run_app(&mut app);

}
