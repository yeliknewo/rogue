use std::sync::{Arc, RwLock};
use std::fmt;
use std::error::Error;
use dorp::{Id, World, EntityData};

use iso::{TileCoords, IsoData};

struct Changes {
    new_touching: Vec<Id>,
    dirty_tick: bool,
    dirty_tick_mut: bool,
}

impl Changes {
    fn new() -> Changes {
        Changes {
            new_touching: vec!(),
            dirty_tick: false,
            dirty_tick_mut: false,
        }
    }
}

pub struct Tile {
    on_tile: Vec<Id>,
    touching: Vec<Id>,
    tile_map: Id,
    changes: Arc<RwLock<Changes>>,
}

impl Tile {
    pub fn new(tile_coords: &TileCoords, tile_map_name: &'static str, world: &World<IsoData>) -> Result<Tile, TileErr> {
        let mut changes = Changes::new();
        changes.dirty_tick = true;
        Ok(
            Tile {
                on_tile: vec!(),
                touching: vec!(),
                tile_map: match world.get_entity_by_name(tile_map_name) {
                    Some(entity) => entity.get_id(),
                    None => return Err(TileErr::Get("World Get Entity By Name Tile Map")),
                },
                changes: Arc::new(RwLock::new(changes)),
            }
        )
    }

    pub fn new_spawn(tile_coords: &TileCoords, tile_map_name: &'static str, world: &mut World<IsoData>) -> Result<Tile, TileErr> {
        let mut changes = Changes::new();
        changes.dirty_tick = true;
        Ok(
            Tile {
                on_tile: vec!(),
                touching: vec!(),
                tile_map: match world.get_entity_by_name(tile_map_name) {
                    Some(entity) => entity.get_id(),
                    None => return Err(TileErr::Get("World Get Entity By Name Tile Map")),
                },
                changes: Arc::new(RwLock::new(changes)),
            }
        )
    }

    pub fn tick(&self, tile_coords: &TileCoords, world: &World<IsoData>) -> Result<(), TileErr> {
        if
            match self.changes.read() {
                Ok(changes) => changes,
                Err(_) => return Err(TileErr::Poison("Self Changes Read")),
            }
        .dirty_tick {
            match world.get_entity_by_id(self.tile_map.clone()) {
                Some(tile_map_entity) => {
                    match tile_map_entity.get_tile_map() {
                        Some(tile_map_component) => {

                        },
                        None => return Err(TileErr::Get("Tile Map Entity Get Tile Map")),
                    }
                },
                None => return Err(TileErr::Get("World Get Entity By Id Tile Map")),
            }
        }
        Ok(())
    }

    pub fn tick_mut(&mut self) -> Result<(), TileErr> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum TileErr {
    Get(&'static str),
    Poison(&'static str),
}

impl fmt::Display for TileErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TileErr::Get(_) => write!(f, "Get was None"),
            TileErr::Poison(_) => write!(f, "Thread was Poisoned at R/W"),
        }
    }
}

impl Error for TileErr {
    fn description(&self) -> &str {
        match *self {
            TileErr::Get(_) => "Get was None",
            TileErr::Poison(_) => "Thread was Poisoned at R/W",
        }
    }
}
