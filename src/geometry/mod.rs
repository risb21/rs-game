use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub colour:   [f32; 3],
}
implement_vertex!(Vertex, position, colour);