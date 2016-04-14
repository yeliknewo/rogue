use std::error::Error;
use std::fmt;

use dorp::{World, WorldErr, IdManager, Id, IdType};

use components::{Block, BlockErr, BlockType, BlockCoords};
use core::{RogueData, RogueDataErr, BLOCK_MAP_NAME};
use entities::{new_block_map_entity, BlockMapEntityErr};

pub fn new_block_entity(block_type: BlockType, block_coords: BlockCoords, manager: &mut IdManager, world: &mut World<RogueData>) -> Result<Id, BlockEntityErr> {
    let id = Id::new(manager, IdType::Entity);

    let block_map_entity = match world.get_entity_by_name(BLOCK_MAP_NAME) {
        Some(entity) => entity,
        None => match new_block_map_entity(manager, world) {
            Ok(id) => match world.get_entity_by_id(id) {
                Some(block_map) => block_map,
                None => return Err(BlockEntityErr::Get("World Get Entity by id")),
            },
            Err(err) => return Err(BlockEntityErr::BlockMapEntityErr("new block map entity", err)),
        },
    };

    Ok(id)
}

#[derive(Debug)]
pub enum BlockEntityErr {
    BlockMapEntityErr(&'static str, BlockMapEntityErr),
    World(&'static str, WorldErr),
    RogueData(&'static str, RogueDataErr),
    Block(&'static str, BlockErr),
    Get(&'static str),
    GetMut(&'static str),
}

impl fmt::Display for BlockEntityErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockEntityErr::BlockMapEntityErr(_, ref err) => err.fmt(f),
            BlockEntityErr::World(_, ref err) => err.fmt(f),
            BlockEntityErr::RogueData(_, ref err) => err.fmt(f),
            BlockEntityErr::Block(_, ref err) => err.fmt(f),
            BlockEntityErr::Get(_) => write!(f, "Get was None"),
            BlockEntityErr::GetMut(_) => write!(f, "Get Mut was None"),
        }
    }
}

impl Error for BlockEntityErr {
    fn description(&self) -> &str {
        match *self {
            BlockEntityErr::BlockMapEntityErr(_, ref err) => err.description(),
            BlockEntityErr::World(_, ref err) => err.description(),
            BlockEntityErr::RogueData(_, ref err) => err.description(),
            BlockEntityErr::Block(_, ref err) => err.description(),
            BlockEntityErr::Get(_) => "Get was None",
            BlockEntityErr::GetMut(_) => "Get Mut was None",
        }
    }
}
