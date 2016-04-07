use std::hash::Hash;

use logic::{Id};
use components::{Map2d};

pub struct Map2dCoords<T: Hash + Eq + Copy> {
    x: T,
    y: T,
}

impl<T: Hash + Eq + Copy> Map2dCoords<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Map2dCoords<T> {
        Map2dCoords {
            x: x,
            y: y,
        }
    }

    #[inline]
    pub fn register(&self, id: Id, map_2d: &mut Map2d<T>) {
        map_2d.add_tile(self.x, self.y, id);
    }
}
