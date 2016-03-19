use std::sync::{Arc};

use logic::{EntityData, World, ID};

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

    pub fn tick_mut<T: EntityData<T>>(&mut self, id: ID, world: Arc<World<T>>) {
        if self.dirty {
            world.register_name(id, self.name);
            self.dirty = false;
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}
