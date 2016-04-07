#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    #[inline]
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex{
            position: position,
        }
    }
}

#[inline]
pub fn init_vertex() {
    implement_vertex!(Vertex, position);
}
