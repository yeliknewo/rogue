use std::sync::{Arc, RwLock};

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
                Err(_) => return Err(TransformErr::TickMut("Unable to Read Changes")),
            }
        }.dirty {
            match self.update_pos() {
                Ok(b) => {
                    if b {
                        match self.changes.write() {
                            Ok(changes) => changes,
                            Err(_) => return Err(TransformErr::TickMut("Unable to Write Changes")),
                        }.new_position = None;
                    }
                },
                Err(err) => return Err(TransformErr::TickMutTransform(Box::new(err))),
            }
            match self.update_rot() {
                Ok(b) => {
                    if b {
                        match self.changes.write() {
                            Ok(changes) => changes,
                            Err(_) => return Err(TransformErr::TickMut("Unable to Write Changes")),
                        }.new_rotation = None;
                    }
                },
                Err(err) => return Err(TransformErr::TickMutTransform(Box::new(err))),
            }
            match self.update_sca() {
                Ok(b) => {
                    if b {
                        match self.changes.write() {
                            Ok(changes) => changes,
                            Err(_) => return Err(TransformErr::TickMut("Unable to Write Changes")),
                        }.new_scalation = None;
                    }
                },
                Err(err) => return Err(TransformErr::TickMutTransform(Box::new(err))),
            }
            match self.changes.write() {
                Ok(changes) => changes,
                Err(_) => return Err(TransformErr::TickMut("Unable to Write Changes")),
            }.dirty = false;
        }
        if match self.changes.read() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::TickMut("Unable to Read Changes")),
        }.dirty_matrix {
            match renderable.set_model(Mat4::scalation_from_vec3(self.scalation) * Mat4::rotation_from_vec3(self.rotation) * Mat4::translation_from_vec3(self.position)) {
                Ok(()) => (),
                Err(err) => return Err(TransformErr::TickMutRenderable(err)),
            };
            match self.changes.write() {
                Ok(changes) => changes,
                Err(_) => return Err(TransformErr::TickMut("Unable to Write Changes")),
            }.dirty_matrix = false;
        }
        Ok(())
    }

    fn update_pos(&mut self) -> Result<bool, TransformErr> {
        let pos = match self.changes.read() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::UpdatePos("Unable to Read Changes")),
        }.new_position.clone();
        match pos {
            Some(pos) => {
                match self.set_position(pos) {
                    Ok(()) => (),
                    Err(err) => return Err(TransformErr::UpdatePosTransform(Box::new(err))),
                }
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn update_rot(&mut self) -> Result<bool, TransformErr> {
        let rot = match self.changes.read() {
            Ok(rot) => rot,
            Err(_) => return Err(TransformErr::UpdateRot("Unable to Read Changes")),
        }.new_rotation.clone();
        match rot {
            Some(rot) => {
                match self.set_rotation(rot) {
                    Ok(()) => (),
                    Err(err) => return Err(TransformErr::UpdateRotTransform(Box::new(err))),
                }
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn update_sca(&mut self) -> Result<bool, TransformErr> {
        let sca = match self.changes.read() {
            Ok(sca) => sca,
            Err(_) => return Err(TransformErr::UpdateSca("Unable to Read Changes")),
        }.new_scalation.clone();
        match sca {
            Some(sca) => {
                match self.set_scalation(sca) {
                    Ok(()) => (),
                    Err(err) => return Err(TransformErr::UpdateScaTransform(Box::new(err))),
                }
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn prep_set_position(&self, pos: Vec3) -> Result<(), TransformErr> {
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::PrepSetPosition("Unable to Write Changes")),
        }.new_position = Some(pos);
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::PrepSetPosition("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    fn prep_set_rotation(&self, rot: Vec3) -> Result<(), TransformErr> {
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::PrepSetRotation("Unable to Write Changes")),
        }.new_rotation = Some(rot);
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::PrepSetRotation("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    fn prep_set_scalation(&self, sca: Vec3) -> Result<(), TransformErr> {
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::PrepSetScalation("Unable to Write Changes")),
        }.new_scalation = Some(sca);
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::PrepSetScalation("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    fn set_position(&mut self, pos: Vec3) -> Result<(), TransformErr> {
        self.position = pos;
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::SetPosition("Unable to Write Changes")),
        }.dirty_matrix = true;
        Ok(())
    }

    fn set_rotation(&mut self, rot: Vec3) -> Result<(), TransformErr> {
        self.rotation = rot;
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::SetRotation("Unable to Write Changes")),
        }.dirty_matrix = true;
        Ok(())
    }

    fn set_scalation(&mut self, sca: Vec3) -> Result<(), TransformErr> {
        self.scalation = sca;
        match self.changes.write() {
            Ok(changes) => changes,
            Err(_) => return Err(TransformErr::SetScalation("Unable to Write Changes")),
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

pub enum TransformErr {
    TickMut(&'static str),
    TickMutRenderable(RenderableErr),
    TickMutTransform(Box<TransformErr>),
    UpdatePos(&'static str),
    UpdatePosTransform(Box<TransformErr>),
    UpdateRot(&'static str),
    UpdateRotTransform(Box<TransformErr>),
    UpdateSca(&'static str),
    UpdateScaTransform(Box<TransformErr>),
    PrepSetPosition(&'static str),
    PrepSetRotation(&'static str),
    PrepSetScalation(&'static str),
    SetPosition(&'static str),
    SetRotation(&'static str),
    SetScalation(&'static str),
}
