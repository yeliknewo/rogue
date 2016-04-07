use std::collections::{HashMap};

use input::{Button, KeyCode, ButtonState};

pub struct Keyboard {
    keys: HashMap<KeyCode, Button>,
}

impl Keyboard {
    #[inline]
    pub fn new() -> Keyboard{
        Keyboard{
            keys: HashMap::new(),
        }
    }

    #[inline]
    pub fn get_key(&self, key_code: KeyCode) -> Button {
        match self.keys.get(&key_code) {
            Some(key) => *key,
            None => Button::new(0, ButtonState::Released),
        }
    }

    #[inline]
    pub fn set_key_state(&mut self, key_code: KeyCode, key: Button) {
        self.keys.insert(key_code, key);
    }
}
