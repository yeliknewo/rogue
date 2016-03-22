use dorp::{init, Window, WindowArgs, Game, Id, IdType, Renderable, Vertex, DrawMethod,
    DepthTestMethod, CullingMethod, Mat4, Vec3, DEG_TO_RAD};

mod iso_data;

pub use self::iso_data::{IsoData, IsoDataErr};

pub static STONE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");

pub fn main() {
    let mut manager = init();
    let mut window = Window::new(WindowArgs::Borderless("Iso2".to_string()));
    let resolution = window.get_resolution_vec2();
    let thread_count = 8;
    let mut game = Game::<IsoData, IsoDataErr>::new(thread_count, resolution);
    {
        let mut world = game.get_mut_world().unwrap();
        {
            let id = Id::new(&mut manager, IdType::Entity);
            let mut renderable = Renderable::new(&mut manager);
            renderable.set_vertices(vec!(
                Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]),
                Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0]),
                Vertex::new([1.0, 0.0, 1.0], [1.0, 1.0]),
                Vertex::new([0.0, 0.0, 1.0], [0.0, 1.0]),
            )).ok();
            renderable.set_indices(vec!(
                0, 1, 2,
                2, 3, 0,
            )).ok();
            renderable.set_texture(STONE_TEXTURE).ok();
            renderable.set_draw_method(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::CounterClockwise)).ok();
            renderable.set_perspective(Mat4::orthographic(0.0, 100.0, 90.0, 16.0 / 9.0)).ok();
            renderable.set_view(Mat4::x_rotation(-45.0 * DEG_TO_RAD) * Mat4::y_rotation((180.0 + 45.0) * DEG_TO_RAD) * Mat4::translation_from_vec3(Vec3::from([0.0, -2.0, -3.0]))).ok();
            renderable.set_model(Mat4::identity()).ok();
            world.add_entity(
                IsoData::new(id)
                .with_renderable(
                    renderable
                )
            ).ok();
        }
    }
    println!("Starting Run Loop");
    game.run(&mut window, &mut manager).ok();
}
