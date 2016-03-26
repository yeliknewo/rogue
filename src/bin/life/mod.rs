use dorp::{
    init, Window, WindowArgs, Game, Id, IdType
};

mod life_data;
mod scene;
mod cell;
mod tile_map;

pub use self::life_data::{LifeData};
pub use self::scene::{Scene, SceneErr};
pub use self::cell::{Cell, CellErr};
pub use self::tile_map::{TileMap};

pub static LIFE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");

pub static TILE_MAP_NAME: &'static str = "TileMap";

pub fn main() {
    let mut manager = init();
    let mut window = Window::new(WindowArgs::Windowed(640, 480, "Life".to_string())).unwrap();
    let resolution = window.get_resolution_vec2();
    let thread_count = 8;
    let mut game = Game::<LifeData>::new(thread_count, resolution);
    {
        let mut world = game.get_mut_world().unwrap();
        let id = Id::new(&mut manager, IdType::Entity);
        let scene = Scene::new();
        world.add_entity(
            LifeData::new(id)
            .with_scene(
                scene
            )
        ).unwrap();
    }
    game.run(&mut window, &mut manager).unwrap();
}
