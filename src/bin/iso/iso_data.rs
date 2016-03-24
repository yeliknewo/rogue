use std::sync::{Arc};
use std::fmt;
use std::error::Error;

use dorp::{
    EntityData, World, IdManager, Id, Window, Renderable, Named, Transform, MatrixData,
    RenderableErr, TransformErr
};

use iso::{Tile, TileErr, TileCoords, TileMap};

pub struct IsoData {
    id: Id,
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
    tile: Option<Arc<Tile>>,
    tile_coords: Option<Arc<TileCoords>>,
    tile_map: Option<Arc<TileMap>>,
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) -> Result<(), Box<Error>> {
        match self.tile.clone() {
            Some(tile) => {
                match self.tile_coords.clone() {
                    Some(tile_coords) => {
                        match tile.tick(tile_coords, world) {
                            Ok(()) => (),
                            Err(err) => return Err(Box::new(IsoDataErr::Tile("Tile Tick", err))),
                        }
                    },
                    None => return Err(Box::new(IsoDataErr::BadComponentSetup("Tile Requires Tile Coords"))),
                }
            },
            None => (),
        }
        Ok(())
    }

    fn tick_mut(&mut self, manager: &mut IdManager) -> Result<(), Box<Error>> {
        if self.transform.is_some() && self.renderable.is_some() {
            match
                match Arc::get_mut(
                    match self.transform.as_mut() {
                        Some(transform) => transform,
                        None => return Err(Box::new(IsoDataErr::Get("Self Transform As Mut"))),
                    }
                ) {
                    Some(transform) => transform,
                    None => return Err(Box::new(IsoDataErr::GetMut("Arc Transform Get Mut"))),
                }.tick_mut(
                    match Arc::get_mut(match self.renderable.as_mut() {
                        Some(renderable) => renderable,
                        None => return Err(Box::new(IsoDataErr::Get("Self Renderable As Mut"))),
                    }
                ) {
                    Some(renderable) => renderable,
                    None => return Err(Box::new(IsoDataErr::GetMut("Arc Renderable Get Mut"))),
                })
            {
                Ok(()) => (),
                Err(err) => return Err(Box::new(IsoDataErr::Transform("Self Transform Tick Mut", err))),
            }
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), Box<Error>> {
        match self.renderable.as_mut() {
            Some(renderable) => {
                match Arc::get_mut(renderable) {
                    Some(renderable) => match renderable.render(window, matrix_data) {
                        Ok(()) => (),
                        Err(err) => return Err(Box::new(IsoDataErr::Renderable("Renderable Render", err))),
                    },
                    None => return Err(Box::new(IsoDataErr::GetMut("Arc Get Mut Renderable"))),
                }
            },
            None => (),
        }
        // if self.renderable.is_some() {
        //     match match Arc::get_mut(match self.renderable.as_mut() {
        //         Some(renderable) => renderable,
        //         None => return Err(Box::new(IsoDataErr::Get("Self Renderable As Mut"))),
        //     }) {
        //         Some(renderable) => renderable,
        //         None => return Err(Box::new(IsoDataErr::GetMut("Arc Renderable Get Mut"))),
        //     }.render(window, matrix_data) {
        //         Ok(()) => (),
        //         Err(err) => return Err(Box::new(IsoDataErr::Renderable("Renderable Render", err))),
        //     };
        // }
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
            tile: None,
            tile_coords: None,
            tile_map: None,
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

    pub fn with_tile(mut self, tile: Tile) -> IsoData {
        self.tile = Some(Arc::new(tile));
        self
    }

    pub fn with_tile_coords(mut self, tile_coords: TileCoords) -> IsoData {
        self.tile_coords = Some(Arc::new(tile_coords));
        self
    }

    pub fn with_tile_map(mut self, tile_map: TileMap) -> IsoData {
        self.tile_map = Some(Arc::new(tile_map));
        self
    }

    pub fn get_tile(&self) -> Option<Arc<Tile>> {
        self.tile.clone()
    }

    pub fn get_tile_map(&self) -> Option<Arc<TileMap>> {
        self.tile_map.clone()
    }

    pub fn get_tile_coords(&self) -> Option<Arc<TileCoords>> {
        self.tile_coords.clone()
    }
}

#[derive(Debug)]
pub enum IsoDataErr {
    BadComponentSetup(&'static str),
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    Tile(&'static str, TileErr),
    GetMut(&'static str),
    Get(&'static str),
}

impl fmt::Display for IsoDataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IsoDataErr::Renderable(_, ref err) => err.fmt(f),
            IsoDataErr::GetMut(_) => write!(f, "Get Mut was None"),
            IsoDataErr::Get(_) => write!(f, "Get was None"),
            IsoDataErr::BadComponentSetup(_) => write!(f, "This Component Requires Certain Other Components"),
            IsoDataErr::Transform(_, ref err) => err.fmt(f),
            IsoDataErr::Tile(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for IsoDataErr {
    fn description(&self) -> &str {
        match *self {
            IsoDataErr::BadComponentSetup(_) => "Component has unmet component requirements",
            IsoDataErr::Renderable(_, ref err) => err.description(),
            IsoDataErr::GetMut(_) => "Get Mut was None",
            IsoDataErr::Get(_) => "Get was None",
            IsoDataErr::Transform(_, ref err) => err.description(),
            IsoDataErr::Tile(_, ref err) => err.description(),
        }
    }
}
