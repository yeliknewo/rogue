use std::sync::{Arc};
use std::fmt;
use std::error::Error;

use dorp::{
    EntityData, World, IdManager, Id, Window, Renderable, Transform, Named, MatrixData,
    RenderableErr, TransformErr
};

use life::{Scene, SceneErr, Cell, CellErr, TileMap};

pub struct LifeData {
    id: Id,
    renderable: Option<Arc<Renderable>>,
    transform: Option<Arc<Transform>>,
    named: Option<Arc<Named>>,
    scene: Option<Arc<Scene>>,
    cell: Option<Arc<Cell>>,
    tile_map: Option<Arc<TileMap>>,
}

impl EntityData<LifeData> for LifeData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<LifeData>>) -> Result<(), Box<Error>> {
        match self.cell {
            Some(ref cell) => match cell.tick(world) {
                Ok(()) => (),
                Err(err) => return Err(Box::new(LifeDataErr::Cell("Cell Tick", err))),
            },
            None => (),
        }
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
        match self.tile_map.as_mut() {
            Some(tile_map) => {
                match Arc::get_mut(tile_map) {
                    Some(tile_map) => tile_map.tick_mut(),
                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Tile Map"))),
                }
            },
            None => (),
        }
        match self.cell.as_mut() {
            Some(cell) => {
                match Arc::get_mut(cell) {
                    Some(cell) => {
                        match self.renderable.as_mut() {
                            Some(renderable) => {
                                match Arc::get_mut(renderable) {
                                    Some(renderable) => {
                                        match cell.tick_mut(renderable) {
                                            Ok(()) => (),
                                            Err(err) => return Err(Box::new(LifeDataErr::Cell("Cell Tick Mut", err))),
                                        }
                                    },
                                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Renderable"))),
                                }
                            },
                            None => return Err(Box::new(LifeDataErr::BadComponentSetup("Cell requires Renderable"))),
                        }
                    },
                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Cell"))),
                }
            },
            None => (),
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), Box<Error>> {
        match self.renderable.as_mut() {
            Some(renderable) => {
                match Arc::get_mut(renderable) {
                    Some(renderable) => {
                        match self.transform.as_mut() {
                            Some(transform) => {
                                match Arc::get_mut(transform) {
                                    Some(transform) => transform.render(renderable),
                                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Transform"))),
                                }
                            },
                            None => (),
                        }
                        match renderable.render(window, matrix_data) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(LifeDataErr::Renderable("Renderable Render", err))),
                        }
                    }
                    None => return Err(Box::new(LifeDataErr::GetMut("Arc Get Mut Renderable"))),
                }
            },
            None => (),
        }
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
            cell: None,
            tile_map: None,
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

    pub fn with_cell(mut self, cell: Cell) -> LifeData {
        self.cell = Some(Arc::new(cell));
        self
    }

    pub fn with_tile_map(mut self, tile_map: TileMap) -> LifeData {
        self.tile_map = Some(Arc::new(tile_map));
        self
    }

    pub fn get_cell(&self) -> Option<Arc<Cell>> {
        self.cell.clone()
    }

    pub fn get_tile_map(&self) -> Option<Arc<TileMap>> {
        self.tile_map.clone()
    }
}

#[derive(Debug)]
pub enum LifeDataErr {
    BadComponentSetup(&'static str),
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    Scene(&'static str, SceneErr),
    Cell(&'static str, CellErr),
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
            LifeDataErr::Cell(_, ref err) => err.fmt(f),
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
            LifeDataErr::Cell(_, ref err) => err.description(),
        }
    }
}
