use dorp::{init, Window, WindowArgs, Game, Id, IdType, Renderable, Vertex, DrawMethod,
    DepthTestMethod, CullingMethod, Mat4, Vec3, DEG_TO_RAD, Transform, Named};

mod iso_data;
mod tile;
mod tile_map;
mod tile_coords;

pub use self::iso_data::{IsoData, IsoDataErr};
pub use self::tile::{Tile, TileErr};
pub use self::tile_map::{TileMap};
pub use self::tile_coords::{TileCoords};

pub static STONE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");

pub fn main() {
    let mut manager = init();
    let mut window = Window::new(WindowArgs::Borderless("Iso2".to_string())).unwrap();
    let resolution = window.get_resolution_vec2();
    let thread_count = 8;
    let mut game = Game::<IsoData>::new(thread_count, resolution);
    {
        let mut world = game.get_mut_world().unwrap();
        {
            let mut renderable = Renderable::new(&mut manager);
            renderable.set_vertices(vec!(
                Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
                Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0]),
                Vertex::new([1.0, 0.0, 1.0], [1.0, 1.0]),
                Vertex::new([0.0, 0.0, 1.0], [0.0, 1.0]),
            )).unwrap();
            renderable.set_indices(vec!(
                0, 1, 2,
                2, 3, 0,
            )).unwrap();
            renderable.set_texture(STONE_TEXTURE).unwrap();
            renderable.set_draw_method(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::CounterClockwise)).unwrap();
            renderable.set_perspective(Mat4::orthographic(0.0, 100.0, 90.0, 16.0 / 9.0)).unwrap();
            renderable.set_view(Mat4::x_rotation(-45.0 * DEG_TO_RAD) * Mat4::y_rotation((180.0 + 45.0) * DEG_TO_RAD) * Mat4::translation_from_vec3(Vec3::from([0.0, -2.0, -3.0]))).unwrap();
            renderable.set_model(Mat4::identity()).unwrap();

            for y in -5..6 {
                for x in -5..6 {
                    let id = Id::new(&mut manager, IdType::Entity);

                    let mut renderable = renderable.clone();
                    renderable.set_model(Mat4::identity()).unwrap();
                    //renderable.set_model_id(Id::new(&mut manager, IdType::Model));

                    let mut transform = Transform::new();
                    transform.set_position(Vec3::from([x as f32, 0.0, y as f32])).unwrap();

                    world.add_entity(
                        IsoData::new(id)
                        .with_renderable(
                            renderable
                        )
                        .with_transform(
                            transform
                        )
                    ).unwrap();
                }
            }
        }
    }
    println!("Starting Run Loop");
    game.run(&mut window, &mut manager).unwrap();
}
