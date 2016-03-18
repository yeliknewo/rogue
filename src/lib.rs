#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;

mod math;
mod input;
mod logic;
mod graphics;

pub use self::math::{Mat4, Vec2, Vec3, Vec4};
pub use self::input::{Keyboard, Mouse, Display};
pub use self::logic::{IDManager, Game};
pub use self::graphics::{Window, WindowArgs};

use std::sync::{Arc, RwLock};

pub fn init() -> Arc<RwLock<IDManager>> {
    graphics::init_vertex();
    Arc::new(RwLock::new(IDManager::new()))
}
