use std::sync::{Arc};
use std::fmt;
use std::error::Error;

use dorp::{
    EntityData, World, IdManager, Id, Window, Renderable, Transform, Named, MatrixData,
    RenderableErr, TransformErr
};

use life::{Scene, SceneErr};

pub struct LifeData {
    id: Id,
    renderable: Option<Arc<Renderable>>,
    transform: Option<Arc<Transform>>,
    named: Option<Arc<Named>>,
    scene: Option<Arc<Scene>>,
}

impl EntityData<LifeData> for LifeData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<LifeData>>) -> Result<(), Box<Error>> {
        Ok(())
    }

    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut World<LifeData>) -> Result<(), Box<Error>> {
        match self.scene.as_mut() {
            Some(scene) => {
                match Arc::get_mut(scene) {
                    Some(scene) => {
                        match scene.tick_mut(manager, world) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(LifeDataErr::Scene("Scene Tick Mut", err))),
                        }
                    },
                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Scene"))),
                }
            },
            None => (),
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), Box<Error>> {
        println!("11");
        match self.renderable.as_mut() {
            Some(renderable) => {
                println!("12");
                match Arc::get_mut(renderable) {
                    Some(renderable) => {
                        println!("13");
                        match self.transform.as_mut() {
                            Some(transform) => {
                                println!("14");
                                match Arc::get_mut(transform) {
                                    Some(transform) => {
                                        println!("15");
                                        match transform.render(renderable) {
                                            Ok(()) => (),
                                            Err(err) => return Err(Box::new(LifeDataErr::Transform("Transform Render", err))),
                                        }
                                        println!("16");
                                    },
                                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Transform"))),
                                }
                            },
                            None => (),
                        }
                        println!("17");
                        match renderable.render(window, matrix_data) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(LifeDataErr::Renderable("Renderable Render", err))),
                        }
                        println!("18");
                    }
                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Renderable"))),
                }
            },
            None => (),
        }
        println!("19");
        Ok(())
    }

    fn get_renderable(&self) -> Option<Arc<Renderable>> {
        self.renderable.clone()
    }

    fn get_transform(&self) -> Option<Arc<Transform>> {
        self.transform.clone()
    }

    fn get_named(&self) -> Option<Arc<Named>> {
        self.named.clone()
    }

    fn get_id(&self) -> Id {
        self.id
    }
}

impl LifeData {
    pub fn new(id: Id) -> LifeData {
        LifeData {
            id: id,
            renderable: None,
            transform: None,
            named: None,
            scene: None,
        }
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> LifeData {
        self.renderable = Some(Arc::new(renderable));
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> LifeData {
        self.transform = Some(Arc::new(transform));
        self
    }

    pub fn with_named(mut self, named: Named) -> LifeData {
        self.named = Some(Arc::new(named));
        self
    }

    pub fn with_scene(mut self, scene: Scene) -> LifeData {
        self.scene = Some(Arc::new(scene));
        self
    }
}

#[derive(Debug)]
pub enum LifeDataErr {
    BadComponentSetup(&'static str),
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    Scene(&'static str, SceneErr),
    GetMut(&'static str),
}

impl fmt::Display for LifeDataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LifeDataErr::BadComponentSetup(_) => write!(f, "This Component Requires Certain Other Components"),
            LifeDataErr::Renderable(_, ref err) => err.fmt(f),
            LifeDataErr::Transform(_, ref err) => err.fmt(f),
            LifeDataErr::GetMut(_) => write!(f, "Get Mut was None"),
            LifeDataErr::Scene(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for LifeDataErr {
    fn description(&self) -> &str {
        match *self {
            LifeDataErr::BadComponentSetup(_) => "Component has unmet component requirements",
            LifeDataErr::Renderable(_, ref err) => err.description(),
            LifeDataErr::Transform(_, ref err) => err.description(),
            LifeDataErr::GetMut(_) => "Get Mut was None",
            LifeDataErr::Scene(_, ref err) => err.description(),
        }
    }
}
