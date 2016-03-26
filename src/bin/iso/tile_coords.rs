pub struct TileCoords {
    x: i32,
    y: i32,
}

impl TileCoords {
    pub fn new(x: i32, y: i32) -> TileCoords {
        TileCoords {
            x: x,
            y: y,
        }
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
}
