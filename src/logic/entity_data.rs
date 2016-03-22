use std::sync::{Arc};
use std::fmt::{Display};

use logic::{World, Id, IdManager};
use graphics::{Window, MatrixData};
use components::{Renderable, Named, Transform};

pub trait EntityData<T: EntityData<T, Y>, Y: EntityDataErr> : Send + Sync {
    fn tick(&self, Arc<f64>, Arc<World<T, Y>>);
    fn tick_mut(&mut self, &mut IdManager);
    fn render(&mut self, &mut Window, &mut MatrixData) -> Result<(), Y>;
    fn get_renderable(&self) -> Option<Arc<Renderable>>;
    fn get_named(&self) -> Option<Arc<Named>>;
    fn get_transform(&self) -> Option<Arc<Transform>>;
    fn get_id(&self) -> Id;
}

pub trait EntityDataErr : Send + Sync + Display {

}
