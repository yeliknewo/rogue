use std::collections::{HashMap};

use dorp::{Id};

//Requires Named to Function

pub struct TileMap {
    map: HashMap<i32, HashMap<i32, Id>>
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            map: HashMap::new(),
        }
    }

    pub fn register_tile_coords(&mut self, coords: (i32, i32), id: Id) -> Result<(), &'static str> {
        self.register_tile(coords.0, coords.1, id)
    }

    pub fn register_tile(&mut self, x:i32, y: i32, id: Id) -> Result<(), &'static str> {
        if self.map.contains_key(&y) {
            let mut y_map = self.map.get_mut(&y).unwrap();
            if y_map.contains_key(&x) {
                return Err("Spot in Tile Map already contains Tile");
            } else {
                y_map.insert(x, id);
                return Ok(());
            }
        } else {
            self.map.insert(y, HashMap::new());
            return self.register_tile(x, y, id);
        }
    }

    pub fn get_at(&self, x: i32, y: i32) -> Option<Id> {
        if self.map.contains_key(&y) {
            let y_map = self.map.get(&y).unwrap();
            if y_map.contains_key(&x) {
                match y_map.get(&x) {
                    Some(id) => Some(id.clone()),
                    None => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_at_xy(&self, coords: (i32, i32)) -> Option<Id> {
        self.get_at(coords.0, coords.1)
    }
}
