use math::{Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 4]) -> Vertex {
        Vertex{
            position: position,
            color: color,
        }
    }
}

impl From<Vec2> for Vertex {
    fn from(other: Vec2) -> Vertex {
        Vertex::new([other[0], other[1], 0.0], [other[0], other[1], other[0], 1.0])
    }
}

impl From<Vec3> for Vertex {
    fn from(other: Vec3) -> Vertex {
        Vertex::new(other.get_vals(), [other[0], other[1], other[2], 1.0])
    }
}

pub fn init_vertex() {
    implement_vertex!(Vertex, position, color);
}
