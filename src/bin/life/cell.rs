use std::sync::{Arc, RwLock};
use std::fmt;
use std::error::Error;

use dorp::{World, Renderable, Id, RenderableErr, TransformErr};

use life::{LifeData, TILE_MAP_NAME, TileMap};

struct Changes {
    on: bool,
    neighbors: Vec<Id>,
    dirty_tick_mut: bool,
    dirty_neighbors: bool,
}

impl Changes {
    pub fn new() -> Changes {
        Changes {
            on: false,
            neighbors: vec!(),
            dirty_tick_mut: false,
            dirty_neighbors: false,
        }
    }
}

pub struct Cell {
    on: bool,
    x: i32,
    y: i32,
    neighbors: Vec<Id>,
    changes: Arc<RwLock<Changes>>,
}

impl Cell {
    pub fn new(x: i32, y: i32, id: Id, on: bool, tile_map: &mut TileMap) -> Cell {
        tile_map.add_tile(x, y, id);
        Cell {
            on: on,
            x: x,
            y: y,
            neighbors: vec!(),
            changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn tick(&self, world: Arc<World<LifeData>>) -> Result<(), CellErr> {
        match world.get_entity_by_name(TILE_MAP_NAME) {
            Some(entity) => {
                match entity.get_tile_map() {
                    Some(tile_map) => {
                        if tile_map.is_dirty() {
                            match self.changes.write() {
                                Ok(mut changes) => {
                                    changes.neighbors = {
                                        let mut vec = vec!();
                                        for y in -1..2 {
                                            for x in -1..2 {
                                                if x != 0 || y != 0 {
                                                    match tile_map.get_tile(self.x + x, self.y + y) {
                                                        Some(tile) => vec.push(tile),
                                                        None => (),
                                                    }
                                                }
                                            }
                                        }
                                        vec
                                    };
                                    changes.dirty_neighbors = true;
                                },
                                Err(_) => return Err(CellErr::Poison("Self Changes Write")),
                            }
                        } else {
                            let mut count = 0;
                            for id in self.neighbors.iter() {
                                match world.get_entity_by_id(id.clone()) {
                                    Some(entity) => {
                                        match entity.get_cell() {
                                            Some(cell) => {
                                                if cell.on {
                                                    count += 1;
                                                }
                                            },
                                            None => return Err(CellErr::Get("Entity Get Cell")),
                                        }
                                    },
                                    None => return Err(CellErr::Get("World Get Entity By Id Neighbor")),
                                }
                            }
                            match count {
                                2 => (),
                                3 => {
                                    match self.changes.write() {
                                        Ok(mut changes) => {
                                            changes.on = true;
                                            changes.dirty_tick_mut = true;
                                        },
                                        Err(_) => return Err(CellErr::Poison("Self Changes Write")),
                                    }
                                }
                                _ => {
                                    match self.changes.write() {
                                        Ok(mut changes) => {
                                            changes.on = false;
                                            changes.dirty_tick_mut = true;
                                        },
                                        Err(_) => return Err(CellErr::Poison("Self Changes Write")),
                                    }
                                }
                            }
                        }
                    },
                    None => return Err(CellErr::Get("Tile Map Entity had no Tile Map")),
                }
            },
            None => return Err(CellErr::Get("World Get Entity By Name TILE_MAP_NAME")),
        }
        Ok(())
    }

    pub fn tick_mut(&mut self, renderable: &mut Renderable) -> Result<(), CellErr> {
        if match self.changes.read() {
            Ok(changes) => changes,
            Err(_) => return Err(CellErr::Poison("Self Changes Read")),
        }.dirty_tick_mut {
            self.on = match self.changes.read() {
                Ok(changes) => changes,
                Err(_) => return Err(CellErr::Poison("Self Changes Read")),
            }.on;
            renderable.set_active(self.on);
            match self.changes.write() {
                Ok(mut changes) => {
                    changes.dirty_tick_mut = false;
                },
                Err(_) => return Err(CellErr::Poison("Self Changes Write")),
            }
        }
        if match self.changes.read() {
            Ok(changes) => changes,
            Err(_) => return Err(CellErr::Poison("Self Changes Read")),
        }.dirty_neighbors {
            self.neighbors = match self.changes.read() {
                Ok(changes) => changes,
                Err(_) => return Err(CellErr::Poison("Self Changes Read")),
            }.neighbors.to_vec();
            match self.changes.write() {
                Ok(mut changes) => {
                    changes.dirty_neighbors = false;
                },
                Err(_) => return Err(CellErr::Poison("Self Changes Write")),
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CellErr {
    Poison(&'static str),
    Transform(&'static str, TransformErr),
    Renderable(&'static str, RenderableErr),
    Get(&'static str),
}

impl fmt::Display for CellErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {

            CellErr::Transform(_, ref err) => err.fmt(f),
            CellErr::Renderable(_, ref err) => err.fmt(f),
            CellErr::Poison(_) => write!(f, "Thread was Poisoned During R/W"),
            CellErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for CellErr {
    fn description(&self) -> &str {
        match *self {
            CellErr::Transform(_, ref err) => err.description(),
            CellErr::Renderable(_, ref err) => err.description(),
            CellErr::Poison(_) => "Thread was Poisoned",
            CellErr::Get(_) => "Get was None",
        }
    }
}
