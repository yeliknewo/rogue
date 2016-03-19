use std::sync::{Arc, RwLock};

use logic::{World, IDManager, ID};
use graphics::{Window};
use components::{Renderable, Named, Transform};

pub trait EntityData<T: EntityData<T>> : Send  + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T>>);
    fn tick_mut(&mut self, Arc<RwLock<IDManager>>, Arc<World<T>>);
    fn render(&mut self, &mut Window, Arc<World<T>>);
    fn get_renderable(&self) -> Option<Box<Arc<RwLock<Renderable>>>>;
    fn get_named(&self) -> Option<Box<Arc<RwLock<Named>>>>;
    fn get_transform(&self) -> Option<Box<Arc<RwLock<Transform>>>>;
    fn get_id(&self) -> ID;
}
