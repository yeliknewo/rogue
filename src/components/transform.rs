use std::sync::{Arc, RwLock};
use std::fmt;
use std::error::Error;

use math::{Vec3, Mat4};
use components::{Renderable, RenderableErr};

struct Changes {
    new_position: Option<Vec3>,
    new_rotation: Option<Vec3>,
    new_scalation: Option<Vec3>,
    dirty: bool,
    dirty_matrix: bool,
}

impl Changes {
    pub fn new() -> Changes {
        Changes {
            new_position: None,
            new_rotation: None,
            new_scalation: None,
            dirty: false,
            dirty_matrix: true,
        }
    }
}

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scalation: Vec3,
    changes: Arc<RwLock<Changes>>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scalation: Vec3::one(),
            changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn tick_mut(&mut self, renderable: &mut Renderable) -> Result<(), TransformErr> {
        if {
            match self.changes.read() {
                Ok(changes) => changes,
                Err(_) => return Err(TransformErr::Poison("Changes Read Dirty")),
            }
        }.dirty {
            match self.update_pos() {
                Ok(b) => {
                    if b {
                        match self.changes.write() {
                            Ok(changes) => changes,
                            Err(_) => return Err(TransformErr::Poison("Changes Write New Position")),
                        }.new_position = None;
                    }
                },
                Err(err) => return Err(TransformErr::Transform("Self Update Pos", Box::new(err))),
            }
            match self.update_rot() {
                Ok(b) => {
                    if b {
                        match self.changes.write() {
                            Ok(changes) => changes,
                            Err(_) => return Err(TransformErr::Poison("Changes Write New Rotation")),
                        }.new_rotation = None;
                    }
                },
                Err(err) => return Err(TransformErr::Transform("Self Update Rot", Box::new(err))),
            }
            match self.update_sca() {
                Ok(b) => {
                    if b {
                        match self.changes.write() {
                            Ok(changes) => changes,
                            Err(_) => return Err(TransformErr::Poison("Changes Write New Scalation")),
                        }.new_scalation = None;
                    }
                },
                Err(err) => return Err(TransformErr::Transform("Self Update Sca", Box::new(err))),
            }
            match self.changes.write() {
                Ok(changes) => changes,
                Err(_) => return Err(TransformErr::Poison("Changes Write Dirty")),
            }.dirty = false;
        }
        if match self.changes.read() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Read Dirty Matrix")),
        }.dirty_matrix {
            match renderable.set_model(Mat4::scalation_from_vec3(self.scalation) * Mat4::rotation_from_vec3(self.rotation) * Mat4::translation_from_vec3(self.position)) {
                Ok(()) => (),
                Err(err) => return Err(TransformErr::Renderable("Renderable Set Model", err)),
            };
            match self.changes.write() {
                Ok(changes) => changes,
                Err(_) => return Err(TransformErr::Poison("Changes Write Dirty Matrix")),
            }.dirty_matrix = false;
        }
        Ok(())
    }

    fn update_pos(&mut self) -> Result<bool, TransformErr> {
        let pos = match self.changes.read() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Read New Position")),
        }.new_position.clone();
        match pos {
            Some(pos) => {
                match self.set_position(pos) {
                    Ok(()) => (),
                    Err(err) => return Err(TransformErr::Transform("Self Set Position", Box::new(err))),
                }
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn update_rot(&mut self) -> Result<bool, TransformErr> {
        let rot = match self.changes.read() {
            Ok(rot) => rot,
            Err(_) => return Err(TransformErr::Poison("Changes Read New Rotation")),
        }.new_rotation.clone();
        match rot {
            Some(rot) => {
                match self.set_rotation(rot) {
                    Ok(()) => (),
                    Err(err) => return Err(TransformErr::Transform("Self Set Rotation", Box::new(err))),
                }
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn update_sca(&mut self) -> Result<bool, TransformErr> {
        let sca = match self.changes.read() {
            Ok(sca) => sca,
            Err(_) => return Err(TransformErr::Poison("Changes Read New Scalation")),
        }.new_scalation.clone();
        match sca {
            Some(sca) => {
                match self.set_scalation(sca) {
                    Ok(()) => (),
                    Err(err) => return Err(TransformErr::Transform("Self Set Scalation", Box::new(err))),
                }
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn prep_set_position(&self, pos: Vec3) -> Result<(), TransformErr> {
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write New Position")),
        }.new_position = Some(pos);
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write Dirty")),
        }.dirty = true;
        Ok(())
    }

    fn prep_set_rotation(&self, rot: Vec3) -> Result<(), TransformErr> {
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write New Rotation")),
        }.new_rotation = Some(rot);
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write Dirty")),
        }.dirty = true;
        Ok(())
    }

    fn prep_set_scalation(&self, sca: Vec3) -> Result<(), TransformErr> {
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write New Scalation")),
        }.new_scalation = Some(sca);
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write Dirty")),
        }.dirty = true;
        Ok(())
    }

    fn set_position(&mut self, pos: Vec3) -> Result<(), TransformErr> {
        self.position = pos;
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write Dirty matrix")),
        }.dirty_matrix = true;
        Ok(())
    }

    fn set_rotation(&mut self, rot: Vec3) -> Result<(), TransformErr> {
        self.rotation = rot;
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write Dirty Matrix")),
        }.dirty_matrix = true;
        Ok(())
    }

    fn set_scalation(&mut self, sca: Vec3) -> Result<(), TransformErr> {
        self.scalation = sca;
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::Poison("Changes Write Dirty Matrix")),
        }.dirty_matrix = true;
        Ok(())
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

#[derive(Debug)]
pub enum TransformErr {
    Poison(&'static str),
    Transform(&'static str, Box<TransformErr>),
    Renderable(&'static str, RenderableErr),
}

impl fmt::Display for TransformErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransformErr::Poison(_) => write!(f, "Thread was Poisoned During R/W"),
            TransformErr::Transform(_, ref err) => err.fmt(f),
            TransformErr::Renderable(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for TransformErr {
    fn description(&self) -> &str {
        match *self {
            TransformErr::Poison(_) => "Thread was Poisoned",
            TransformErr::Transform(_, ref err) => err.description(),
            TransformErr::Renderable(_, ref err) => err.description(),
        }
    }
}
