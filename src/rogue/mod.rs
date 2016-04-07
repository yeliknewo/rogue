use std::sync::{Arc};
use std::fmt;
use std::error::{Error};

use dorp::{
    WindowBuilder, Game, Vec2, Renderable, Transform, IdManager, Id, Mat4, IdType, Vec3,
    RenderableVertexColor, Named, Vec4, DEG_TO_RAD, Scene, WorldErr, NamedErr, OptErr
};
use dorp::graphics::vertex_color;

mod rogue_data;
pub mod block;
pub mod utils;

pub use self::rogue_data::{RogueData, RogueDataErr};
pub use self::block::{Block, BlockErr, BlockCoords, BlockMap, BlockType};
pub use self::utils::{RogueWorld};

pub static BLOCK_MAP_NAME: &'static str = "BlockMap";

pub fn main() {
    let mut manager = IdManager::new();
    let (mut window, resolution) = WindowBuilder::new()
        .with_title("Rogue".to_string())
        .build()
        .unwrap();
    let thread_count = 8;
    let mut game = Game::<RogueData>::new(thread_count, Vec2::from([resolution.0 as f32, resolution.1 as f32]));
    {
        let mut world = game.get_mut_world().unwrap();
        {
            let id = Id::new(&mut manager, IdType::Entity);
            let scene = Scene::new(Box::new(|manager, world| {
                let block_map_id = {
                    let id = Id::new(manager, IdType::Entity);
                    let block_map = BlockMap::new();
                    let named = match Named::new(BLOCK_MAP_NAME, id, world) {
                        Ok(named) => named,
                        Err(err) => return Err(Box::new(SceneErr::Named("Named New", err))),
                    };

                    match world.add_entity(RogueData::new(id)
                        .with_block_map(
                            block_map
                        )
                        .with_named(
                            named
                        )
                    ) {
                        Ok(()) => (),
                        Err(err) => return Err(Box::new(SceneErr::World("World Add Entity", err))),
                    }
                    id
                };
                {
                    let vertex_color = {
                        let mut vertex_color = RenderableVertexColor::new(manager);
                        vertex_color.set_indices(vec!(
                            0, 1, 2,
                            2, 3, 0,

                            5, 4, 0,
                            0, 1, 5,

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
                         vertex_color.set_view(Mat4::x_rotation(45.0 * DEG_TO_RAD) * Mat4::y_rotation(45.0 * DEG_TO_RAD));
                         Arc::new(vertex_color)
                    };
                    {
                        let mut colors : Vec<Vec4> = vec!();
                        let p0 = Vec3::from([-10.0, -1.0, -10.0]);
                        let p1 = Vec3::from([10.0, 1.0, 10.0]);
                        let scale = {
                            let b = p1 - p0;
                            Vec3::from([1.0 / b[0], 1.0 / b[1], 1.0 / b[2]])
                        };
                        let width = (p1[0] - p0[0]) as i32;
                        let height = (p1[1] - p0[1]) as i32;
                        let depth = (p1[2] - p0[2]) as i32;
                        for z in p0[2] as i32..p1[2] as i32 {
                            for y in p0[1] as i32..p1[1] as i32 {
                                for x in p0[0] as i32..p1[0] as i32 {
                                    colors.push(Vec4::from([(x).abs() as f32 % 0.9, (y).abs() as f32 % 0.9, (z).abs() as f32 % 0.9, 1.0]))
                                }
                            }
                        }
                        for z in 0..depth - 1 {
                            for y in 0..height - 1 {
                                for x in 0..width - 1 {
                                    let id = Id::new(manager, IdType::Entity);
                                    let mut renderable = Renderable::new();
                                    {
                                        let mut vertex_color = RenderableVertexColor::new_from(vertex_color.clone());
                                        vertex_color.set_vertex_id(Id::new(manager, IdType::Vertex));
                                        vertex_color.set_vertices(vec!(
                                            vertex_color::Vertex::new([0.0, 0.0, 0.0], match colors.get(((z + 0) * height * width + (y + 0) * width + (x + 0)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([0.0, 1.0, 0.0], match colors.get(((z + 0) * height * width + (y + 1) * width + (x + 0)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([1.0, 1.0, 0.0], match colors.get(((z + 0) * height * width + (y + 1) * width + (x + 1)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([1.0, 0.0, 0.0], match colors.get(((z + 0) * height * width + (y + 0) * width + (x + 1)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([0.0, 0.0, 1.0], match colors.get(((z + 1) * height * width + (y + 0) * width + (x + 0)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([0.0, 1.0, 1.0], match colors.get(((z + 1) * height * width + (y + 1) * width + (x + 0)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([1.0, 1.0, 1.0], match colors.get(((z + 1) * height * width + (y + 1) * width + (x + 1)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                            vertex_color::Vertex::new([1.0, 0.0, 1.0], match colors.get(((z + 1) * height * width + (y + 0) * width + (x + 1)) as usize) {
                                                Some(color) => color.get_vals(),
                                                None => return Err(Box::new(SceneErr::Get("Colors Get"))),
                                            }),
                                        ));
                                        vertex_color.set_model_id(Id::new(manager, IdType::Matrix));
                                        vertex_color.set_model(Mat4::identity());
                                        renderable.set_vertex_color(vertex_color);
                                    }
                                    let mut transform  = Transform::new();
                                    let sx0 = (x as f32 + p0[0]) * scale[0];
                                    let sy0 = (y as f32 + p0[1]) * scale[1];
                                    let sz0 = (z as f32 + p0[2]) * scale[2];
                                    transform.set_position(Vec3::from([sx0, sy0, sz0]));
                                    transform.set_scalation(scale);

                                    let block_coords = BlockCoords::new(x as i64, y as i64, z as i64);

                                    println!("Block Created");
                                    let block = {
                                        let mut block_map = match world.get_mut_entity_by_id(block_map_id) {
                                            OptErr::Full(block_map_entity) => match block_map_entity.get_mut_block_map() {
                                                OptErr::Full(block_map) => block_map,
                                                OptErr::Empty => return Err(Box::new(SceneErr::Get("Block Map Entity Get Mut Block Map"))),
                                                OptErr::Error(err) => return Err(Box::new(SceneErr::RogueData("Block Map Entity Get mut Block Map", err))),
                                            },
                                            OptErr::Empty => return Err(Box::new(SceneErr::Get("World Get Mut Entity By Id Block Map Id"))),
                                            OptErr::Error(err) => return Err(Box::new(SceneErr::World("World Get Mut Entity By Id Block map Id", err))),
                                        };
                                        Block::new_with_block_map(BlockType::Dirt, id, &block_coords, &mut block_map, block_map_id)
                                    };

                                    match world.add_entity(RogueData::new(id)
                                        .with_renderable(renderable)
                                        .with_transform(transform)
                                        .with_block_coords(block_coords)
                                        .with_block(block)
                                    ) {
                                        Ok(()) => (),
                                        Err(err) => return Err(Box::new(SceneErr::World("World Add Entity", err))),
                                    }
                                }
                            }
                        }
                    }
                }
                println!("Scene Loaded");
                Ok(())
            }));
            world.add_entity(
                RogueData::new(id)
                .with_scene(
                    scene
                )
            ).unwrap();
        }
    }
    println!("Starting Run Loop");
    game.run(&mut window, &mut manager).unwrap();
}

#[derive(Debug)]
enum SceneErr {
    World(&'static str, WorldErr),
    Named(&'static str, NamedErr),
    Block(&'static str, BlockErr),
    RogueData(&'static str, RogueDataErr),
    Get(&'static str),
}

impl fmt::Display for SceneErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SceneErr::World(_, ref err) => err.fmt(f),
            SceneErr::Named(_, ref err) => err.fmt(f),
            SceneErr::Block(_, ref err) => err.fmt(f),
            SceneErr::RogueData(_, ref err) => err.fmt(f),
            SceneErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for SceneErr {
    fn description(&self) -> &str {
        match *self {
            SceneErr::World(_, ref err) => err.description(),
            SceneErr::Named(_, ref err) => err.description(),
            SceneErr::Block(_, ref err) => err.description(),
            SceneErr::RogueData(_, ref err) => err.description(),
            SceneErr::Get(_) => "Get was None",
        }
    }
}
