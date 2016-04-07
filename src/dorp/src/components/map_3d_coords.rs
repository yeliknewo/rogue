use std::hash::Hash;

use logic::{Id};
use components::{Map3d};

pub struct Map3dCoords<T: Hash + Eq + Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Hash + Eq + Copy> Map3dCoords<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Map3dCoords<T> {
        Map3dCoords {
            x: x,
            y: y,
            z: z,
        }
    }
    
    #[inline]
    pub fn register(&self, id: Id, map_3d: &mut Map3d<T>) {
        map_3d.add_tile(self.x, self.y, self.z, id);
    }
}
