use std::sync::{Arc};

use logic::{EntityData, World, Id};

pub struct Named {
    name: &'static str,
    dirty: bool,
}

impl Named {
    pub fn new(name: &'static str) -> Named {
        Named {
            name: name,
            dirty: true,
        }
    }

    pub fn tick_mut<T: EntityData<T>>(&mut self, id: Id, world: Arc<World<T>>) {
        if self.dirty {
            world.register_name(id, self.name).unwrap();
            self.dirty = false;
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}
