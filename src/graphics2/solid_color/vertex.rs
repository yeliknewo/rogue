use math::{Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex{
            position: position,
        }
    }
}

impl From<Vec2> for Vertex {
    fn from(other: Vec2) -> Vertex {
        Vertex::new([other[0], other[1], 0.0])
    }
}

impl From<Vec3> for Vertex {
    fn from(other: Vec3) -> Vertex {
        Vertex::new(other.get_vals())
    }
}

pub fn init_vertex() {
    implement_vertex!(Vertex, position);
}
