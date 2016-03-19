use std::sync::{Arc, RwLock};

use math::{Mat4, Vec3};
use components::{Renderable};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scalation: Vec3,
    dirty: bool,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scalation: Vec3::one(),
            dirty: true,
        }
    }

    pub fn tick_mut(&mut self, renderable: Box<Arc<RwLock<Renderable>>>) {
        if self.dirty {
            let mut renderable = renderable.write().expect("Unable to Write Renderable in Tick Mut in Transform");
            renderable.set_model(Mat4::scalation_from_vec3(self.scalation) * Mat4::rotation_from_vec3(self.rotation) * Mat4::translation_from_vec3(self.position));
            self.dirty = false;
        }
    }

    pub fn with_position(mut self, pos: Vec3) -> Transform {
        self.set_position(pos);
        self
    }

    pub fn with_rotation(mut self, rot: Vec3) -> Transform {
        self.set_rotation(rot);
        self
    }

    pub fn with_scalation(mut self, sca: Vec3) -> Transform {
        self.set_scalation(sca);
        self
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        self.dirty = true;
    }

    pub fn set_rotation(&mut self, rot: Vec3) {
        self.rotation = rot;
        self.dirty = true;
    }

    pub fn set_scalation(&mut self, sca: Vec3) {
        self.scalation = sca;
        self.dirty = true;
    }
}
