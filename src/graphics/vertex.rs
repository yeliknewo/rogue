use math::{Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coord: [f32; 2]) -> Vertex {
        Vertex{
            position: position,
            tex_coord: tex_coord,
        }
    }
}

impl From<Vec2> for Vertex {
    fn from(other: Vec2) -> Vertex {
        Vertex::new([other[0], other[1], 0.0], other.get_vals())
    }
}

impl From<Vec3> for Vertex {
    fn from(other: Vec3) -> Vertex {
        Vertex::new(other.get_vals(), [other[0], other[1]])
    }
}

pub fn init_vertex() {
    implement_vertex!(Vertex, position, tex_coord);
}
