use std::sync::{Arc};
use std::fmt::{Debug};
use std::fmt;

use dorp::{
    EntityData, EntityDataErr, World, IdManager, Id, Window, Renderable, Named, Transform,
    MatrixData, RenderableErr
};

pub struct IsoData {
    id: Id,
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
}

impl EntityData<IsoData, IsoDataErr> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData, IsoDataErr>>) {

    }

    fn tick_mut(&mut self, manager: &mut IdManager) {

    }

    fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), IsoDataErr>{
        if self.renderable.is_some() {
            match match Arc::get_mut(&mut self.renderable.clone().unwrap()) {
                Some(renderable) => renderable,
                None => return Err(IsoDataErr::Render("Unable to Get Mut Renderable")),
            }.render(window, matrix_data) {
                Ok(()) => (),
                Err(err) => return Err(IsoDataErr::RenderRenderable(err)),
            };
        }
        Ok(())
    }

    fn get_renderable(&self) -> Option<Arc<Renderable>> {
        self.renderable.clone()
    }

    fn get_named(&self) -> Option<Arc<Named>> {
        self.named.clone()
    }

    fn get_transform(&self) -> Option<Arc<Transform>> {
        self.transform.clone()
    }

    fn get_id(&self) -> Id {
        self.id
    }
}

impl IsoData {
    pub fn new(id: Id) -> IsoData {
        IsoData {
            id: id,
            transform: None,
            renderable: None,
            named: None,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> IsoData {
        self.transform = Some(Arc::new(transform));
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> IsoData {
        self.renderable = Some(Arc::new(renderable));
        self
    }

    pub fn with_named(mut self, named: Named) -> IsoData {
        self.named = Some(Arc::new(named));
        self
    }
}

pub enum IsoDataErr {
    Render(&'static str),
    RenderRenderable(RenderableErr),
}

impl fmt::Display for IsoDataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IsoDataErr::Render(err) => {
                write!(f, "{}", err);
            },
            &IsoDataErr::RenderRenderable(ref err) => {
                write!(f, "{}", err);
            }
        }
        Ok(())
    }
}

impl EntityDataErr for IsoDataErr {

}
