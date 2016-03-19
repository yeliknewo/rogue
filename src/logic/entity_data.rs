use std::sync::{Arc, RwLock};

use logic::{World, IdManager, Id};
use graphics::{Window};
use components::{Renderable, Named, Transform};

pub trait EntityData<T: EntityData<T>> : Send  + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T>>);
    fn tick_mut(&mut self, Arc<RwLock<IdManager>>, Arc<World<T>>);
    fn render(&mut self, &mut Window, Arc<World<T>>);
    fn get_renderable(&self) -> Option<Arc<RwLock<Renderable>>>;
    fn get_named(&self) -> Option<Arc<RwLock<Named>>>;
    fn get_transform(&self) -> Option<Arc<RwLock<Transform>>>;
    fn get_id(&self) -> Id;
}
