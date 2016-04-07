use std::collections::{HashMap};

use math::{Vec2};
use input::{Button, MouseButton, ButtonState};

pub struct Mouse {
    buttons: HashMap<MouseButton, Button>,
    position: Vec2,
}

impl Mouse {
    #[inline]
    pub fn new() -> Mouse {
        Mouse {
            buttons: HashMap::new(),
            position: Vec2::zero(),
        }
    }

    #[inline]
    pub fn set_mouse_button(&mut self, button: MouseButton, state: Button) {
        self.buttons.insert(button, state);
    }

    #[inline]
    pub fn get_button(&self, mouse_button: MouseButton) -> Button {
        match self.buttons.get(&mouse_button) {
            Some(button) => *button,
            None => Button::new(0, ButtonState::Released),
        }
    }

    #[inline]
    pub fn set_mouse_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    #[inline]
    pub fn get_mouse_position(&self) -> Vec2 {
        self.position
    }
}
