use std::sync::{Arc};
use std::error::Error;
use std::fmt;

use dorp::{
    EntityData, World, IdManager, Window, SyncData, Renderers, Id, Renderable, Named, Transform,
    RenderableErr, TransformErr, Map3d, Scene, OptErr, TickCount
};

use components::{Block, BlockMap, BlockCoords};

pub struct RogueData {
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
    scene: Option<Arc<Scene<RogueData>>>,
    block_map: Option<Arc<BlockMap>>,
    block_coords: Option<Arc<BlockCoords>>,
    block: Option<Arc<Block>>,
    id: Id,
}

impl RogueData {
    pub fn new(id: Id) -> RogueData {
        RogueData {
            renderable: None,
            named: None,
            transform: None,
            block_map: None,
            scene: None,
            block: None,
            block_coords: None,
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


    pub fn with_block_map(mut self, block_map: BlockMap) -> RogueData {
        self.block_map = Some(Arc::new(block_map));
        self
    }


    pub fn with_scene(mut self, scene: Scene<RogueData>) -> RogueData {
        self.scene = Some(Arc::new(scene));
        self
    }


    pub fn with_block(mut self, block: Block) -> RogueData {
        self.block = Some(Arc::new(block));
        self
    }


    pub fn with_block_coords(mut self, block_coords: BlockCoords) -> RogueData {
        self.block_coords = Some(Arc::new(block_coords));
        self
    }


    pub fn get_block_map(&self) -> Option<Arc<BlockMap>> {
        self.block_map.clone()
    }


    pub fn get_block(&self) -> Option<Arc<Block>> {
        self.block.clone()
    }


    pub fn get_block_coords(&self) -> Option<Arc<BlockCoords>> {
        self.block_coords.clone()
    }

    pub fn get_mut_block_map(&mut self) -> OptErr<&mut BlockMap, RogueDataErr> {
        match self.block_map.as_mut() {
            Some(map_3d) => {
                match Arc::get_mut(map_3d) {
                    Some(map_3d) => return OptErr::Full(map_3d),
                    None => return OptErr::Error(RogueDataErr::GetMut("Arc Get Mut Map 3d")),
                }
            },
            None => return OptErr::Empty,
        }
    }

    pub fn get_mut_block(&mut self) -> OptErr<&mut Block, RogueDataErr> {
        match self.block.as_mut() {
            Some(block) => {
                match Arc::get_mut(block) {
                    Some(block) => return OptErr::Full(block),
                    None => return OptErr::Error(RogueDataErr::GetMut("Arc Get Mut Block")),
                }
            },
            None => return OptErr::Empty,
        }
    }
}

impl EntityData<RogueData> for RogueData {
    fn tick(&self, tick_count: Arc<TickCount>, delta_time: Arc<f64>, world: Arc<World<RogueData>>) -> Result<(), Box<Error>> {
        Ok(())
    }

    fn tick_mut(&mut self, tick_count: TickCount, manager: &mut IdManager, world: &mut World<RogueData>) -> Result<(), Box<Error>> {
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
    Scene(&'static str, Box<Error>),
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
