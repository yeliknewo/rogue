use std::sync::{Arc};
use std::fmt;
use std::error::Error;

use dorp::{
    EntityData, World, IdManager, Id, Window, Renderable, Named, Transform, MatrixData,
    RenderableErr
};

pub struct IsoData {
    id: Id,
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {

    }

    fn tick_mut(&mut self, manager: &mut IdManager) {

    }

    fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), Box<Error>> {
        if self.renderable.is_some() {
            match match Arc::get_mut(match self.renderable.as_mut() {
                Some(renderable) => renderable,
                None => return Err(Box::new(IsoDataErr::Get("Renderable Get"))),
            }) {
                Some(renderable) => renderable,
                None => return Err(Box::new(IsoDataErr::GetMut("Arc Renderable Get Mut"))),
            }.render(window, matrix_data) {
                Ok(()) => (),
                Err(err) => return Err(Box::new(IsoDataErr::Renderable("Renderable Render", err))),
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

#[derive(Debug)]
pub enum IsoDataErr {
    Renderable(&'static str, RenderableErr),
    GetMut(&'static str),
    Get(&'static str),
}

impl fmt::Display for IsoDataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IsoDataErr::Renderable(_, ref err) => err.fmt(f),
            IsoDataErr::GetMut(_) => write!(f, "Get Mut was None"),
            IsoDataErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for IsoDataErr {
    fn description(&self) -> &str {
        match *self {
            IsoDataErr::Renderable(_, ref err) => err.description(),
            IsoDataErr::GetMut(_) => "Get Mut was None",
            IsoDataErr::Get(_) => "Get was None",
        }
    }
}
