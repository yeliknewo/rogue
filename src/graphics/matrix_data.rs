use std::sync::{Arc};
use std::collections::{HashMap};
use std::fmt;
use std::error::Error;

use logic::{Id};
use math::{Mat4, Vec2, Vec3, Vec4};
use components::{Renderable};

pub struct MatrixData {
    perspective_mat4s: Arc<HashMap<Id, Mat4>>,
    perspective_mat4s_inverse: Arc<HashMap<Id, Mat4>>,
    view_mat4s: Arc<HashMap<Id, Mat4>>,
    view_mat4s_inverse: Arc<HashMap<Id, Mat4>>,
    model_mat4s: Arc<HashMap<Id, Mat4>>,
    model_mat4s_inverse: Arc<HashMap<Id, Mat4>>,
}

impl MatrixData {
    pub fn new() -> MatrixData {
        MatrixData {
            perspective_mat4s: Arc::new(HashMap::new()),
            perspective_mat4s_inverse: Arc::new(HashMap::new()),
            view_mat4s: Arc::new(HashMap::new()),
            view_mat4s_inverse: Arc::new(HashMap::new()),
            model_mat4s: Arc::new(HashMap::new()),
            model_mat4s_inverse: Arc::new(HashMap::new()),
        }
    }

    pub fn backwards2(&self, vec2: Vec2, entity: &Renderable) -> Result<Vec2, MatrixDataErr> {
        Ok(
            Vec2::from(match self.get_perspective_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get Perspective Inverse", Box::new(err))),
            } * match self.get_view_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get View Inverse", Box::new(err))),
            } * match self.get_model_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get Model Inverse", Box::new(err))),
            } * vec2.to_vec4(0.0, 0.0))
        )
    }

    pub fn backwards3(&self, vec3: Vec3, entity: &Renderable) -> Result<Vec3, MatrixDataErr> {
        Ok(
            Vec3::from(match self.get_perspective_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get Perspective Inverse", Box::new(err))),
            } * match self.get_view_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get View Inverse", Box::new(err))),
            } * match self.get_model_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get Model Inverse", Box::new(err))),
            } * vec3.to_vec4(0.0))
        )
    }

    pub fn backwards4(&self, vec4: Vec4, entity: &Renderable) -> Result<Vec4, MatrixDataErr> {
        Ok(
            match self.get_perspective_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get Perspective Inverse", Box::new(err))),
            } * match self.get_view_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get View Inverse", Box::new(err))),
            } * match self.get_model_inverse(entity) {
                Ok(mat) => mat,
                Err(err) => return Err(MatrixDataErr::MatrixData("Self Get Model Inverse", Box::new(err))),
            } * vec4
        )
        // let p = match self.get_perspective_inverse(entity) {
        //     Ok(mat) => mat,
        //     Err(err) => return Err(MatrixDataErr::MatrixData(Box::new(err))),
        // };
        // let v = match self.get_view_inverse(entity) {
        //     Ok(mat) => mat,
        //     Err(err) => return Err(MatrixDataErr::MatrixData(Box::new(err))),
        // };
        // let m = match self.get_model_inverse(entity) {
        //     Ok(mat) => mat,
        //     Err(err) => return Err(MatrixDataErr::MatrixData(Box::new(err))),
        // };
        //
        // return Ok(p * v * m * vec4);
    }

    pub fn get_perspective_matrix(&self, entity: &Renderable) -> Result<Mat4, MatrixDataErr> {
        match self.perspective_mat4s.get(&entity.get_view_id()) {
            Some(matrix) => Ok(*matrix),
            None => Err(MatrixDataErr::Get("Self Perspective Mat4s Get")),
        }
    }

    pub fn get_perspective_inverse(&self, entity: &Renderable) -> Result<Mat4, MatrixDataErr> {
        match self.perspective_mat4s_inverse.get(&entity.get_view_id()) {
            Some(matrix) => Ok(*matrix),
            None => Err(MatrixDataErr::Get("Self Perspective Mat4s Inverse Get")),
        }
    }

    pub fn set_perspective_matrix(&mut self, id: Id, perspective: Mat4, inverse: Mat4) -> Result<(), MatrixDataErr> {
        match Arc::get_mut(&mut self.perspective_mat4s) {
            Some(mat4s) => {
                mat4s.insert(id, perspective);
            },
            None => return Err(MatrixDataErr::GetMut("Arc Get Mut Self Perspective Mat4s")),
        }
        match Arc::get_mut(&mut self.perspective_mat4s_inverse) {
            Some(mat4s) => {
                mat4s.insert(id, inverse);
                Ok(())
            },
            None => Err(MatrixDataErr::GetMut("Arc Get Mut Self Perspective Mat4s Inverse")),
        }
    }

    pub fn get_view_matrix(&self, entity: &Renderable) -> Result<Mat4, MatrixDataErr> {
        match self.view_mat4s.get(&entity.get_view_id()) {
            Some(matrix) => Ok(*matrix),
            None => Err(MatrixDataErr::Get("Self View Mat4s Get")),
        }
    }

    pub fn get_view_inverse(&self, entity: &Renderable) -> Result<Mat4, MatrixDataErr> {
        match self.view_mat4s_inverse.get(&entity.get_view_id()) {
            Some(matrix) => Ok(*matrix),
            None => Err(MatrixDataErr::Get("Self View Mat4s Inverse Get")),
        }
    }

    pub fn set_view_matrix(&mut self, id: Id, view: Mat4, inverse: Mat4) -> Result<(), MatrixDataErr> {
        match Arc::get_mut(&mut self.view_mat4s) {
            Some(mat4s) => {
                mat4s.insert(id, view);
            },
            None => return Err(MatrixDataErr::GetMut("Arc Get Mut Self View Mat4s")),
        }
        match Arc::get_mut(&mut self.view_mat4s_inverse) {
            Some(mat4s) => {
                mat4s.insert(id, inverse);
                Ok(())
            },
            None => Err(MatrixDataErr::GetMut("Arc Get Mut Self View Mat4s Inverse")),
        }
    }

    pub fn get_model_matrix(&self, entity: &Renderable) -> Result<Mat4, MatrixDataErr> {
        match self.model_mat4s.get(&entity.get_model_id()) {
            Some(matrix) => Ok(*matrix),
            None => Err(MatrixDataErr::Get("Self Model Mat4s Get")),
        }
    }

    pub fn get_model_inverse(&self, entity: &Renderable) -> Result<Mat4, MatrixDataErr> {
        match self.model_mat4s_inverse.get(&entity.get_model_id()) {
            Some(matrix) => Ok(*matrix),
            None => Err(MatrixDataErr::Get("Self Model Mat4s Inverse Get")),
        }
    }

    pub fn set_model_matrix(&mut self, id: Id, model: Mat4, inverse: Mat4) -> Result<(), MatrixDataErr> {
        match Arc::get_mut(&mut self.model_mat4s) {
            Some(mat4s) => {
                mat4s.insert(id, model);
            },
            None => return Err(MatrixDataErr::GetMut("Arc Get Mut Self Model Mat4s")),
        }
        match Arc::get_mut(&mut self.model_mat4s_inverse) {
            Some(mat4s) => {
                mat4s.insert(id, inverse);
                Ok(())
            },
            None => Err(MatrixDataErr::GetMut("Arc Get Mut Self Model Mat4s Inverse")),
        }
    }
}

#[derive(Debug)]
pub enum MatrixDataErr {
    Get(&'static str),
    GetMut(&'static str),
    MatrixData(&'static str, Box<MatrixDataErr>)
}

impl fmt::Display for MatrixDataErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MatrixDataErr::Get(_) => write!(f, "Get was None"),
            MatrixDataErr::GetMut(_) => write!(f, "Get Mut was None"),
            MatrixDataErr::MatrixData(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for MatrixDataErr {
    fn description(&self) -> &str {
        match *self {
            MatrixDataErr::Get(_) => "Get was None",
            MatrixDataErr::GetMut(_) => "Get Mut was None",
            MatrixDataErr::MatrixData(_, ref err) => err.description(),
        }
    }
}
