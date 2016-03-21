use std::sync::{Arc};
use dorp::{Transform, Renderable, Named, Id, EntityData, IdManager, World, Window};

use iso::{Being, Item, Tile, TileMap, TileCoordinates, TILE_MAP_NAME};

pub struct IsoData {
    id: Id,
    transform: Option<Arc<Transform>>,
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    tile_map: Option<Arc<TileMap>>,
    tile: Option<Arc<Tile>>,
    item: Option<Arc<Item>>,
    being: Option<Arc<Being>>,
    tile_coordinates: Option<Arc<TileCoordinates>>,
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
            tile_coordinates: None,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> IsoData {
        self.transform = Some(Arc::new(transform));
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> IsoData {
        self.renderable = Some(Arc::new(renderable));
        self
    }

    pub fn with_named(mut self, named: Named) -> IsoData {
        self.named = Some(Arc::new(named));
        self
    }

    pub fn with_tile_map(mut self, tile_map: TileMap) -> IsoData {
        self.tile_map = Some(Arc::new(tile_map));
        self
    }

    pub fn with_tile(mut self, tile: Tile) -> IsoData {
        self.tile = Some(Arc::new(tile));
        self
    }

    pub fn with_item(mut self, item: Item) -> IsoData {
        self.item = Some(Arc::new(item));
        self
    }

    pub fn with_being(mut self, being: Being) -> IsoData {
        self.being = Some(Arc::new(being));
        self
    }

    pub fn with_tile_coordinates(mut self, coords: TileCoordinates) -> IsoData {
        self.tile_coordinates = Some(Arc::new(coords));
        self
    }

    pub fn with_tile_coordinates_arc(mut self, coords: Arc<TileCoordinates>) -> IsoData {
        self.tile_coordinates = Some(coords);
        self
    }

    pub fn get_tile_map(&self) -> Option<Arc<TileMap>> {
        self.tile_map.clone()
    }

    pub fn get_tile(&self) -> Option<Arc<Tile>> {
        self.tile.clone()
    }

    pub fn get_item(&self) -> Option<Arc<Item>> {
        self.item.clone()
    }

    pub fn get_being(&self) -> Option<Arc<Being>> {
        self.being.clone()
    }

    pub fn get_tile_coordinates(&self) -> Option<Arc<TileCoordinates>> {
        self.tile_coordinates.clone()
    }
}

impl EntityData<IsoData> for IsoData {
    fn tick(&self, delta_time: Arc<f64>, world: Arc<World<IsoData>>) {
        match self.being.clone() {
            Some(being) => {
                match self.tile_coordinates.clone() {
                    Some(tile_coordinates) => {
                        being.tick(tile_coordinates, world.clone());
                    },
                    None => panic!("Being has no TileCoordinates in Tick in IsoData"),
                }
            },
            None => (),
        }
    }

    fn tick_mut(&mut self) {
        match self.tile.clone() {
            Some(tile) => {
                match self.transform.clone() {
                    Some(transform) => {
                        Arc::get_mut(&mut tile).unwrap().tick_mut(transform);
                    },
                    None => panic!("Tile has no Transform in Tick Mut in IsoData"),
                }
            },
            None => (),
        }
        match self.transform.clone() {
            Some(transform) => {
                Arc::get_mut(&mut transform).unwrap().tick_mut();
            },
            None => (),
        }
        match self.tile_coordinates.clone() {
            Some(tile_coordinates) => {
                Arc::get_mut(&mut tile_coordinates).unwrap().tick_mut(self.get_id(), TILE_MAP_NAME);
            },
            None => (),
        }
    }

    fn render(&mut self, window: &mut Window) {
        match self.renderable.clone() {
            Some(renderable) => {
                Arc::get_mut(&mut renderable).unwrap().render(window);
            },
            None => (),
        }
    }

    fn get_transform(&self) -> Option<Arc<Transform>> {
        self.transform.clone()
    }

    fn get_renderable(&self) -> Option<Arc<Renderable>> {
        self.renderable.clone()
    }

    fn get_named(&self) -> Option<Arc<Named>> {
        self.named.clone()
    }

    fn get_id(&self) -> Id {
        self.id
    }
}
