use std::collections::{HashMap};

use logic::{Id};
use math::{Mat4};

pub struct MatrixData {
    mat4s: HashMap<Id, Mat4>,
    inverse: HashMap<Id, Mat4>,
}

impl MatrixData {
    pub fn new() -> MatrixData {
        MatrixData {
            mat4s: HashMap::new(),
            inverse: HashMap::new(),
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
