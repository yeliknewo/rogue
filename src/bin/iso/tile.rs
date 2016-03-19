use dorp::{ID};

pub struct Tile {
    items: Vec<ID>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            items: vec!(),
        }
    }

    pub fn with_item(mut self, entity_id: ID) -> Tile {
        self.add_item(entity_id);
        self
    }

    pub fn add_item(&mut self, entity_id: ID) {
        self.items.push(entity_id);
    }

    pub fn rem_item(&mut self, entity_id: ID) {
        for i in 0..self.items.len() {
            if *self.items.get(i).expect("Unable to Get Item in Rem Item in Tile") == entity_id {
                self.items.remove(i);
                break;
            }
        }
    }
}
