use std::sync::{Arc};

use dorp::{Id, World};

use iso::{IsoData};

struct Changes {
    x: Option<i32>,
    y: Option<i32>,
    dirty: bool,
}

impl Changes {
    pub fn new() -> Changes {
        Changes {
            x: None,
            y: None,
            dirty: false,
        }
    }
}

pub struct TileCoordinates {
    x: i32,
    y: i32,
    tile_id: Id,
    //changes: Arc<RwLock<Changes>>,
}

impl TileCoordinates {
    pub fn new_no_move(x: i32, y: i32, tile_id: Id) -> TileCoordinates {
        TileCoordinates {
            x: x,
            y: y,
            tile_id: tile_id,
            //changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn new(x: i32, y: i32, my_id: Id, tile_id: Id, tile_map_name: &'static str, world: Arc<World<IsoData>>) -> TileCoordinates {
        TileCoordinates::finish_move(my_id, tile_map_name, (x, y), world);
        TileCoordinates {
            x: x,
            y: y,
            tile_id: tile_id,
            //changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn new_find(x: i32, y: i32, my_id: Id, tile_map_name: &'static str, world: Arc<World<IsoData>>) -> TileCoordinates {
        let tile_id = {
            let tile_map = world.get_entity_by_name(tile_map_name).expect("Unable to Get Entity By Name in New Find in Tile Coordinates").get_tile_map().expect("Unable to Get Tile Map in New Find in Tile Coordinates");
            let id = tile_map.get_at(x, y).expect("Unable to Get At in New Find in Tile Coordinates");
            id
        };
        TileCoordinates::new(x, y, my_id, tile_id, tile_map_name, world)
    }

    pub fn tick_mut(&mut self, id: Id, tile_map_name: &'static str, world: Arc<World<IsoData>>) {
        // if self.changes.read().expect("Unable to Read Changes in Tick Mut in Tile Coordinates").dirty {
        //     let new_x = self.changes.read().expect("Unable to Read Changes in Tick Mut in Tile Coordinates").x;
        //     match new_x {
        //         Some(new_x) => {
        //             self.changes.write().expect("Unable to Write Changes in Tick Mut in Tile Coordinates").x = None;
        //             let new_y = self.changes.read().expect("Unable to Read Changes in Tick Mut in Tile Coordinates").y;
        //             match new_y {
        //                 Some(new_y) => {
        //                     self.changes.write().expect("Unable to Write Changes in Tick Mut in Tile Coordinates").y = None;
        //                     self.move_to(id, tile_map_name, (new_x, new_y), world);
        //                 },
        //                 None => {
        //                     let y = self.y;
        //                     self.move_to(id, tile_map_name, (new_x, y), world);
        //                 },
        //             }
        //         },
        //         None => {
        //             let new_y = self.changes.read().expect("Unable to Read Changes in Tick Mut in Tile Coordinates").y;
        //             match new_y {
        //                 Some(new_y) => {
        //                     self.changes.write().expect("Unable to Write Changes in Tick Mut in Tile Coordinates").y = None;
        //                     let x = self.x;
        //                     self.move_to(id, tile_map_name, (x, new_y), world);
        //                 },
        //                 None => (),
        //             }
        //         },
        //     }
        // }
    }

    fn move_to(&mut self, id: Id, tile_map_name: &'static str, coords: (i32, i32), world: Arc<World<IsoData>>) {
        {
            let entity = world.get_entity_by_id(self.tile_id).expect("Unable to Get Entity By Id in Move To in Tile Coordinates");
            let tile = entity.get_tile().expect("Unable to Get Tile in Move To in Tile Coordinates");
            tile.remove_from_tile(id);
        }
        TileCoordinates::finish_move(id, tile_map_name, coords, world);
    }

    fn finish_move(id: Id, tile_map_name: &'static str, coords: (i32, i32), world: Arc<World<IsoData>>) {
        let tile_map_entity = world.get_entity_by_name(tile_map_name).expect("Unable to get Entity By Name in Move To in Tile Coordinates");
        let tile_map = tile_map_entity.get_tile_map().expect("Unable to Get Tile Map in Move To in Tile Coordinates");
        let tile_id = tile_map.get_at_xy(coords).expect("Unable to Get At XY in Move To in Tile Coordinates");
        let tile_entity = world.get_entity_by_id(tile_id).expect("Unable to Get Entity By Id in Move To in Tile Coordinates");
        let tile = tile_entity.get_tile().expect("Unable to Get Tile in Move To in Tile Coordinates");
        tile.add_to_tile(id);
    }

    pub fn get_coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_tile(&self) -> Id {
        self.tile_id
    }

    pub fn set_coords(&self, x: Option<i32>, y: Option<i32>) {
        // let mut changes = self.changes.write().expect("Unable to Write Changes in Set Coords in Tile Coordinates");
        // changes.x = x;
        // changes.y = y;
        // changes.dirty = true;
    }
}
