use std::sync::{Arc, RwLock};
use dorp::{init, Game, Window, WindowArgs, Id, IdType, Mat4, Vec3,
     DEG_TO_RAD, Transform, Renderable, DrawMethod, DepthTestMethod, CullingMethod, Vertex,
     Named
};

static STONE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");
static WOOD_TEXTURE: &'static [u8] = include_bytes!("../../../assets/wood.png");

static TILE_MAP_NAME: &'static str = "TileMap";

mod iso_data;
mod tile;
mod item;
mod being;
mod tile_map;

pub use self::iso_data::{IsoData};
pub use self::tile::{Tile};
pub use self::item::{Item, ItemType};
pub use self::being::{Being};
pub use self::tile_map::{TileMap};

pub fn main() {
    let manager = init();

    let mut window = Window::new(WindowArgs::Borderless("Iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game = Game::<IsoData>::new(manager.clone(), thread_count, resolution);

    let world = game.get_world();
    let data = world.get_entity_data();
    {
        let mut data = data.write().expect("Unable to Write Entity Data in Main in Iso");
        let id = Id::new(manager.clone(), IdType::Entity);
        data.insert(id, Arc::new(RwLock::new(
            IsoData::new(id)
            .with_named(Named::new(TILE_MAP_NAME))
            .with_tile_map(TileMap::new())
        )));
        let tile_graphics = Renderable::new(manager.clone())
            .with_vertices(vec!(
                Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
                Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0]),
                Vertex::new([1.0, 0.0, 1.0], [1.0, 1.0]),
                Vertex::new([0.0, 0.0, 1.0], [0.0, 1.0]),
            ))
            .with_indices(vec!(
                0, 1, 2,
                2, 3, 0,
            ))
            .with_texture(STONE_TEXTURE)
            .with_draw_method(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::CounterClockwise))
            .with_perspective(Mat4::orthographic(0.1, 100.0, 90.0, 16.0 / 9.0))
            .with_view(Mat4::x_rotation(-45.0 * DEG_TO_RAD) * Mat4::y_rotation((180.0 + 45.0) * DEG_TO_RAD) * Mat4::translation_from_vec3(Vec3::from([0.0, -2.0, -3.0])))
            .with_model(Mat4::identity());
        let item_graphics = tile_graphics.clone()
            .with_texture_id(Id::new(manager.clone(), IdType::Texture))
            .with_texture(WOOD_TEXTURE);
        let item_id = Id::new(manager.clone(), IdType::Entity);
        data.insert(item_id, Arc::new(RwLock::new(
            IsoData::new(item_id)
            .with_renderable(
                item_graphics.clone()
                .with_model_id(Id::new(manager.clone(), IdType::Model))
                .with_model(Mat4::identity())
            )
            .with_transform(
                Transform::new()
                .with_scalation(Vec3::from([0.5, 1.0, 0.5]))
            )
            .with_named(
                Named::new("Item")
            )
            .with_item(
                Item::new(ItemType::Planks)
            )
        )));
        let mut has_item = true;
        for z in 0..10 {
            for x in 0..10 {
                let id = Id::new(manager.clone(), IdType::Entity);
                if has_item && z > 3{
                    data.insert(id, Arc::new(RwLock::new(
                        IsoData::new(id)
                        .with_renderable(
                            tile_graphics.clone()
                            .with_model_id(Id::new(manager.clone(), IdType::Model))
                            .with_model(Mat4::identity())
                        )
                        .with_transform(
                            Transform::new()
                            .with_position(Vec3::from([x as f32, 0.0, z as f32]))
                            .with_rotation(Vec3::from([0.0, 0.0, 0.0]))
                            .with_scalation(Vec3::one())
                        )
                        .with_tile(
                            Tile::new(TILE_MAP_NAME, x, z)
                            .with_on_tile(item_id)
                        )
                    )));
                    has_item = false;
                } else {
                    data.insert(id, Arc::new(RwLock::new(
                        IsoData::new(id)
                        .with_renderable(
                            tile_graphics.clone()
                            .with_model_id(Id::new(manager.clone(), IdType::Model))
                            .with_model(Mat4::identity())
                        )
                        .with_transform(
                            Transform::new()
                            .with_position(Vec3::from([x as f32, 0.0, z as f32]))
                            .with_rotation(Vec3::from([0.0, 0.0, 0.0]))
                            .with_scalation(Vec3::one())
                        )
                        .with_tile(
                            Tile::new(TILE_MAP_NAME, x, z)
                        )
                    )));
                }
            }
        }
    }

    game.run(&mut window);
}
