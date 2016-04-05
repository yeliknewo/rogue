use std::fmt;
use std::error::Error;

use dorp::{
    Id, World, Map3d
};

use rogue::{RogueData, BlockCoords};

pub struct Block;

impl Block {
    pub fn new(id: Id, block_coords: BlockCoords, world: &mut World<RogueData>, block_map_name: &'static str) -> Result<Block, BlockErr> {
        Ok(
            Block
        )
    }
}

#[derive(Debug)]
pub enum BlockErr {
    // World(&'static str, WorldErr),
    // Named(&'static str, NamedErr),
    Get(&'static str),
}

impl fmt::Display for BlockErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // SceneErr::World(_, ref err) => err.fmt(f),
            // SceneErr::Named(_, ref err) => err.fmt(f),
            BlockErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for BlockErr {
    fn description(&self) -> &str {
        match *self {
            // SceneErr::World(_, ref err) => err.description(),
            // SceneErr::Named(_, ref err) => err.description(),
            BlockErr::Get(_) => "Get was None",
        }
    }
}
