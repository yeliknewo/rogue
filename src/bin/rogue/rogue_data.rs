use std::sync::{Arc};
use std::error::Error;
use std::fmt;

use dorp::{
    EntityData, World, IdManager, Window, SyncData, Renderers, Id, Renderable, Named, Transform,
    RenderableErr, TransformErr, TileMap
};

use rogue::{Scene, SceneErr};

pub struct RogueData {
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
    tile_map: Option<Arc<TileMap>>,
    scene: Option<Arc<Scene>>,
    id: Id,
}

impl RogueData {
    pub fn new(id: Id) -> RogueData {
        RogueData {
            renderable: None,
            named: None,
            transform: None,
            tile_map: None,
            scene: None,
            id: id,
        }
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> RogueData {
        self.renderable = Some(Arc::new(renderable));
        self
    }

    pub fn with_named(mut self, named: Named) -> RogueData {
        self.named = Some(Arc::new(named));
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> RogueData {
        self.transform = Some(Arc::new(transform));
        self
    }

    pub fn with_tile_map(mut self, tile_map: TileMap) -> RogueData {
        self.tile_map = Some(Arc::new(tile_map));
        self
    }

    pub fn with_scene(mut self, scene: Scene) -> RogueData {
        self.scene = Some(Arc::new(scene));
        self
    }
}

impl EntityData<RogueData> for RogueData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<RogueData>>) -> Result<(), Box<Error>> {
        Ok(())
    }

    fn tick_mut(&mut self, manager: &mut IdManager, world: &mut World<RogueData>) -> Result<(), Box<Error>> {
        let id = self.get_id();
        match self.scene.as_mut() {
            Some(scene) => {
                match Arc::get_mut(scene) {
                    Some(scene) => {
                        match scene.tick_mut(id, manager, world) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(RogueDataErr::Scene("Scene Tick Mut", err))),
                        }
                    },
                    None => return Err(Box::new(RogueDataErr::GetMut("Arc Get Mut Scene"))),
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
                                    Some(transform) => match transform.render(renderable) {
                                        Ok(()) => (),
                                        Err(err) => return Err(Box::new(RogueDataErr::Transform("Transform Render", err))),
                                    },
                                    None => return Err(Box::new(RogueDataErr::GetMut("Arc Get Mut Transform"))),
                                }
                            },
                            None => (),
                        };
                        match renderable.render(window, sync_data, renderers) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(RogueDataErr::Renderable("Renderable Render", err))),
                        }
                    },
                    None => return Err(Box::new(RogueDataErr::GetMut("Arc Get Mut Renderable"))),
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
pub enum RogueDataErr {
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    Scene(&'static str, SceneErr),
    GetMut(&'static str),
}

impl fmt::Display for RogueDataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RogueDataErr::Renderable(_, ref err) => err.fmt(f),
            RogueDataErr::Transform(_, ref err) => err.fmt(f),
            RogueDataErr::Scene(_, ref err) => err.fmt(f),
            RogueDataErr::GetMut(_) => write!(f, "Get Mut was None"),
        }
    }
}

impl Error for RogueDataErr {
    fn description(&self) -> &str {
        match *self {
            RogueDataErr::Renderable(_, ref err) => err.description(),
            RogueDataErr::Transform(_, ref err) => err.description(),
            RogueDataErr::Scene(_, ref err) => err.description(),
            RogueDataErr::GetMut(_) => "Get Mut was None",
        }
    }
}