use std::sync::{Arc, RwLock};
use dorp::{init, Game, Window, WindowArgs, EntityData, World, IDManager, ID, IDType, Mat4, Vec3,
     DEG_TO_RAD, Transform, Renderable, DrawMethod, DepthTestMethod, CullingMethod, Vertex,
     Named
};

static STONE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/brick.png");
static WOOD_TEXTURE: &'static [u8] = include_bytes!("../../../assets/wood.png");

mod tile;
mod item;

use self::tile::{Tile};
use self::item::{Item};

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
            .with_view(Mat4::x_rotation(-45.0 * DEG_TO_RAD) * Mat4::y_rotation(45.0 * DEG_TO_RAD) * Mat4::translation_from_vec3(Vec3::from([0.0, 2.0, 1.0])))
            .with_model(Mat4::identity());
        let item_graphics = tile_graphics.clone()
            .with_texture(WOOD_TEXTURE);
        for z in 0..10 {
            for x in 0..10 {
                let id = ID::new(manager.clone(), IDType::Entity);
                data.insert(id, Arc::new(RwLock::new(
                    IsoData::new(id)
                    .with_renderable(
                        tile_graphics.clone()
                        .with_model_id(ID::new(manager.clone(), IDType::Model))
                        .with_model(Mat4::identity())
                    )
                    .with_transform(
                        Transform::new()
                        .with_position(Vec3::from([x as f32, 0.0, z as f32]))
                        .with_rotation(Vec3::from([0.0, 0.0, 0.0]))
                        .with_scalation(Vec3::one())
                    )
                    .with_tile(
                        Tile::new()
                    )
                )));
            }
        }
        let id = ID::new(manager.clone(), IDType::Entity);
        data.insert(id, Arc::new(RwLock::new(
            IsoData::new(id)
            .with_renderable(
                item_graphics.clone()
                .with_model_id(ID::new(manager.clone(), IDType::Model))
                .with_model(Mat4::identity())
            )
            .with_transform(
                Transform::new()
                .with_scalation(Vec3::from([0.5, 0.5, 0.5]))
            )
            .with_named(
                Named::new("Item")
            )
            .with_item(
                Item::new()
            )
        )));
    }

    game.run(&mut window);
}

pub struct IsoData {
    id: ID,
    transform: Option<Box<Arc<RwLock<Transform>>>>,
    renderable: Option<Box<Arc<RwLock<Renderable>>>>,
    named: Option<Box<Arc<RwLock<Named>>>>,
    tile: Option<Box<Arc<RwLock<Tile>>>>,
    item: Option<Box<Arc<RwLock<Item>>>>,
}

impl IsoData {
    pub fn new(id: ID) -> IsoData {
        IsoData {
            id: id,
            transform: None,
            renderable: None,
            named: None,
            tile: None,
            item: None,
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

    pub fn with_named(mut self, named: Named) -> IsoData {
        self.named = Some(Box::new(Arc::new(RwLock::new(named))));
        self
    }

    pub fn with_tile(mut self, tile: Tile) -> IsoData {
        self.tile = Some(Box::new(Arc::new(RwLock::new(tile))));
        self
    }

    pub fn with_item(mut self, item: Item) -> IsoData {
        self.item = Some(Box::new(Arc::new(RwLock::new(item))));
        self
    }

    pub fn get_tile(&self) -> Option<Box<Arc<RwLock<Tile>>>> {
        self.tile.clone()
    }

    pub fn get_item(&self) -> Option<Box<Arc<RwLock<Item>>>> {
        self.item.clone()
    }
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {
        
    }

    fn tick_mut(&mut self, manager: Arc<RwLock<IDManager>>, world: Arc<World<IsoData>>) {
        match self.named.clone() {
            Some(named) => {
                named.write().expect("Unable to Write Named in Tick Mut in IsoData").tick_mut(self.get_id(), world.clone());
            },
            None => (),
        }
        match self.renderable.clone() {
            Some(renderable) => {
                match self.transform.clone() {
                    Some(transform) => {
                        transform.write().expect("Unable to Write Transform in Tick Mut in IsoData").tick_mut(renderable);
                    },
                    None => (),
                }
            },
            None => (),
        }
    }

    fn render(&mut self, window: &mut Window, world: Arc<World<IsoData>>) {
        match self.renderable.clone() {
            Some(renderable) => {
                renderable.write().expect("Unable to Write Renderable in Render in IsoData").render(window, world.clone());
            },
            None => (),
        }
    }

    fn get_transform(&self) -> Option<Box<Arc<RwLock<Transform>>>> {
        self.transform.clone()
    }

    fn get_renderable(&self) -> Option<Box<Arc<RwLock<Renderable>>>> {
        self.renderable.clone()
    }

    fn get_named(&self) -> Option<Box<Arc<RwLock<Named>>>> {
        self.named.clone()
    }

    fn get_id(&self) -> ID {
        self.id
    }
}
