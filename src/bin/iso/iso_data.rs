use std::sync::{Arc, RwLock};
use dorp::{Transform, Renderable, Named, Id, EntityData, IdManager, World, Window};

use iso::{Being, Item, Tile, TileMap};

pub struct IsoData {
    id: Id,
    transform: Option<Arc<RwLock<Transform>>>,
    renderable: Option<Arc<RwLock<Renderable>>>,
    named: Option<Arc<RwLock<Named>>>,
    tile_map: Option<Arc<RwLock<TileMap>>>,
    tile: Option<Arc<RwLock<Tile>>>,
    item: Option<Arc<RwLock<Item>>>,
    being: Option<Arc<RwLock<Being>>>,
}

impl IsoData {
    pub fn new(id: Id) -> IsoData {
        IsoData {
            id: id,
            transform: None,
            renderable: None,
            named: None,
            tile_map: None,
            tile: None,
            item: None,
            being: None,
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

    pub fn with_tile_map(mut self, tile_map: TileMap) -> IsoData {
        self.tile_map = Some(Arc::new(RwLock::new(tile_map)));
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

    pub fn with_being(mut self, being: Being) -> IsoData {
        self.being = Some(Arc::new(RwLock::new(being)));
        self
    }

    pub fn get_tile_map(&self) -> Option<Arc<RwLock<TileMap>>> {
        self.tile_map.clone()
    }

    pub fn get_tile(&self) -> Option<Arc<RwLock<Tile>>> {
        self.tile.clone()
    }

    pub fn get_item(&self) -> Option<Arc<RwLock<Item>>> {
        self.item.clone()
    }

    pub fn get_being(&self) -> Option<Arc<RwLock<Being>>> {
        self.being.clone()
    }
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {

    }

    fn tick_mut(&mut self, manager: Arc<RwLock<IdManager>>, world: Arc<World<IsoData>>) {
        match self.tile.clone() {
            Some(tile) => {
                match self.transform.clone() {
                    Some(transform) => {
                        tile.write().expect("Unable to Write Tile in Tick in IsoData").tick_mut(self.get_id(), transform, world.clone());
                    },
                    None => panic!("Tile has no Transform in Tick Mut in IsoData"),
                }
            },
            None => (),
        }
        match self.named.clone() {
            Some(named) => {
                named.write().expect("Unable to Write Named in Tick Mut in IsoData").tick_mut(self.get_id(), world.clone());
            },
            None => (),
        }
        match self.transform.clone() {
            Some(transform) => {
                transform.write().expect("Unable to Write Transform in Tick Mut in IsoData").tick_mut();
            },
            None => (),
        }
    }

    fn render_sync(&mut self, world: Arc<World<IsoData>>) {
        match self.renderable.clone() {
            Some(renderable) => {
                renderable.write().expect("Unable to Write Renderable in Render Sync in IsoData").render_sync(world);
                match self.transform.clone() {
                    Some(transform) => {
                        transform.write().expect("Unable to Write Transform in Render Sync in IsoData").render_sync(renderable);
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