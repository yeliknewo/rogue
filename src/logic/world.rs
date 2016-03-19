use std::sync::{Arc, RwLock};
use std::collections::{HashMap};

use input::{Keyboard, Mouse, Display, KeyCode, MouseButton, Button};
use logic::{ID, EntityData};
use math::{Vec2};
use graphics::{Transforms};

pub struct World<T: EntityData<T>> {
    keyboard: Arc<RwLock<Keyboard>>,
    mouse: Arc<RwLock<Mouse>>,
    display: Arc<RwLock<Display>>,
    transforms: Arc<RwLock<Transforms>>,
    entity_data: Arc<RwLock<HashMap<ID, Arc<RwLock<T>>>>>,
}

impl<T: EntityData<T>> World<T> {
    pub fn new(keyboard: Arc<RwLock<Keyboard>>, mouse: Arc<RwLock<Mouse>>, display: Arc<RwLock<Display>>, transforms: Arc<RwLock<Transforms>>) -> World<T> {
        World {
            keyboard: keyboard,
            mouse: mouse,
            display: display,
            transforms: transforms,
            entity_data: Arc::new(RwLock::new(HashMap::new())),
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

    pub fn get_entity_data(&self) -> Arc<RwLock<HashMap<ID, Arc<RwLock<T>>>>> {
        self.entity_data.clone()
    }

    pub fn get_transforms(&self) -> Arc<RwLock<Transforms>> {
        self.transforms.clone()
    }
}
