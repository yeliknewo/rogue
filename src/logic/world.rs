use std::sync::{Arc, RwLock};
use std::collections::{HashMap};

use input::{Keyboard, Mouse, Display, KeyCode, MouseButton, Button};
use logic::{Id, EntityData};
use math::{Vec2};
use graphics::{Transforms};

pub struct World<T: EntityData<T>> {
    keyboard: Arc<RwLock<Keyboard>>,
    mouse: Arc<RwLock<Mouse>>,
    display: Arc<RwLock<Display>>,
    transforms: Arc<RwLock<Transforms>>,
    entity_data: Arc<RwLock<HashMap<Id, Arc<T>>>>,
    names: Arc<RwLock<HashMap<&'static str, Id>>>,
}

impl<T: EntityData<T>> World<T> {
    pub fn new(keyboard: Arc<RwLock<Keyboard>>, mouse: Arc<RwLock<Mouse>>, display: Arc<RwLock<Display>>, transforms: Arc<RwLock<Transforms>>) -> World<T> {
        World {
            keyboard: keyboard,
            mouse: mouse,
            display: display,
            transforms: transforms,
            entity_data: Arc::new(RwLock::new(HashMap::new())),
            names: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_key(&self, key_code: KeyCode) -> Button {
        self.keyboard.read().expect("Unable to Read Keyboard in Get Key in World").get_key(key_code)
    }

    pub fn get_mouse_button(&self, mouse_button: MouseButton) -> Button {
        self.mouse.read().expect("Unable to Read Mouse in Get Mouse Button in World").get_button(mouse_button)
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse.read().expect("Unable to Read Mouse in Get Mouse Position in World").get_mouse_position()
    }

    pub fn get_resolution(&self) -> Vec2 {
        self.display.read().expect("Unable to Read Display in Get Resolution in World").get_resolution()
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.display.read().expect("Unable to Read Display in Get Aspect Ratio in World").get_aspect_ratio()
    }

    pub fn get_entity_data(&self) -> Arc<RwLock<HashMap<Id, Arc<T>>>> {
        self.entity_data.clone()
    }

    pub fn add_entity(&self, entity: T) {
        self.entity_data.write().expect("Unable to Write Entity Data in Add Entity in World").insert(entity.get_id(), Arc::new(entity));
    }

    pub fn get_entity_by_id(&self, id: Id) -> Option<Arc<T>> {
        match self.entity_data.read().expect("Unable to Read Entity Data in Get Entity By Id in World").get(&id) {
            Some(entity_data) => {
                Some(entity_data.clone())
            }
            None => None,
        }
    }

    pub fn get_entity_by_name(&self, name: &'static str) -> Option<Arc<T>> {
        let names = self.names.read().expect("Unable to Read Names in Get Entity By Name in World");
        match names.get(name) {
            Some(id) => {
                let entities = self.entity_data.read().expect("Unable to Read Entity Data in Get Entity By Name in World");
                match entities.get(id) {
                    Some(entity) => {
                        Some(entity.clone())
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_transforms(&self) -> Arc<RwLock<Transforms>> {
        self.transforms.clone()
    }

    pub fn register_name(&self, id: Id, name: &'static str) -> Result<(), &'static str>{
        let mut names = self.names.write().expect("Unable to Write Names in Register Name in World");
        if names.contains_key(name) {
            return Err("Name Already in Use");
        }
        names.insert(name, id);
        Ok(())
    }

    pub fn deregister_name(&self, name: &'static str) {
        let mut names = self.names.write().expect("Unable to Write Names in Deregister Name in World");
        names.remove(name);
    }
}
