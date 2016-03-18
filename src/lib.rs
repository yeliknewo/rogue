#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;

#[allow(dead_code)]
mod math;
#[allow(dead_code)]
mod input;
#[allow(dead_code)]
mod logic;
#[allow(dead_code)]
mod graphics;

pub use self::math::{Mat4, Vec2, Vec3, Vec4};
pub use self::input::{Keyboard, Mouse, Display};
pub use self::logic::{IDManager, Game, UserData, World, ID, IDType, EntityData};
pub use self::graphics::{Window, WindowArgs, EntityDataGraphics};

use std::sync::{Arc, RwLock};

pub fn init() -> Arc<RwLock<IDManager>> {
    graphics::init_vertex();
    Arc::new(RwLock::new(IDManager::new()))
}
