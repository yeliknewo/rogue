use std::sync::{Arc, RwLock};

use logic::{World, IDManager};
use graphics::{Window, EntityDataGraphics};

pub trait UserData<T: UserData<T>> : Send  + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T>>);
    fn tick_mut(&mut self, Arc<RwLock<IDManager>>, Arc<World<T>>);
    fn render(&mut self, &mut Window, Arc<RwLock<EntityDataGraphics>>);
}
