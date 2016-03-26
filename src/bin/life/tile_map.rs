use std::collections::{HashMap};

use dorp::{Id};

pub struct TileMap {
    tiles: HashMap<i32, HashMap<i32, Id>>,
    dirty_tiles: bool,
    ticks: i32,
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            tiles: HashMap::new(),
            dirty_tiles: false,
            ticks: 0,
        }
    }

    pub fn tick_mut(&mut self) {
        if self.dirty_tiles {
            self.ticks += 1;
            if self.ticks > 1 {
                self.dirty_tiles = false;
                self.ticks = 0;
            }
        }
    }

    pub fn add_tile(&mut self, x: i32, y: i32, id: Id) {
        match self.tiles.remove(&y) {
            Some(mut row) => {
                row.insert(x, id);
                self.dirty_tiles = true;
                self.tiles.insert(y, row);
            },
            None => {
                self.tiles.insert(y, HashMap::new());
                self.add_tile(x, y, id);
            }
        }
    }

    pub fn get_tile(&self, x:i32, y: i32) -> Option<Id> {
        match self.tiles.get(&y) {
            Some(row) => match row.get(&x) {
                Some(id) => Some(id.clone()),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_tiles(&self) -> &HashMap<i32, HashMap<i32, Id>> {
        &self.tiles
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_tiles
    }
}
