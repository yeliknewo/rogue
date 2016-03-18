use std::sync::{Arc, RwLock};

use graphics::{EntityDataGraphics};
use logic::{World};

pub struct EntityData<T: Send + Sync> {
    graphics: Option<Arc<RwLock<EntityDataGraphics>>>,
    user: Option<Arc<RwLock<T>>>,
}

impl<T: Send + Sync> EntityData<T> {
    pub fn new() -> EntityData<T> {
        EntityData {
            graphics: None,
            user: None,
        }
    }

    pub fn get_graphics_data(&self) -> Option<Arc<RwLock<EntityDataGraphics>>> {
        self.graphics.clone()
    }

    pub fn get_user_data(&self) -> Option<Arc<RwLock<T>>> {
        self.user.clone()
    }

    pub fn tick(&self, delta_time: &f64, world: &World<T>) {
        match self.graphics.clone() {
            Some(data) => {
                data.read().expect("Unable to Read Graphics in Tick in Entity Data").tick(delta_time, world);
            },
            None => (),
        }
        match self.user.clone() {
            Some(data) => {
                data.read().expect("Unable to Read User in Tick in Entity Data").tick(delta_time, world);
            },
            None => (),
        }
    }
}
