use std::sync::{Arc};
use std::error::{Error};

use logic::{World, Id, IdManager};
use graphics::{Window, MatrixData};
use components::{Renderable, Named, Transform};

pub trait EntityData<T: EntityData<T>> : Send + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T>>) -> Result<(), Box<Error>>;
    fn tick_mut(&mut self, &mut IdManager, &mut World<T>) -> Result<(), Box<Error>>;
    fn render(&mut self, &mut Window, &mut MatrixData) -> Result<(), Box<Error>>;
    fn get_renderable(&self) -> Option<Arc<Renderable>>;
    fn get_named(&self) -> Option<Arc<Named>>;
    fn get_transform(&self) -> Option<Arc<Transform>>;
    fn get_id(&self) -> Id;
}
