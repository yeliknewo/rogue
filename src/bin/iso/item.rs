pub struct Item {
    item_type: ItemType,
}

impl Item {
    pub fn new(item_type: ItemType) -> Item {
        Item {
            item_type: item_type,
        }
    }
}

pub enum ItemType {
    Planks,
}
