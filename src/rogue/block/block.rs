use std::fmt;
use std::error::Error;
use std::sync::{Arc};

use dorp::{
    Id, World, WorldErr, OptErr, EntityData, TickCount
};

use rogue::{RogueData, RogueDataErr, BlockCoords, RogueWorld, BlockMap};

pub enum BlockType {
    Air,
    Dirt,
}

enum NeighborState {
    New,
    Dirty,
    Clean,
}

pub struct Block {
    block_type: BlockType,
    block_map_id: Id,
    neighbors: Vec<Id>,
    neighbor_state: NeighborState,
}

impl Block {
    pub fn new(block_type: BlockType, id: Id, block_coords: &BlockCoords, world: &mut World<RogueData>, block_map_name: &'static str) -> Result<Block, BlockErr> {
        let block_map_id = match world.get_mut_entity_by_name(block_map_name) {
            OptErr::Full(block_map) => {
                match block_map.get_mut_block_map() {
                    OptErr::Full(block_map) => match block_coords.register(id, block_map) {
                        Ok(()) => (),
                        Err(err) => panic!(),
                    },
                    OptErr::Empty => return Err(BlockErr::Get("Block Map Get Mut Map 3d")),
                    OptErr::Error(err) => return Err(BlockErr::RogueData("Block map get mut map 3d", err)),
                }
                block_map.get_id()
            },
            OptErr::Empty => return Err(BlockErr::Get("World get mut Entity by Name Block Map name")),
            OptErr::Error(err) => return Err(BlockErr::World("World Get Mut Entity By Name", err)),
        };
        Ok(
            Block {
                block_type: block_type,
                block_map_id: block_map_id,
                neighbors: vec!(),
                neighbor_state: NeighborState::New,
            }
        )
    }

    pub fn new_with_block_map(block_type: BlockType, id: Id, block_coords: &BlockCoords, block_map: &mut BlockMap, block_map_id: Id) -> Block {
        block_coords.register(id, block_map);
        Block{
            block_type: block_type,
            block_map_id: block_map_id,
            neighbors: vec!(),
            neighbor_state: NeighborState::New,
        }
    }

    pub fn tick_mut(&mut self, my_id: Id, world: &mut RogueWorld) -> Result<(), BlockErr> {
        match self.neighbor_state {
            NeighborState::Clean => Ok(()),
            NeighborState::Dirty => match self.update_neighbors(my_id, false, world) {
                Ok(()) => Ok(()),
                Err(err) => return Err(BlockErr::Block("self Update Neighbors my id false world", Box::new(err))),
            },
            NeighborState::New => match self.update_neighbors(my_id, true, world) {
                Ok(()) => Ok(()),
                Err(err) => return Err(BlockErr::Block("Self Update Neighbors My Id True World", Box::new(err))),
            },
        }
    }

    fn update_neighbors(&mut self, my_id: Id, trigger_more: bool, world: &mut RogueWorld) -> Result<(), BlockErr> {
        match world.get_entity_by_id(my_id) {
            Some(my_entity) => {
                match my_entity.get_block_coords() {
                    Some(block_coords) => {
                        match world.get_entity_by_id(self.block_map_id) {
                            Some(block_map_entity) => {
                                match block_map_entity.get_block_map() {
                                    Some(block_map) => {
                                        for z in -1..2 {
                                            for y in -1..2 {
                                                for x in -1..2 {
                                                    match block_map.get(block_coords.get_x() + x, block_coords.get_y() + y, block_coords.get_z() + z) {
                                                        Some(block_id) => {
                                                            if trigger_more {
                                                                match world.take_entity_by_id(block_id) {
                                                                    OptErr::Full(mut block_entity_arc) => {
                                                                        match Arc::get_mut(&mut block_entity_arc) {
                                                                            Some(block_entity) => {
                                                                                match block_entity.get_mut_block() {
                                                                                    OptErr::Full(block) => {
                                                                                        match block.update_neighbors(block_id, false, world) {
                                                                                            Ok(()) => (),
                                                                                            Err(err) => return Err(BlockErr::Block("Block Update Neighbors Block Id False World", Box::new(err))),
                                                                                        }
                                                                                    },
                                                                                    OptErr::Empty => return Err(BlockErr::Get("Block Entity Get Mut Block")),
                                                                                    OptErr::Error(err) => return Err(BlockErr::RogueData("Block Entity Get Mut Block", err)),
                                                                                }
                                                                            },
                                                                            None => return Err(BlockErr::GetMut("Arc Get Mut Block Entity Arc")),
                                                                        }
                                                                        world.add_entity_arc(block_entity_arc);
                                                                    },
                                                                    OptErr::Empty => return Err(BlockErr::Get("World Get Mut Entity By Id Block Id")),
                                                                    OptErr::Error(err) => return Err(BlockErr::World("World get Mut Entity by Id Block Id", err)),
                                                                }
                                                            }
                                                            self.neighbors.push(block_id);
                                                        },
                                                        None => return Err(BlockErr::Get("Block Map Get Tile Block Coords + Offset")),
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    None => return Err(BlockErr::Get("Block Map Entitty Get Block Map")),
                                }
                            },
                            None => return Err(BlockErr::Get("World Get Entity by Id self block map id")),
                        }
                    },
                    None => return Err(BlockErr::Get("Entity Get Block Coords")),
                }
            },
            None => return Err(BlockErr::Get("World Get Entity by id My ID")),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum BlockErr {
    World(&'static str, WorldErr),
    RogueData(&'static str, RogueDataErr),
    Block(&'static str, Box<BlockErr>),
    Get(&'static str),
    GetMut(&'static str),
}

impl fmt::Display for BlockErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockErr::World(_, ref err) => err.fmt(f),
            BlockErr::RogueData(_, ref err) => err.fmt(f),
            BlockErr::Block(_, ref err) => err.fmt(f),
            BlockErr::Get(_) => write!(f, "Get was None"),
            BlockErr::GetMut(_) => write!(f, "Get Mut was None"),
        }
    }
}

impl Error for BlockErr {
    fn description(&self) -> &str {
        match *self {
            BlockErr::World(_, ref err) => err.description(),
            BlockErr::RogueData(_, ref err) => err.description(),
            BlockErr::Block(_, ref err) => err.description(),
            BlockErr::Get(_) => "Get was None",
            BlockErr::GetMut(_) => "Get Mut was None",
        }
    }
}
