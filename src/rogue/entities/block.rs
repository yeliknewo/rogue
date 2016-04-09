use std::error::Error;
use std::fmt;

use dorp::{WorldErr};

use rogue::{RogueData, RogueDataErr, BlockErr};

pub fn new_block_entity() -> Result<RogueData, BlockEntityErr> {

}

#[derive(Debug)]
pub enum BlockEntityErr {
    World(&'static str, WorldErr),
    RogueData(&'static str, RogueDataErr),
    Block(&'static str, Box<BlockErr>),
    Get(&'static str),
    GetMut(&'static str),
}

impl fmt::Display for BlockEntityErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
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
            BlockEntityErr::World(_, ref err) => err.description(),
            BlockEntityErr::RogueData(_, ref err) => err.description(),
            BlockEntityErr::Block(_, ref err) => err.description(),
            BlockEntityErr::Get(_) => "Get was None",
            BlockEntityErr::GetMut(_) => "Get Mut was None",
        }
    }
}
