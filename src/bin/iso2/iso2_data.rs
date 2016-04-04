use std::sync::{Arc};
use std::error::Error;
use std::fmt;

use dorp::{
    EntityData, World, IdManager, Window, SyncData, Renderers, Id, Renderable, Named, Transform,
    RenderableErr, TransformErr
};

use iso2::{Scene, SceneErr};

pub struct Iso2Data {
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
    scene: Option<Arc<Scene>>,
    id: Id,
}

impl Iso2Data {
    pub fn new(id: Id) -> Iso2Data {
        Iso2Data {
            renderable: None,
            named: None,
            transform: None,
            scene: None,
            id: id,
        }
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Iso2Data {
        self.renderable = Some(Arc::new(renderable));
        self
    }

    pub fn with_named(mut self, named: Named) -> Iso2Data {
        self.named = Some(Arc::new(named));
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Iso2Data {
        self.transform = Some(Arc::new(transform));
        self
    }

    pub fn with_scene(mut self, scene: Scene) -> Iso2Data {
        self.scene = Some(Arc::new(scene));
        self
    }
}

impl EntityData<Iso2Data> for Iso2Data {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<Iso2Data>>) -> Result<(), Box<Error>> {
        Ok(())
    }

    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut World<Iso2Data>) -> Result<(), Box<Error>> {
        let id = self.get_id();
        match self.scene.as_mut() {
            Some(scene) => {
                match Arc::get_mut(scene) {
                    Some(scene) => {
                        match scene.tick_mut(id, manager, world) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(Iso2DataErr::Scene("Scene Tick Mut", err))),
                        }
                    },
                    None => return Err(Box::new(Iso2DataErr::GetMut("Arc Get Mut Scene"))),
                }
            },
            None => (),
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, sync_data: &mut SyncData, renderers: &mut Renderers) -> Result<(), Box<Error>> {
        match self.renderable.as_mut() {
            Some(renderable) => {
                match Arc::get_mut(renderable) {
                    Some(renderable) => {
                        match self.transform.as_mut() {
                            Some(transform) => {
                                match Arc::get_mut(transform) {
                                    Some(transform) => transform.render(renderable),
                                    None => return Err(Box::new(Iso2DataErr::GetMut("Arc Get Mut Transform"))),
                                }
                            },
                            None => (),
                        };
                        match renderable.render(window, sync_data, renderers) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(Iso2DataErr::Renderable("Renderable Render", err))),
                        }
                    },
                    None => return Err(Box::new(Iso2DataErr::GetMut("Arc Get Mut Renderable"))),
                }
            },
            None => (),
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

#[derive(Debug)]
pub enum Iso2DataErr {
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    Scene(&'static str, SceneErr),
    GetMut(&'static str),
}

impl fmt::Display for Iso2DataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Iso2DataErr::Renderable(_, ref err) => err.fmt(f),
            Iso2DataErr::Transform(_, ref err) => err.fmt(f),
            Iso2DataErr::Scene(_, ref err) => err.fmt(f),
            Iso2DataErr::GetMut(_) => write!(f, "Get Mut was None"),
        }
    }
}

impl Error for Iso2DataErr {
    fn description(&self) -> &str {
        match *self {
            Iso2DataErr::Renderable(_, ref err) => err.description(),
            Iso2DataErr::Transform(_, ref err) => err.description(),
            Iso2DataErr::Scene(_, ref err) => err.description(),
            Iso2DataErr::GetMut(_) => "Get Mut was None",
        }
    }
}
