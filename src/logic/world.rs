use std::sync::{Arc};
use std::collections::{HashMap};
use std::fmt;
use std::error::Error;

use input::{Keyboard, Mouse, Display, KeyCode, MouseButton, Button};
use logic::{Id, EntityData};
use math::{Vec2};

pub struct World<T: EntityData<T>> {
    keyboard: Arc<Keyboard>,
    mouse: Arc<Mouse>,
    display: Arc<Display>,
    entity_data: Arc<HashMap<Id, Arc<T>>>,
    names: Arc<HashMap<&'static str, Id>>,
}

impl<T: EntityData<T>> World<T> {
    pub fn new(keyboard: Arc<Keyboard>, mouse: Arc<Mouse>, display: Arc<Display>) -> World<T> {
        World {
            keyboard: keyboard,
            mouse: mouse,
            display: display,
            entity_data: Arc::new(HashMap::new()),
            names: Arc::new(HashMap::new()),
        }
    }

    pub fn set_key(&mut self, key_code: KeyCode, key: Button) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.keyboard){
            Some(keyboard) => {
                keyboard.set_key_state(key_code, key);
                Ok(())
            },
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn set_mouse_button(&mut self, mouse_button: MouseButton, button: Button) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.mouse) {
            Some(mouse) => {
                mouse.set_mouse_button(mouse_button, button);
                Ok(())
            },
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn set_mouse_position(&mut self, pos: Vec2) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.mouse) {
            Some(mouse) => {
                mouse.set_mouse_position(pos);
                Ok(())
            },
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn set_resolution(&mut self, resolution:  Vec2) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.display) {
            Some(display) => {
                display.set_resolution(resolution);
                Ok(())
            },
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn get_key(&self, key_code: KeyCode) -> Button {
        self.keyboard.get_key(key_code)
    }

    pub fn get_mouse_button(&self, mouse_button: MouseButton) -> Button {
        self.mouse.get_button(mouse_button)
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse.get_mouse_position()
    }

    pub fn get_resolution(&self) -> Vec2 {
        self.display.get_resolution()
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.display.get_aspect_ratio()
    }

    pub fn get_entity_data(&self) -> Arc<HashMap<Id, Arc<T>>> {
        self.entity_data.clone()
    }

    pub fn get_mut_entity_data(&mut self) -> Result<&mut HashMap<Id, Arc<T>>, WorldErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => Ok(entity_data),
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn add_entity(&mut self, entity: T) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => {
                entity_data.insert(entity.get_id(), Arc::new(entity));
                Ok(())
            },
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn get_entity_by_id(&self, id: Id) -> Option<Arc<T>> {
        match self.entity_data.get(&id) {
            Some(entity_data) => {
                Some(entity_data.clone())
            }
            None => None,
        }
    }

    pub fn get_entity_by_name(&self, name: &'static str) -> Option<Arc<T>> {
        match self.names.get(name) {
            Some(id) => {
                match self.entity_data.get(id) {
                    Some(entity) => {
                        Some(entity.clone())
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn register_name(&mut self, id: Id, name: &'static str) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.names) {
            Some(names) => {
                if names.contains_key(name) {
                    Err(WorldErr::InvalidName)
                } else {
                    names.insert(name, id);
                    Ok(())
                }
            },
            None => Err(WorldErr::GetMut),
        }
    }

    pub fn deregister_name(&mut self, name: &'static str) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.names) {
            Some(names) => {
                names.remove(name);
                Ok(())
            },
            None => Err(WorldErr::GetMut)
        }
    }
}

#[derive(Debug)]
pub enum WorldErr {
    GetMut,
    InvalidName,
}

impl fmt::Display for WorldErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldErr::GetMut => write!(f, "Get Mut was None"),
            WorldErr::InvalidName => write!(f, "Name was already taken"),
        }
    }
}

impl Error for WorldErr {
    fn description(&self) -> &str {
        match *self {
            WorldErr::GetMut => "Get Mut was None",
            WorldErr::InvalidName => "Invalid Name",
        }
    }
}
