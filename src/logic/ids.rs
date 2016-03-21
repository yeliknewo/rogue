use std::collections::{HashMap};
use std::fmt::{Display, Formatter, Error};
use std::sync::{Arc, RwLock};

#[derive(Copy, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Id {
    id: IdSize,
}

impl Id {
    pub fn new(manager: Arc<RwLock<IdManager>>, id_type: IdType) -> Id {
        Id {
            id: manager.write().expect("Unable to Write Manager in New in Id").get_id(id_type),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.id)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum IdType {
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

pub struct IdManager {
    map: HashMap<IdType, IdSize>,
}

impl IdManager {
    pub fn new() -> IdManager {
        IdManager {
            map: HashMap::new(),
        }
    }

    fn get_id(&mut self, id_type: IdType) -> IdSize {
        let id = match self.map.get(&id_type) {
            Some(id) => *id,
            None => 0,
        };
        self.map.insert(id_type, id + 1);
        id
    }
}

pub type IdSize = u64;
