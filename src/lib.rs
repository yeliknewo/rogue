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
#[allow(dead_code)]
mod components;

pub use self::math::{Mat4, Vec2, Vec3, Vec4, DEG_TO_RAD};
pub use self::input::{Keyboard, Mouse, Display};
pub use self::logic::{IdManager, Game, World, Id, IdType, EntityData};
pub use self::graphics::{Window, WindowArgs, DrawMethod, CullingMethod, DepthTestMethod, Vertex, Index};
pub use self::components::{Transform, Renderable, Named};

use std::sync::{Arc, RwLock};

pub fn init() -> Arc<RwLock<IdManager>> {
    graphics::init_vertex();
    Arc::new(RwLock::new(IdManager::new()))
}
