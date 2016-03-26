use std::fmt;
use std::error::Error;
use std::sync::{Arc};

use dorp::{
    IdManager, World, WorldErr, Renderable, RenderableErr, Transform, TransformErr, Id, IdType,
    Mat4, Vec3, Vertex, DrawMethod, Named, NamedErr
};

use life::{LIFE_TEXTURE, LifeData, TileMap, Cell, TILE_MAP_NAME};

pub struct Scene {
    running: bool,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            running: true,
        }
    }

    pub fn tick_mut(&mut self, manager: &mut IdManager, world: &mut World<LifeData>) -> Result<(), SceneErr> {
        if self.running {
            let mut tile_map = TileMap::new();

            let renderable = {
                let mut renderable = Renderable::new(manager);
                match renderable.set_vertices(vec!(
                    Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
                    Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0]),
                    Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0]),
                    Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0])
                )) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set Vertices", err)),
                }
                match renderable.set_indices(vec!(
                    0, 1, 2,
                    2, 3, 0
                )) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set Indices", err)),
                }
                match renderable.set_texture(LIFE_TEXTURE) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set Texture Life Texture", err)),
                }
                match renderable.set_draw_method(DrawMethod::Neither) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set Draw Method Neither", err)),
                }
                match renderable.set_perspective(Mat4::orthographic(0.1, 100.0, 90.0, world.get_aspect_ratio())) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set Perspective Orthographic", err)),
                }
                match renderable.set_view(Mat4::identity()) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set View Identity", err)),
                }
                match renderable.set_model(Mat4::identity()) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::Renderable("Renderable Set Model Identity", err)),
                }
                Arc::new(renderable)
            };
            for y in -5..6{
                for x in -5..6 {
                    let id = Id::new(manager, IdType::Entity);

                    let mut renderable = match Renderable::new_from(renderable.clone()) {
                        Ok(renderable) => renderable,
                        Err(err) => return Err(SceneErr::Renderable("Renderable New From Renderable", err)),
                    };
                    renderable.set_model_id(Id::new(manager, IdType::Model));
                    match renderable.set_model(Mat4::identity()) {
                        Ok(()) => (),
                        Err(err) => return Err(SceneErr::Renderable("Renderable Set Model Mat4 Identity", err)),
                    }

                    let mut transform = Transform::new();
                    match transform.set_position(Vec3::from([x as f32, y as f32, -10.0])) {
                        Ok(()) => (),
                        Err(err) => return Err(SceneErr::Transform("Transform Set Position", err)),
                    }
                    
                    let cell = Cell::new(x, y, id, y % 2 < x % 2, &mut tile_map);

                    match world.add_entity(
                        LifeData::new(id)
                        .with_renderable(
                            renderable
                        )
                        .with_transform(
                            transform
                        )
                        .with_cell(
                            cell
                        )
                    ) {
                        Ok(()) => (),
                        Err(err) => return Err(SceneErr::World("World Add Entity", err)),
                    }
                }
            }
            {
                let id = Id::new(manager, IdType::Entity);

                let named = match Named::new(TILE_MAP_NAME, id, world) {
                    Ok(named) => named,
                    Err(err) => return Err(SceneErr::Named("Named New", err)),
                };
                match world.add_entity(
                    LifeData::new(id)
                    .with_tile_map(
                        tile_map
                    )
                    .with_named(
                        named
                    )
                ) {
                    Ok(()) => (),
                    Err(err) => return Err(SceneErr::World("World Add Entity TileMap", err)),
                };
            }
            self.running = false;
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
