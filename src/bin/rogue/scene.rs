use std::fmt;
use std::error::Error;
use std::sync::{Arc};

use dorp::{
    Renderable, RenderableErr, Transform, TransformErr, WorldErr, NamedErr, IdManager, World, Id,
    Mat4, IdType, Vec3, RenderableVertexColor, TileMap, Named, Vec4, DEG_TO_RAD
};
use dorp::graphics2::vertex_color;

use rogue::{RogueData};

pub struct Scene;

impl Scene {
    pub fn new() -> Scene {
        Scene
    }

    pub fn tick_mut(&mut self, my_id: Id, manager: &mut IdManager, world: &mut World<RogueData>) -> Result<(), SceneErr> {
        world.queue_remove_entity(my_id);
        {
            let id = Id::new(manager, IdType::Entity);

            let tile_map = TileMap::new();

            let named = match Named::new("TileMap", id, world) {
                Ok(named) => named,
                Err(err) => return Err(SceneErr::Named("Named New", err)),
            };

            match world.add_entity(RogueData::new(id)
                .with_tile_map(
                    tile_map
                )
                .with_named(
                    named
                )
            ) {
                Ok(()) => (),
                Err(err) => return Err(SceneErr::World("World Add Entity", err)),
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
                     2, 3, 0,

                     0, 4, 5,
                     5, 1, 0,

                     1, 2, 6,
                     6, 5, 1,

                     3, 7, 6,
                     6, 2, 3,

                     0, 4, 7,
                     7, 3, 0,

                     4, 5, 6,
                     6, 7, 4
                 ));
                 vertex_color.set_draw_method(vertex_color::DrawMethod::Both(vertex_color::DepthTestMethod::IfLess, vertex_color::CullingMethod::CounterClockwise));
                 vertex_color.set_perspective(Mat4::orthographic(0.1, 100.0, 90.0, world.get_aspect_ratio()));
                 vertex_color.set_view(Mat4::x_rotation(30.0 * DEG_TO_RAD) * Mat4::y_rotation(-30.0 * DEG_TO_RAD));
                 vertex_color.set_model(Mat4::identity());
                Arc::new(vertex_color)
            };
            {
                let mut colors : Vec<Vec4> = vec!();
                let x0: i32 = -10;
                let x1 = 11;
                let y0: i32 = -10;
                let y1 = 11;
                let scale = 0.1;
                let width = x1 - x0;
                let height = y1 - y0;
                for y in y0..y1 {
                    for x in x0..x1 {
                        colors.push(Vec4::from([(x + y).abs() as f32 % 0.9, (x).abs() as f32 % 0.9, (y).abs() as f32 % 0.9, 1.0]))
                    }
                }
                for y in 0..height - 1 {
                    for x in 0..width - 1 {
                        let id = Id::new(manager, IdType::Entity);

                        let mut renderable = Renderable::new();
                        {
                            let mut vertex_color = RenderableVertexColor::new_from(vertex_color.clone());
                            vertex_color.set_vertex_id(Id::new(manager, IdType::Vertex));
                            vertex_color.set_vertices(vec!(
                                vertex_color::Vertex::new([0.0, 0.0, 0.0], match colors.get(((y + 0) * width + (x + 0)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([0.0, 1.0, 0.0], match colors.get(((y + 1) * width + (x + 0)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([1.0, 1.0, 0.0], match colors.get(((y + 1) * width + (x + 1)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([1.0, 0.0, 0.0], match colors.get(((y + 0) * width + (x + 1)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([0.0, 0.0, 1.0], match colors.get(((y + 0) * width + (x + 0)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([0.0, 1.0, 1.0], match colors.get(((y + 1) * width + (x + 0)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([1.0, 1.0, 1.0], match colors.get(((y + 1) * width + (x + 1)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                                vertex_color::Vertex::new([1.0, 0.0, 1.0], match colors.get(((y + 0) * width + (x + 1)) as usize) {
                                    Some(color) => color.get_vals(),
                                    None => return Err(SceneErr::Get("Colors Get")),
                                }),
                            ));
                            vertex_color.set_model_id(Id::new(manager, IdType::Matrix));
                            vertex_color.set_model(Mat4::identity());
                            renderable.set_vertex_color(vertex_color);
                        }

                        let mut transform  = Transform::new();
                        let sx = x as f32 * scale;
                        let sy = y as f32 * scale;
                        let sx0 = (x + x0) as f32 * scale;
                        let sy0 = (y + y0) as f32 * scale;
                        transform.set_position(Vec3::from([sx0, sy0, 0.0]));
                        transform.set_scalation(Vec3::from([scale, scale, scale]));

                        match world.add_entity(RogueData::new(id)
                            .with_renderable(renderable)
                            .with_transform(transform)
                        ) {
                            Ok(()) => (),
                            Err(err) => return Err(SceneErr::World("World Add Entity", err)),
                        }
                    }
                }
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
    Get(&'static str),
}

impl fmt::Display for SceneErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SceneErr::Renderable(_, ref err) => err.fmt(f),
            SceneErr::Transform(_, ref err) => err.fmt(f),
            SceneErr::World(_, ref err) => err.fmt(f),
            SceneErr::Named(_, ref err) => err.fmt(f),
            SceneErr::Get(_) => write!(f, "Get was None"),
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
            SceneErr::Get(_) => "Get was None",
        }
    }
}
