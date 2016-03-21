use std::sync::{Arc, RwLock};
use std::collections::{VecDeque};

use dorp::{Id, World};

use iso::{IsoData, TileCoordinates};

struct Changes {
    goal: Option<Id>,
    path: VecDeque<Id>,
    next: Option<Id>,
    dirty: bool,
}

impl Changes {
    fn new() -> Changes {
        Changes {
            goal: None,
            path: VecDeque::new(),
            next: None,
            dirty: false,
        }
    }
}

pub struct Being {
    speed: i32,
    changes: Arc<RwLock<Changes>>,
}

impl Being {
    pub fn new(speed: i32) -> Being {
        Being {
            speed: speed,
            changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn with_goal(self, goal: Id) -> Being {
        self.set_goal(goal);
        self
    }

    pub fn set_goal(&self, goal: Id) {
        self.changes.write().expect("Unable to Write Changes in Set Goal in Being").goal = Some(goal);
    }

    pub fn tick(&self, tile_coordinates: Arc<RwLock<TileCoordinates>>, world: Arc<World<IsoData>>) {
        let goal = self.changes.read().expect("Unable to Read Changes in Tick in Being").goal.clone();
        match goal {
            Some(goal) => {
                let tile_coordinates = tile_coordinates.read().expect("Unable to Read TileCoordinates in Tick in Being");
                let entity = world.get_entity_by_id(tile_coordinates.get_tile()).expect("Unable to Get Entity By Id in Tick in Being");
                let tile = entity.get_tile().expect("Unable to Get Tile in Tick in Being");
                //NOTE Add Path to Front
                let mut changes = self.changes.write().expect("Unable to Write Changes in Tick in Being");
                changes.goal = None;
                changes.dirty = true;
            },
            None => (),
        }

        if !self.changes.read().expect("Unable to Read Changes in Tick in Being").path.is_empty() {
            let mut at = self.changes.read().expect("Unable to Read Changes in Tick in Being").path.len() as i32 - self.speed;
            if at < 0 {
                at = 0;
            }
            let mut moves = self.changes.write().expect("Unable to Write Changes in Tick in Being").path.split_off(at as usize);
            if !moves.is_empty() {
                self.changes.write().expect("Unable to Write Changes in Tick in Being").next = moves.remove(0);
            }
        }
    }
}
