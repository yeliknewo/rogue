use std::sync::{Arc, RwLock};
use dorp::{init, Game, Window, WindowArgs, UserData, World, IDManager, ID, IDType, EntityData, EntityDataGraphics};

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
        data.insert(ID::new(manager.clone(), IDType::Entity), Arc::new(RwLock::new(
            EntityData::new()
            .with_graphics(Arc::new(RwLock::new(EntityDataGraphics::new(manager.clone()))))
            .with_user(Arc::new(RwLock::new(IsoData::new())))
        )));
    }

    game.run(&mut window);
}

pub struct IsoData;

impl IsoData {
    pub fn new() -> IsoData {
        IsoData
    }
}

impl UserData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {

    }

    fn tick_mut(&mut self, manager: Arc<RwLock<IDManager>>, world: Arc<World<IsoData>>) {

    }

    fn render(&mut self, window: &mut Window, data_graphics: Arc<RwLock<EntityDataGraphics>>) {

    }
}
