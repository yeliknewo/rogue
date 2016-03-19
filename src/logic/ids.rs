use std::collections::{HashMap};
use std::fmt::{Display, Formatter, Error};
use std::sync::{Arc, RwLock};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct ID {
    id: IDSize,
}

impl ID {
    pub fn new(manager: Arc<RwLock<IDManager>>, id_type: IDType) -> ID {
        ID {
            id: manager.write().expect("Unable to Write Manager in New in ID").get_id(id_type),
        }
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.id)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum IDType {
    World,
    Entity,
    Component,
    Vertex,
    Index,
    Texture,
    DrawMethod,
    Perspective,
    View,
    Model,
}

pub struct IDManager {
    map: HashMap<IDType, IDSize>,
}

impl IDManager {
    pub fn new() -> IDManager {
        IDManager {
            map: HashMap::new(),
        }
    }

    fn get_id(&mut self, id_type: IDType) -> IDSize {
        let id = match self.map.get(&id_type) {
            Some(id) => *id,
            None => 0,
        };
        self.map.insert(id_type, id + 1);
        id
    }
}

pub type IDSize = u64;
