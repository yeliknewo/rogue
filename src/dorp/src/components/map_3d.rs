use std::collections::{HashMap};
use std::hash::{Hash};

use logic::{Id};

pub struct Map3d<T: Hash + Eq + Copy> {
    tiles: HashMap<T, HashMap<T, HashMap<T, Id>>>,
    dirty_tiles: bool,
    ticks: i32,
}

impl<T: Hash + Eq + Copy> Map3d<T> {
    #[inline]
    pub fn new() -> Map3d<T> {
        Map3d {
            tiles: HashMap::new(),
            dirty_tiles: false,
            ticks: 0,
        }
    }

    #[inline]
    pub fn tick_mut(&mut self) {
        if self.dirty_tiles {
            self.ticks += 1;
            if self.ticks > 1 {
                self.dirty_tiles = false;
                self.ticks = 0;
            }
        }
    }

    #[inline]
    pub fn add_tile(&mut self, x: T, y: T, z: T, id: Id) {
        match self.tiles.remove(&z) {
            Some(mut plane) => {
                match plane.remove(&y) {
                    Some(mut line) => {
                        line.insert(x, id);
                        self.dirty_tiles = true;
                        plane.insert(y, line);
                    },
                    None => {
                        plane.insert(y, HashMap::new());
                        self.add_tile(x, y, z, id);
                    }
                }
                self.tiles.insert(z, plane);
            },
            None => {
                self.tiles.insert(z, HashMap::new());
                self.add_tile(x, y, z, id);
            }
        }
    }

    #[inline]
    pub fn get_tile(&self, x: T, y: T, z: T) -> Option<Id> {
        match self.tiles.get(&z) {
            Some(plane) => match plane.get(&y) {
                Some(row) => match row.get(&x) {
                    Some(id) => Some(*id),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }

    #[inline]
    pub fn get_tiles(&self) -> &HashMap<T, HashMap<T, HashMap<T, Id>>> {
        &self.tiles
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty_tiles
    }
}
