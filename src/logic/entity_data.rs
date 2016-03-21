use std::sync::{Arc};

use logic::{World, Id};
use graphics::{Window};
use components::{Renderable, Named, Transform};

pub trait EntityData<T: EntityData<T>> : Send + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T>>);
    fn tick_mut(&mut self);
    fn render(&mut self, &mut Window);
    fn get_renderable(&self) -> Option<Arc<Renderable>>;
    fn get_named(&self) -> Option<Arc<Named>>;
    fn get_transform(&self) -> Option<Arc<Transform>>;
    fn get_id(&self) -> Id;
}
