use std::fmt;
use std::error::Error;
use std::sync::{Arc};

use dorp::{
    Renderable, RenderableErr, Transform, TransformErr, WorldErr, NamedErr, IdManager, World, Id,
    RenderableTex2, Mat4, IdType, Vec3
};
use dorp::graphics2::texture2d::{Vertex, DrawMethod};

use iso2::{Iso2Data, BRICK_TEXTURE};

pub struct Scene;

impl Scene {
    pub fn new() -> Scene {
        Scene
    }

    pub fn tick_mut(&mut self, my_id: Id, manager: &mut IdManager, world: &mut World<Iso2Data>) -> Result<(), SceneErr> {
        world.queue_remove_entity(my_id);
        let texture2d = {
            let mut texture2d = RenderableTex2::new(manager);
            texture2d.set_vertices(vec!(
                    Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
                    Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0]),
                    Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0]),
                    Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0])
            ));
            texture2d.set_indices(vec!(
                0, 1, 2,
                2, 3, 0
            ));
            texture2d.set_texture(BRICK_TEXTURE);
            texture2d.set_draw_method(DrawMethod::Neither);
            texture2d.set_perspective(Mat4::orthographic(0.1, 100.0, 90.0, world.get_aspect_ratio()));
            texture2d.set_view(Mat4::identity());
            texture2d.set_model(Mat4::identity());
            Arc::new(texture2d)
        };
        {
            let id = Id::new(manager, IdType::Entity);

            let mut renderable = Renderable::new();
            {
                let mut texture2d = RenderableTex2::new_from(texture2d);
                texture2d.set_model_id(Id::new(manager, IdType::Matrix));
                texture2d.set_model(Mat4::identity());
                renderable.set_texture2d(texture2d);
            }

            let mut transform = Transform::new();
            transform.set_position(Vec3::from([0.0, 0.0, -10.0]));

            match world.add_entity(
                Iso2Data::new(id)
                .with_renderable(
                    renderable
                )
                .with_transform(
                    transform
                )
            ) {
                Ok(()) => (),
                Err(err) => return Err(SceneErr::World("World Add Entity", err)),
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SceneErr {
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    World(&'static str, WorldErr),
    Named(&'static str, NamedErr),
}

impl fmt::Display for SceneErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SceneErr::Renderable(_, ref err) => err.fmt(f),
            SceneErr::Transform(_, ref err) => err.fmt(f),
            SceneErr::World(_, ref err) => err.fmt(f),
            SceneErr::Named(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for SceneErr {
    fn description(&self) -> &str {
        match *self {
            SceneErr::Renderable(_, ref err) => err.description(),
            SceneErr::Transform(_, ref err) => err.description(),
            SceneErr::World(_, ref err) => err.description(),
            SceneErr::Named(_, ref err) => err.description(),
        }
    }
}
