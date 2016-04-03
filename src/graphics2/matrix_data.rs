use std::sync::{Arc};
use std::collections::{HashMap};
use std::fmt;
use std::error::Error;

use logic::{Id};
use math::{Mat4, Vec2, Vec3, Vec4};
use components::{Renderable};

pub struct MatrixData {
    mat4s: Arc<HashMap<Id, Mat4>>,
    inverse: Arc<HashMap<Id, Mat4>>,
}

impl MatrixData {
    pub fn new() -> MatrixData {
        MatrixData {
            mat4s: Arc::new(HashMap::new()),
            inverse: Arc::new(HashMap::new()),
        }
    }

    pub fn set_matrix(&mut self, id: Id, mat4: Mat4, inverse: Mat4) {
        self.mat4s.insert(id, mat4);
        self.mat4s.insert(id, inverse);
    }

    pub fn get_matrix(&self, id: Id) -> Option<&Mat4> {
        self.mat4s.get(&id)
    }

    pub fn get_inverse(&self, id: Id) -> Option<&Mat4> {
        self.inverse.get(&id)
    }

}
