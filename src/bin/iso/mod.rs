use std::sync::{Arc, RwLock};
use dorp::{init, Game, Window, WindowArgs, EntityData, World, IDManager, ID, IDType, Mat4, Vec2, Vec3, Transform, Renderable, DrawMethod, DepthTestMethod, CullingMethod, Vertex};

static STONE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");
static WOOD_TEXTURE: &'static [u8] = include_bytes!("../../../assets/wood.png");

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
            IsoData::new()
            .with_renderable(
                Renderable::new(manager.clone())
                .with_vertices(vec!(
                    Vertex::from(Vec2::from([0.0, 0.0])),
                    Vertex::from(Vec2::from([1.0, 0.0])),
                    Vertex::from(Vec2::from([1.0, 1.0])),
                    Vertex::from(Vec2::from([0.0, 1.0])),
                ))
                .with_indices(vec!(
                    0, 1, 2,
                    2, 3, 0,
                ))
                .with_texture(STONE_TEXTURE)
                .with_draw_method(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::Clockwise))
                .with_perspective(Mat4::identity(), Mat4::identity())
                .with_view(Mat4::identity(), Mat4::identity())
                .with_model(Mat4::identity(), Mat4::identity())
            )
            .with_transform(
                Transform::new()
                .with_position(Vec3::zero())
                .with_rotation(Vec3::zero())
                .with_scalation(Vec3::one())
            )
        )));
    }

    game.run(&mut window);
}

pub struct IsoData {
    transform: Option<Box<Arc<RwLock<Transform>>>>,
    renderable: Option<Box<Arc<RwLock<Renderable>>>>,
}

impl IsoData {
    pub fn new() -> IsoData {
        IsoData {
            transform: None,
            renderable: None,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> IsoData {
        self.transform = Some(Box::new(Arc::new(RwLock::new(transform))));
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> IsoData {
        self.renderable = Some(Box::new(Arc::new(RwLock::new(renderable))));
        self
    }
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {

    }

    fn tick_mut(&mut self, manager: Arc<RwLock<IDManager>>, world: Arc<World<IsoData>>) {

    }

    fn render(&mut self, window: &mut Window, world: Arc<World<IsoData>>) {
        match self.renderable.clone() {
            Some(renderable) => {
                renderable.write().expect("Unable to Write Renderable in Render in IsoData").render(window, world);
            },
            None => (),
        }
    }

    fn get_graphics_data(&self) -> Option<Box<Arc<RwLock<Renderable>>>> {
        self.renderable.clone()
    }
}
