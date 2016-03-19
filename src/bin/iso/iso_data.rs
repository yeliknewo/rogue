use std::sync::{Arc, RwLock};
use dorp::{Transform, Renderable, Named, Id, EntityData, IdManager, World, Window};

use iso::item::{Item};
use iso::tile::{Tile};

pub struct IsoData {
    id: Id,
    transform: Option<Arc<RwLock<Transform>>>,
    renderable: Option<Arc<RwLock<Renderable>>>,
    named: Option<Arc<RwLock<Named>>>,
    tile: Option<Arc<RwLock<Tile>>>,
    item: Option<Arc<RwLock<Item>>>,
}

impl IsoData {
    pub fn new(id: Id) -> IsoData {
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
        self.transform = Some(Arc::new(RwLock::new(transform)));
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> IsoData {
        self.renderable = Some(Arc::new(RwLock::new(renderable)));
        self
    }

    pub fn with_named(mut self, named: Named) -> IsoData {
        self.named = Some(Arc::new(RwLock::new(named)));
        self
    }

    pub fn with_tile(mut self, tile: Tile) -> IsoData {
        self.tile = Some(Arc::new(RwLock::new(tile)));
        self
    }

    pub fn with_item(mut self, item: Item) -> IsoData {
        self.item = Some(Arc::new(RwLock::new(item)));
        self
    }

    pub fn get_tile(&self) -> Option<Arc<RwLock<Tile>>> {
        self.tile.clone()
    }

    pub fn get_item(&self) -> Option<Arc<RwLock<Item>>> {
        self.item.clone()
    }
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {
        match world.get_entity_by_name("Item") {
            Some(entity) => {
                let entity = entity.read().expect("Unable to Read Entity in Tick in IsoData");
                let transform = entity
                .get_transform().expect("Unable to Get Transform");
                let position = transform
                .read().expect("Unable to Read Transform in Tick in IsoData").get_position();
                println!("{}", position);
            },
            None => (),
        }
        match self.tile.clone() {
            Some(tile) => {
                tile.write().expect("Unable to Write Tile in Tick in IsoData").tick(self.get_transform(), world.clone());
            },
            None => (),
        }
    }

    fn tick_mut(&mut self, manager: Arc<RwLock<IdManager>>, world: Arc<World<IsoData>>) {
        match self.named.clone() {
            Some(named) => {
                named.write().expect("Unable to Write Named in Tick Mut in IsoData").tick_mut(self.get_id(), world.clone());
            },
            None => (),
        }
        match self.transform.clone() {
            Some(transform) => {
                match self.renderable.clone() {
                    Some(renderable) => {
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

    fn get_transform(&self) -> Option<Arc<RwLock<Transform>>> {
        self.transform.clone()
    }

    fn get_renderable(&self) -> Option<Arc<RwLock<Renderable>>> {
        self.renderable.clone()
    }

    fn get_named(&self) -> Option<Arc<RwLock<Named>>> {
        self.named.clone()
    }

    fn get_id(&self) -> Id {
        self.id
    }
}
