use std::sync::{Arc, RwLock};
use dorp::{Vec3, Id, World, EntityData, Transform};

use iso::{IsoData};

pub struct Tile {
    on_tile: Vec<Id>,
    tile_map_name: Option<&'static str>,
    dirty: bool,
    x: i32,
    y: i32,
}

impl Tile {
    pub fn new(tile_map_name: &'static str, x: i32, y: i32) -> Tile {
        Tile {
            on_tile: vec!(),
            tile_map_name: Some(tile_map_name),
            dirty: false,
            x: x,
            y: y,
        }
    }

    pub fn tick_mut(&mut self, my_id: Id, my_transform: Arc<RwLock<Transform>>, world: Arc<World<IsoData>>) {
        match self.tile_map_name {
            Some(tile_map_name) => {
                let entity = world.get_entity_by_name(tile_map_name).expect("Unable to Get Entity by Name in Tick Mut in Tile");
                let entity = entity.read().expect("Unable to Read Entity in Tick Mut in Tile");
                let tile_map = entity.get_tile_map().expect("Unable to Find Tile Map in Tick Mut in Tile");
                tile_map.write().expect("Unable to Write Tile Map in Tick mut in Tile").register_tile(self.x, self.y, my_id).unwrap();
                self.tile_map_name = None;
            },
            None => (),
        }
        if self.dirty {
            let entity_data = world.get_entity_data();
            let entity_data = entity_data.read().expect("Unable to Read Entity Data in Tick in Tile");
            for id in self.on_tile.iter() {
                let entity = entity_data.get(&id).expect("Unable to Get Entity in Tick in Tile");
                let entity = entity.read().expect("Unable to Read Entity in Tick in Tile");
                match entity.get_transform() {
                    Some(transform) => {
                        transform.write().expect("Unable to Write Transform in Tick in Tile")
                        .prep_set_position(
                            my_transform.read().expect("Unable to Read Transform in Tick in Tile")
                            .get_position() + Vec3::from([0.25, 0.01, 0.25])
                        );
                    },
                    None => {
                        panic!("Item contains no Transform in Tick in Tile");
                    },
                }
            }
            self.dirty = false;
        }
    }

    pub fn with_on_tile(mut self, entity_id: Id) -> Tile {
        self.add_to_tile(entity_id);
        self
    }

    pub fn add_to_tile(&mut self, entity_id: Id) {
        self.on_tile.push(entity_id);
        self.dirty = true;
    }

    pub fn remove_from_tile(&mut self, entity_id: Id) {
        for i in 0..self.on_tile.len() {
            if *self.on_tile.get(i).expect("Unable to Get Item in Rem Item in Tile") == entity_id {
                self.on_tile.remove(i);
                break;
            }
        }
    }
}
