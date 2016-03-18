use std::sync::{Arc, RwLock};

use graphics::{EntityDataGraphics, Window};
use logic::{World, UserData, IDManager};

pub struct EntityData<T: UserData<T>> {
    graphics: Option<Arc<RwLock<EntityDataGraphics>>>,
    user: Option<Arc<RwLock<T>>>,
}

impl<T: UserData<T>> EntityData<T> {
    pub fn new() -> EntityData<T> {
        EntityData {
            graphics: None,
            user: None,
        }
    }

    pub fn with_graphics(mut self, graphics: Arc<RwLock<EntityDataGraphics>>) -> EntityData<T> {
        self.graphics = Some(graphics);
        self
    }

    pub fn with_user(mut self, user: Arc<RwLock<T>>) -> EntityData<T> {
        self.user = Some(user);
        self
    }

    pub fn get_graphics_data(&self) -> Option<Arc<RwLock<EntityDataGraphics>>> {
        self.graphics.clone()
    }

    pub fn get_user_data(&self) -> Option<Arc<RwLock<T>>> {
        self.user.clone()
    }

    pub fn tick(&self, delta_time: Arc<f64>, world: Arc<World<T>>) {
        match self.user.clone() {
            Some(data) => {
                data.read().expect("Unable to Read User in Tick in Entity Data").tick(delta_time, world);
            },
            None => (),
        }
    }

    pub fn tick2(&self, manager: Arc<RwLock<IDManager>>, world: Arc<World<T>>) {
        match self.user.clone() {
            Some(data) => {
                data.write().expect("Unable to Write User Data in Tick Mut in Entity Data").tick_mut(manager, world);
            },
            None => (),
        }
    }

    pub fn render(&self, window: &mut Window) {
        match self.graphics.clone() {
            Some(graphics_data) => {
                match self.user.clone() {
                    Some(user_data) => {
                        user_data.write().expect("Unable to Write User Data in Render in Entity Data").render(window, graphics_data);
                    },
                    None => (),
                }
            },
            None => (),
        }
    }
}
