use std::fmt;
use std::error::Error;
use std::sync::{Arc};

use dorp::{
    Renderable, RenderableErr, Transform, TransformErr, WorldErr, NamedErr, IdManager, World, Id,
    RenderableTex2, Mat4, IdType, Vec3, RenderableSolidColor, Vec4, RenderableVertexColor
};
use dorp::graphics2::texture2d;
use dorp::graphics2::solid_color;
use dorp::graphics2::vertex_color;

use iso2::{Iso2Data, BRICK_TEXTURE};

pub struct Scene;

impl Scene {
    pub fn new() -> Scene {
        Scene
    }

    pub fn tick_mut(&mut self, my_id: Id, manager: &mut IdManager, world: &mut World<Iso2Data>) -> Result<(), SceneErr> {
        world.queue_remove_entity(my_id);
        {
            let texture2d = {
                let mut texture2d = RenderableTex2::new(manager);
                texture2d.set_vertices(vec!(
                        texture2d::Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
                        texture2d::Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0]),
                        texture2d::Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0]),
                        texture2d::Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0])
                ));
                texture2d.set_indices(vec!(
                    0, 1, 2,
                    2, 3, 0
                ));
                texture2d.set_texture(BRICK_TEXTURE);
                texture2d.set_draw_method(texture2d::DrawMethod::Neither);
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
                transform.set_position(Vec3::from([0.0, 0.0, 0.0]));

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
        }
        {
            let solid_color = {
                let mut solid_color = RenderableSolidColor::new(manager);
                solid_color.set_vertices(vec!(
                    solid_color::Vertex::new([0.0, 0.0, 0.0]),
                    solid_color::Vertex::new([1.0, 0.0, 0.0]),
                    solid_color::Vertex::new([1.0, 1.0, 0.0]),
                    solid_color::Vertex::new([0.0, 1.0, 0.0]),
                ));
                solid_color.set_indices(vec!(
                    0, 1, 2,
                    2, 3, 0
                ));
                solid_color.set_draw_method(solid_color::DrawMethod::Neither);
                solid_color.set_perspective(Mat4::orthographic(0.1, 100.0, 90.0, world.get_aspect_ratio()));
                solid_color.set_view(Mat4::identity());
                solid_color.set_model(Mat4::identity());
                solid_color.set_color(Vec4::from([1.0, 0.0, 0.0, 1.0]));
                Arc::new(solid_color)
            };
            {
                let id = Id::new(manager, IdType::Entity);

                let mut renderable = Renderable::new();
                {
                    let mut solid_color = RenderableSolidColor::new_from(solid_color);
                    solid_color.set_model_id(Id::new(manager, IdType::Matrix));
                    solid_color.set_model(Mat4::identity());
                    renderable.set_solid_color(solid_color);
                }

                let mut transform = Transform::new();
                transform.set_position(Vec3::from([-1.0, 0.0, 0.0]));

                match world.add_entity(Iso2Data::new(id)
                    .with_renderable(
                        renderable
                    )
                    .with_transform(
                        transform
                    )
                ) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::World("World Add Entity", err)),
                };
            }
        }
        {
            let vertex_color = {
                let mut vertex_color = RenderableVertexColor::new(manager);
                vertex_color.set_vertices(vec!(
                    vertex_color::Vertex::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]),
                    vertex_color::Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0, 1.0]),
                    vertex_color::Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0, 1.0, 1.0]),
                    vertex_color::Vertex::new([0.0, 1.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
                ));
                vertex_color.set_indices(vec!(
                    0, 1, 2,
                    2, 3, 0
                ));
                vertex_color.set_draw_method(vertex_color::DrawMethod::Neither);
                vertex_color.set_perspective(Mat4::orthographic(0.1, 100.0, 90.0, world.get_aspect_ratio()));
                vertex_color.set_view(Mat4::identity());
                vertex_color.set_model(Mat4::identity());
                Arc::new(vertex_color)
            };
            {
                let id = Id::new(manager, IdType::Entity);

                let mut renderable = Renderable::new();
                {
                    let mut vertex_color = RenderableVertexColor::new_from(vertex_color);
                    vertex_color.set_model_id(Id::new(manager, IdType::Matrix));
                    vertex_color.set_model(Mat4::identity());
                    renderable.set_vertex_color(vertex_color);
                }

                let mut transform = Transform::new();
                transform.set_position(Vec3::from([-1.0, -1.0, 0.0]));

                match world.add_entity(Iso2Data::new(id)
                    .with_renderable(
                        renderable
                    )
                    .with_transform(
                        transform
                    )
                ) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::World("World Add Entity", err)),
                };
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
