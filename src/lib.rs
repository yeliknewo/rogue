#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;

mod math;
mod input;
mod logic;
mod graphics;
mod components;
mod err;

pub use self::math::{Mat4, Vec2, Vec3, Vec4, DEG_TO_RAD};
pub use self::input::{Keyboard, Mouse, Display};
pub use self::logic::{IdManager, Game, World, WorldErr, Id, IdType, EntityData};
pub use self::graphics::{Window, WindowArgs, DrawMethod, CullingMethod, DepthTestMethod, Vertex, Index, MatrixData};
pub use self::components::{Transform, TransformErr, Renderable, RenderableErr, Named};
pub use self::err::{Error};

pub fn init() -> IdManager {
    graphics::init_vertex();
    IdManager::new()
}
