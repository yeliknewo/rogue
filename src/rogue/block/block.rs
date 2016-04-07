use std::fmt;
use std::error::Error;

use dorp::{
    Id, World, WorldErr, OptErr
};

use rogue::{RogueData, RogueDataErr, BlockCoords};

pub enum BlockType {
    Air,
    Dirt,
    Water,
}

pub struct Block {
    block_type: BlockType,
}

impl Block {
    pub fn new(block_type: BlockType, id: Id, block_coords: BlockCoords, world: &mut World<RogueData>, block_map_name: &'static str) -> Result<Block, BlockErr> {
        match world.get_mut_entity_by_name(block_map_name) {
            OptErr::Full(block_map) => {
                match block_map.get_mut_block_map() {
                    OptErr::Full(block_map) => block_coords.register(id, block_map),
                    OptErr::Empty => return Err(BlockErr::Get("Block Map Get Mut Map 3d")),
                    OptErr::Error(err) => return Err(BlockErr::RogueData("Block map get mut map 3d", err)),
                }
            },
            OptErr::Empty => return Err(BlockErr::Get("World get mut Entity by Name Block Map name")),
            OptErr::Error(err) => return Err(BlockErr::World("World Get Mut Entity By Name", err)),
        }
        Ok(
            Block {
                block_type: block_type,
            }
        )
    }

    pub fn tick(&self) {
        
    }
}

#[derive(Debug)]
pub enum BlockErr {
    World(&'static str, WorldErr),
    RogueData(&'static str, RogueDataErr),
    Get(&'static str),
}

impl fmt::Display for BlockErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockErr::World(_, ref err) => err.fmt(f),
            BlockErr::RogueData(_, ref err) => err.fmt(f),
            BlockErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for BlockErr {
    fn description(&self) -> &str {
        match *self {
            BlockErr::World(_, ref err) => err.description(),
            BlockErr::RogueData(_, ref err) => err.description(),
            BlockErr::Get(_) => "Get was None",
        }
    }
}
