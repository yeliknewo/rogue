#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    #[inline]
    pub fn new(position: [f32; 3], color: [f32; 4]) -> Vertex {
        Vertex{
            position: position,
            color: color,
        }
    }
}

#[inline]
pub fn init_vertex() {
    implement_vertex!(Vertex, position, color);
}
