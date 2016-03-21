use std::sync::{Arc};

use dorp::{init, Game, Window, WindowArgs, Id, IdType, Mat4, Vec3,
     DEG_TO_RAD, Transform, Renderable, DrawMethod, DepthTestMethod, CullingMethod, Vertex,
     Named
};

pub static STONE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");
pub static WOOD_TEXTURE: &'static [u8] = include_bytes!("../../../assets/wood.png");
pub static BEING_TEXTURE: &'static [u8] = include_bytes!("../../../assets/being.png");

pub static TILE_MAP_NAME: &'static str = "TileMap";
pub static ITEM_NAME: &'static str = "Item";

mod iso_data;
mod tile;
mod item;
mod being;
mod tile_map;
mod tile_coordinates;

pub use self::iso_data::{IsoData};
pub use self::tile::{Tile};
pub use self::item::{Item, ItemType};
pub use self::being::{Being};
pub use self::tile_map::{TileMap};
pub use self::tile_coordinates::{TileCoordinates};

pub fn main() {
    let mut manager = init();

    let mut window = Window::new(WindowArgs::Borderless("Iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game = Game::<IsoData>::new(thread_count, resolution);
    // {
    //     {
    //         let id = Id::new(&mut manager, IdType::Entity);
    //         world.add_entity(
    //             IsoData::new(id)
    //             .with_named(Named::new(TILE_MAP_NAME, id, &mut world))
    //             .with_tile_map(TileMap::new())
    //         );
    //     }
    //     let tile_graphics = Renderable::new(manager.clone())
    //         .with_vertices(vec!(
    //             Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
    //             Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0]),
    //             Vertex::new([1.0, 0.0, 1.0], [1.0, 1.0]),
    //             Vertex::new([0.0, 0.0, 1.0], [0.0, 1.0]),
    //         ))
    //         .with_indices(vec!(
    //             0, 1, 2,
    //             2, 3, 0,
    //         ))
    //         .with_texture(STONE_TEXTURE)
    //         .with_draw_method(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::CounterClockwise))
    //         .with_perspective(Mat4::orthographic(0.1, 100.0, 90.0, 16.0 / 9.0))
    //         .with_view(Mat4::x_rotation(-45.0 * DEG_TO_RAD) * Mat4::y_rotation((180.0 + 45.0) * DEG_TO_RAD) * Mat4::translation_from_vec3(Vec3::from([0.0, -2.0, -3.0])))
    //         .with_model(Mat4::identity());
    //     for z in 0..10 {
    //         for x in 0..10 {
    //             let id = Id::new(manager.clone(), IdType::Entity);
    //             let tile_coordinates = Arc::new(TileCoordinates::new_no_move(x, z, id));
    //             world.add_entity(
    //                 IsoData::new(id)
    //                 .with_renderable(
    //                     tile_graphics.clone()
    //                     .with_new_model(Mat4::identity(), manager.clone())
    //                 )
    //                 .with_transform(
    //                     Transform::new()
    //                     .with_position(Vec3::from([x as f32, 0.0, z as f32]))
    //                     .with_rotation(Vec3::from([0.0, 0.0, 0.0]))
    //                     .with_scalation(Vec3::one())
    //                 )
    //                 .with_tile(
    //                     Tile::new(id, TILE_MAP_NAME, tile_coordinates.clone(), world.clone())
    //                 )
    //                 .with_tile_coordinates_arc(
    //                     tile_coordinates
    //                 )
    //             );
    //         }
    //     }
    //     {
    //         let item_graphics = tile_graphics.clone()
    //             .with_new_texture(WOOD_TEXTURE, manager.clone());
    //         let item_id = Id::new(manager.clone(), IdType::Entity);
    //         world.add_entity(
    //             IsoData::new(item_id)
    //             .with_renderable(
    //                 item_graphics.clone()
    //                 .with_new_model(Mat4::identity(), manager.clone())
    //             )
    //             .with_transform(
    //                 Transform::new()
    //                 .with_scalation(Vec3::from([0.5, 1.0, 0.5]))
    //             )
    //             .with_tile_coordinates(
    //                 TileCoordinates::new_find(0, 3, item_id, TILE_MAP_NAME, world.clone())
    //             )
    //             .with_named(
    //                 Named::new(ITEM_NAME, item_id, world.clone())
    //             )
    //             .with_item(
    //                 Item::new(ItemType::Planks)
    //             )
    //         );
    //     }
    //     {
    //         let id = Id::new(manager.clone(), IdType::Entity);
    //         world.add_entity(
    //             IsoData::new(id)
    //             .with_renderable(
    //                 tile_graphics.clone()
    //                 .with_new_model(Mat4::identity(), manager.clone())
    //                 .with_new_texture(BEING_TEXTURE, manager.clone())
    //             )
    //             .with_transform(
    //                 Transform::new()
    //                 .with_scalation(Vec3::from([0.5, 1.0, 0.5]))
    //             )
    //             .with_being(
    //                 Being::new(5)
    //             )
    //             .with_tile_coordinates(
    //                 TileCoordinates::new_find(0, 0, id, TILE_MAP_NAME, world.clone())
    //             )
    //         );
    //     }
    // }
    println!("Starting Run Loop");
    game.run(&mut window);
}
