use std::sync::{Arc, RwLock};
use std::collections::{HashMap};

use logic::{ID};
use math::{Mat4, Vec2, Vec3, Vec4};
use graphics::{EntityDataGraphics};

pub struct Transforms {
    perspective_mat4s: Arc<RwLock<HashMap<ID, Mat4>>>,
    perspective_mat4s_inverse: Arc<RwLock<HashMap<ID, Mat4>>>,
    view_mat4s: Arc<RwLock<HashMap<ID, Mat4>>>,
    view_mat4s_inverse: Arc<RwLock<HashMap<ID, Mat4>>>,
    model_mat4s: Arc<RwLock<HashMap<ID, Mat4>>>,
    model_mat4s_inverse: Arc<RwLock<HashMap<ID, Mat4>>>,
}

impl Transforms {
    pub fn new() -> Transforms {
        Transforms {
            perspective_mat4s: Arc::new(RwLock::new(HashMap::new())),
            perspective_mat4s_inverse: Arc::new(RwLock::new(HashMap::new())),
            view_mat4s: Arc::new(RwLock::new(HashMap::new())),
            view_mat4s_inverse: Arc::new(RwLock::new(HashMap::new())),
            model_mat4s: Arc::new(RwLock::new(HashMap::new())),
            model_mat4s_inverse: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn backwards2(&self, vec2: Vec2, entity: &EntityDataGraphics) -> Vec2 {
        Vec2::from(self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec2.to_vec4(0.0, 0.0))
    }

    pub fn backwards3(&self, vec3: Vec3, entity: &EntityDataGraphics) -> Vec3 {
        Vec3::from(self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec3.to_vec4(0.0))
    }

    pub fn backwards4(&self, vec4: Vec4, entity: &EntityDataGraphics) -> Vec4 {
        self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec4
    }

    pub fn get_perspective_matrix(&self, entity: &EntityDataGraphics) -> Mat4 {
        *self.perspective_mat4s.read().expect("Unable to Read Perspective Matrix in Transforms").get(&entity.get_perspective_id()).expect("Unable to Get Perspective in Get Perspective")
    }

    pub fn get_perspective_inverse(&self, entity: &EntityDataGraphics) -> Mat4 {
        *self.perspective_mat4s_inverse.read().expect("Unable to Read Perspective Inverse in Transforms").get(&entity.get_perspective_id()).expect("Unable to Get Perspective Inverse in Get Perspective Inverse")
    }

    pub fn set_perspective_matrix(&self, entity: &EntityDataGraphics, perspective: Mat4, inverse: Mat4) {
        self.perspective_mat4s.write().expect("Unable to Write Perspective Matrix in Set Perspective Matrix in Transforms").insert(entity.get_perspective_id(), perspective);
        self.perspective_mat4s_inverse.write().expect("Unable to Write Perspective Inverse in Set Perspective Matrix in Transforms").insert(entity.get_perspective_id(), inverse);
    }

    pub fn get_view_matrix(&self, entity: &EntityDataGraphics) -> Mat4 {
        *self.view_mat4s.read().expect("Unable to Read View Matrix in Get View Matrix in Transforms").get(&entity.get_view_id()).expect("Unable to Get View in Get View")
    }

    pub fn get_view_inverse(&self, entity: &EntityDataGraphics) -> Mat4 {
        *self.view_mat4s_inverse.read().expect("Unable to Read View Inverse in Get View Inverse in Transforms").get(&entity.get_view_id()).expect("Unable to Get View Inverse in Get View Inverse")
    }

    pub fn set_view_matrix(&self, entity: &EntityDataGraphics, view: Mat4, inverse: Mat4) {
        self.view_mat4s.write().expect("Unable to Write View Matrix in Set View Matrix in Transforms").insert(entity.get_view_id(), view);
        self.view_mat4s_inverse.write().expect("Unable to Write View Inverse in Set View Matrix in Transforms").insert(entity.get_view_id(), inverse);
    }

    pub fn get_model_matrix(&self, entity: &EntityDataGraphics) -> Mat4 {
        *self.model_mat4s.read().expect("Unable to Read Model Matrix in Get Model Matrix in Transforms").get(&entity.get_model_id()).expect("Unable to Get Model in Get Model")
    }

    pub fn get_model_inverse(&self, entity: &EntityDataGraphics) -> Mat4 {
        *self.model_mat4s_inverse.read().expect("Unable to Read Model Inverse in Get Model Inverse in Transforms").get(&entity.get_model_id()).expect("Unable to Get Model Inverse in Get Model Inverse")
    }

    pub fn set_model_matrix(&self, entity: &EntityDataGraphics, model: Mat4, inverse: Mat4) {
        self.model_mat4s.write().expect("Unable to Write Model Matrix in Set Model Matrix in Transforms").insert(entity.get_model_id(), model);
        self.model_mat4s_inverse.write().expect("Unable to Write Model Inverse in Set Model Matrix in Transforms").insert(entity.get_model_id(), inverse);
    }
}
