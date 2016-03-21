use std::sync::{Arc};

use logic::{EntityData, World, Id};

pub struct Named {
    name: &'static str,
}

impl Named {
    pub fn new<T: EntityData<T>>(name: &'static str, id: Id, world: Arc<World<T>>) -> Named {
        world.register_name(id, name).unwrap();
        Named {
            name: name,
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}
