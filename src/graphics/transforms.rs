use std::sync::{Arc};
use std::collections::{HashMap};

use logic::{Id};
use math::{Mat4, Vec2, Vec3, Vec4};
use components::{Renderable};

pub struct Transforms {
    perspective_mat4s: Arc<HashMap<Id, Mat4>>,
    perspective_mat4s_inverse: Arc<HashMap<Id, Mat4>>,
    view_mat4s: Arc<HashMap<Id, Mat4>>,
    view_mat4s_inverse: Arc<HashMap<Id, Mat4>>,
    model_mat4s: Arc<HashMap<Id, Mat4>>,
    model_mat4s_inverse: Arc<HashMap<Id, Mat4>>,
}

impl Transforms {
    pub fn new() -> Transforms {
        Transforms {
            perspective_mat4s: Arc::new(HashMap::new()),
            perspective_mat4s_inverse: Arc::new(HashMap::new()),
            view_mat4s: Arc::new(HashMap::new()),
            view_mat4s_inverse: Arc::new(HashMap::new()),
            model_mat4s: Arc::new(HashMap::new()),
            model_mat4s_inverse: Arc::new(HashMap::new()),
        }
    }

    pub fn backwards2(&self, vec2: Vec2, entity: &Renderable) -> Vec2 {
        Vec2::from(self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec2.to_vec4(0.0, 0.0))
    }

    pub fn backwards3(&self, vec3: Vec3, entity: &Renderable) -> Vec3 {
        Vec3::from(self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec3.to_vec4(0.0))
    }

    pub fn backwards4(&self, vec4: Vec4, entity: &Renderable) -> Vec4 {
        self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec4
    }

    pub fn get_perspective_matrix(&self, entity: &Renderable) -> Mat4 {
        *self.perspective_mat4s.get(&entity.get_perspective_id()).expect("Unable to Get Perspective in Get Perspective")
    }

    pub fn get_perspective_inverse(&self, entity: &Renderable) -> Mat4 {
        *self.perspective_mat4s_inverse.get(&entity.get_perspective_id()).expect("Unable to Get Perspective Inverse in Get Perspective Inverse")
    }

    pub fn set_perspective_matrix(&mut self, id: Id, perspective: Mat4, inverse: Mat4) -> Result<(), TransformsError> {
        match Arc::get_mut(&mut self.perspective_mat4s) {
            Some(mat4s) => {
                mat4s.insert(id, perspective);
            },
            None => return Err(TransformsError::SetPerspectiveMatrix("Unable to Get Mut Perspective Mat4s")),
        }
        match Arc::get_mut(&mut self.perspective_mat4s_inverse) {
            Some(mat4s) => {
                mat4s.insert(id, inverse);
                Ok(())
            },
            None => Err(TransformsError::SetPerspectiveMatrix("Unable to Get Mut Perspective Mat4s Inverse")),
        }
    }

    pub fn get_view_matrix(&self, entity: &Renderable) -> Mat4 {
        *self.view_mat4s.get(&entity.get_view_id()).expect("Unable to Get View in Get View")
    }

    pub fn get_view_inverse(&self, entity: &Renderable) -> Mat4 {
        *self.view_mat4s_inverse.get(&entity.get_view_id()).expect("Unable to Get View Inverse in Get View Inverse")
    }

    pub fn set_view_matrix(&mut self, id: Id, view: Mat4, inverse: Mat4) -> Result<(), TransformsError> {
        match Arc::get_mut(&mut self.view_mat4s) {
            Some(mat4s) => {
                mat4s.insert(id, view);
            },
            None => return Err(TransformsError::SetViewMatrix("Unable to Get Mut View Mat4s")),
        }
        match Arc::get_mut(&mut self.view_mat4s_inverse) {
            Some(mat4s) => {
                mat4s.insert(id, inverse);
                Ok(())
            },
            None => Err(TransformsError::SetViewMatrix("Unable to Get Mut View Mat4s Inverse")),
        }
    }

    pub fn get_model_matrix(&self, entity: &Renderable) -> Mat4 {
        *self.model_mat4s.get(&entity.get_model_id()).expect("Unable to Get Model in Get Model")
    }

    pub fn get_model_inverse(&self, entity: &Renderable) -> Mat4 {
        *self.model_mat4s_inverse.get(&entity.get_model_id()).expect("Unable to Get Model Inverse in Get Model Inverse")
    }

    pub fn set_model_matrix(&mut self, id: Id, model: Mat4, inverse: Mat4) -> Result<(), TransformsError> {
        match Arc::get_mut(&mut self.model_mat4s) {
            Some(mat4s) => {
                mat4s.insert(id, model);
            },
            None => return Err(TransformsError::SetModelMatrix("Unable to Get Mut Model Mat4s")),
        }
        match Arc::get_mut(&mut self.model_mat4s_inverse) {
            Some(mat4s) => {
                mat4s.insert(id, inverse);
                Ok(())
            },
            None => Err(TransformsError::SetModelMatrix("Unable to Get Mut Model Mat4s Inverse")),
        }
    }
}

#[derive(Debug)]
pub enum TransformsError {
    SetPerspectiveMatrix(&'static str),
    SetViewMatrix(&'static str),
    SetModelMatrix(&'static str),
}
