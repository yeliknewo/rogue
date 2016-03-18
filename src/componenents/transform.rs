use std::sync::{Arc, RwLock};

use math::{Vec3};

pub struct Transform {
    position: Arc<RwLock<Vec3>>,
    rotation: Arc<RwLock<Vec3>>,
    scalation: Arc<RwLock<Vec3>>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Arc::new(RwLock::new(Vec3::zero())),
            rotation: Arc::new(RwLock::new(Vec3::zero())),
            scalation: Arc::new(RwLock::new(Vec3::one())),
        }
    }

    pub fn with_position(mut self, pos: Vec3) -> Transform {
        self.position = Arc::new(RwLock::new(pos));
        self
    }

    pub fn with_rotation(mut self, rot: Vec3) -> Transform {
        self.rotation = Arc::new(RwLock::new(rot));
        self
    }

    pub fn with_scalation(mut self, sca: Vec3) -> Transform {
        self.scalation = Arc::new(RwLock::new(sca));
        self
    }
}
