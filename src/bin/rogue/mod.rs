use dorp::{
    IdManager, WindowBuilder, Game, Vec2, Id, IdType
};

mod rogue_data;
mod scene;

pub use self::rogue_data::{RogueData};
pub use self::scene::{Scene, SceneErr};

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
            let scene = Scene::new();
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
