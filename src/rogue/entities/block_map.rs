use std::error::Error;
use std::fmt;

use dorp::{World, WorldErr, IdManager, Id, IdType, Named, NamedErr};

use rogue::{RogueData, RogueDataErr, BlockErr, BlockMap, BLOCK_MAP_NAME};

pub fn new_block_map_entity(manager: &mut IdManager, world: &mut World<RogueData>) -> Result<Id, BlockMapEntityErr> {
    let id = Id::new(manager, IdType::Entity);

    let block_map = BlockMap::new();
    let named = match Named::new(BLOCK_MAP_NAME, id, world) {
        Ok(named) => named,
        Err(err) => return Err(BlockMapEntityErr::Named("Named New", err)),
    };

    match world.add_entity(RogueData::new(id)
        .with_block_map(
            block_map
        )
        .with_named(
            named
        )
    ) {
        Ok(()) => (),
        Err(err) => return Err(BlockMapEntityErr::World("World Add Entity", err)),
    };

    Ok(id)
}

#[derive(Debug)]
pub enum BlockMapEntityErr {
    Named(&'static str, NamedErr),
    World(&'static str, WorldErr),
    RogueData(&'static str, RogueDataErr),
    Block(&'static str, BlockErr),
    Get(&'static str),
    GetMut(&'static str),
}

impl fmt::Display for BlockMapEntityErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockMapEntityErr::Named(_, ref err) => err.fmt(f),
            BlockMapEntityErr::World(_, ref err) => err.fmt(f),
            BlockMapEntityErr::RogueData(_, ref err) => err.fmt(f),
            BlockMapEntityErr::Block(_, ref err) => err.fmt(f),
            BlockMapEntityErr::Get(_) => write!(f, "Get was None"),
            BlockMapEntityErr::GetMut(_) => write!(f, "Get Mut was None"),
        }
    }
}

impl Error for BlockMapEntityErr {
    fn description(&self) -> &str {
        match *self {
            BlockMapEntityErr::Named(_, ref err) => err.description(),
            BlockMapEntityErr::World(_, ref err) => err.description(),
            BlockMapEntityErr::RogueData(_, ref err) => err.description(),
            BlockMapEntityErr::Block(_, ref err) => err.description(),
            BlockMapEntityErr::Get(_) => "Get was None",
            BlockMapEntityErr::GetMut(_) => "Get Mut was None",
        }
    }
}
