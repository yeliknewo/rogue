use std::sync::{Arc, RwLock};

use math::{Mat4, Vec3};
use components::{Renderable};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scalation: Vec3,
    new_position: Option<Vec3>,
    new_rotation: Option<Vec3>,
    new_scalation: Option<Vec3>,
    dirty: bool,
    dirty_matrix: bool,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scalation: Vec3::one(),
            new_position: None,
            new_rotation: None,
            new_scalation: None,
            dirty: false,
            dirty_matrix: true,
        }
    }

    pub fn tick_mut(&mut self) {
        if self.dirty {
            match self.new_position.clone() {
                Some(pos) => {
                    self.set_position(pos);
                    self.new_position = None;
                },
                None => (),
            }
            match self.new_rotation.clone() {
                Some(rot) => {
                    self.set_rotation(rot);
                    self.new_rotation = None;
                },
                None => (),
            }
            match self.new_scalation.clone() {
                Some(sca) => {
                    self.set_scalation(sca);
                    self.new_scalation = None;
                },
                None => (),
            }
            self.dirty = false;
        }
    }

    pub fn render_sync(&mut self, renderable: Arc<RwLock<Renderable>>) {
        if self.dirty_matrix {
            let mut renderable = renderable.write().expect("Unable to Write Renderable in Tick Mut in Transform");
            renderable.set_model(Mat4::scalation_from_vec3(self.scalation) * Mat4::rotation_from_vec3(self.rotation) * Mat4::translation_from_vec3(self.position));
            self.dirty_matrix = false;
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

    pub fn prep_set_position(&mut self, pos: Vec3) {
        self.new_position = Some(pos);
        self.dirty = true;
    }

    pub fn prep_set_rotation(&mut self, rot: Vec3) {
        self.new_rotation = Some(rot);
        self.dirty = true;
    }

    pub fn prep_set_scalation(&mut self, sca: Vec3) {
        self.new_scalation = Some(sca);
        self.dirty = true;
    }

    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        self.dirty_matrix = true;
    }

    fn set_rotation(&mut self, rot: Vec3) {
        self.rotation = rot;
        self.dirty_matrix = true;
    }

    fn set_scalation(&mut self, sca: Vec3) {
        self.scalation = sca;
        self.dirty_matrix = true;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    pub fn get_scalation(&self) -> Vec3 {
        self.scalation
    }
}
