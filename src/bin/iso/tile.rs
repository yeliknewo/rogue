use std::sync::{Arc, RwLock};
use dorp::{Vec3, Id, World, EntityData, Transform};

use iso::iso_data::{IsoData};

pub struct Tile {
    items: Vec<Id>,
    dirty: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            items: vec!(),
            dirty: false,
        }
    }

    pub fn tick(&mut self, my_transform: Option<Arc<RwLock<Transform>>>, world: Arc<World<IsoData>>) {
        let my_transform = match my_transform {
            Some(transform) => {
                transform
            }
            None => {
                panic!("Tile contains no Transform in Tick in Tile");
            }
        };
        if self.dirty {
            let entity_data = world.get_entity_data();
            let entity_data = entity_data.read().expect("Unable to Read Entity Data in Tick in Tile");
            for id in self.items.iter() {
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

    pub fn with_item(mut self, entity_id: Id) -> Tile {
        self.add_item(entity_id);
        self
    }

    pub fn add_item(&mut self, entity_id: Id) {
        self.items.push(entity_id);
        self.dirty = true;
    }

    pub fn rem_item(&mut self, entity_id: Id) {
        for i in 0..self.items.len() {
            if *self.items.get(i).expect("Unable to Get Item in Rem Item in Tile") == entity_id {
                self.items.remove(i);
                break;
            }
        }
    }
}
