use std::sync::{Arc, RwLock};
use dorp::{Vec3, Id, World, EntityData, Transform};

use iso::{IsoData, TileCoordinates};

//Requires TileCoordinates, Transform

struct Changes {
    on_tile: Vec<Id>,
    off_tile: Vec<Id>,
    touching: Vec<Id>,
    dirty: bool,
}

impl Changes {
    pub fn new() -> Changes {
        Changes {
            on_tile: vec!(),
            off_tile: vec!(),
            dirty: false,
            touching: vec!(),
        }
    }
}

pub struct Tile {
    on_tile: Vec<Id>,
    touching: Vec<Id>,
    changes: Arc<RwLock<Changes>>
}

impl Tile {
    pub fn new(id: Id, tile_map_name: &'static str, tile_coordinates: Arc<RwLock<TileCoordinates>>, world: Arc<World<IsoData>>) -> Tile {
        let mut new_touching = vec!();
        let entity = world.get_entity_by_name(tile_map_name).expect("Unable to Get Entity by Name in Tick Mut in Tile");
        let tile_map = entity.get_tile_map().expect("Unable to Find Tile Map in Tick Mut in Tile");
        let tile_coordinates = tile_coordinates.read().expect("Unable to Read Tile Coordinates in New in Tile");
        tile_map.write().expect("Unable to Write Tile Map in Tick mut in Tile").register_tile_coords(tile_coordinates.get_coords(), id).unwrap();
        let tile_map = tile_map.read().expect("Unable to Write Tile Map in Tick mut in Tile");
        for x in -1..2 {
            for y in -1..2 {
                if x != 0 && y != 0 {
                    match tile_map.get_at(tile_coordinates.get_x() + x, tile_coordinates.get_y() + y) {
                        Some(tile) => {
                            new_touching.push(tile);
                        },
                        None => (),
                    }
                }
            }
        }
        Tile {
            on_tile: vec!(),
            touching: vec!(),
            changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn tick_mut(&mut self, my_transform: Arc<RwLock<Transform>>, world: Arc<World<IsoData>>) {
        let mut changes = self.changes.write().expect("Unable to Write Changes in Tick Mut in Tile");
        if changes.dirty {
            self.on_tile.append(&mut changes.on_tile);
            self.on_tile.sort();
            loop{
                match changes.off_tile.pop() {
                    Some(id) => {
                        let index = self.on_tile.binary_search(&id).expect("Unable to Binary Search in Tick Mut in Tile");
                        self.on_tile.remove(index);
                    },
                    None => break,
                }
            }
            if changes.touching.is_empty() {
                self.touching = changes.touching.to_vec();
            }
            for id in self.on_tile.iter() {
                let entity = world.get_entity_by_id(*id).expect("Unable to Get Entity By Id in Tick Mut in Tile");
                match entity.get_transform() {
                    Some(transform) => {
                        transform.write().expect("Unable to Write Transform in Tick Mut in Tile")
                        .prep_set_position(
                            my_transform.read().expect("Unable to Read Transform in Tick Mut in Tile")
                            .get_position() + Vec3::from([0.25, 0.01, 0.25])
                        );
                    },
                    None => {
                        panic!("On Tile contains no Transform in Tick Mut in Tile");
                    },
                }
            }
            changes.dirty = false;
        }
    }

    pub fn add_to_tile(&self, entity_id: Id) {
        let mut changes = self.changes.write().expect("Unable to Write Changes in Add To Tile in Tile");
        changes.on_tile.push(entity_id);
        changes.dirty = true;
    }

    pub fn remove_from_tile(&self, entity_id: Id) {
        for i in 0..self.on_tile.len() {
            if *self.on_tile.get(i).expect("Unable to Get Item in Rem Item in Tile") == entity_id {
                self.changes.write().expect("Unable to Write Changes in Remove From Tile in Tile").off_tile.push(entity_id);
                break;
            }
        }
    }

    pub fn get_touching(&self) -> Vec<Id> {
        self.touching.to_vec()
    }
}
