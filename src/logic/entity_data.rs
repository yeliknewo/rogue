use std::sync::{Arc, RwLock};

use logic::{World, IDManager};
use graphics::{Window};
use componenents::{Renderable};

pub trait EntityData<T: EntityData<T>> : Send  + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T>>);
    fn tick_mut(&mut self, Arc<RwLock<IDManager>>, Arc<World<T>>);
    fn render(&mut self, &mut Window);
    fn get_graphics_data(&self) -> Option<Box<Arc<RwLock<Renderable>>>>;
}
