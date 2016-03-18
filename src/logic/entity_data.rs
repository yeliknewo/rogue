use std::sync::{Arc, RwLock};

use graphics::{EntityDataGraphics};

pub struct EntityData<T> {
    graphics: Option<Arc<RwLock<EntityDataGraphics>>>,
    user: Option<T>,
}

impl<T> EntityData<T> {
    pub fn get_graphics_data(&self) -> Option<Arc<RwLock<EntityDataGraphics>>> {
        self.graphics.clone()
    }
}
