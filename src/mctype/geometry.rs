impl Block {
    fn is_block(&self, name: &str) -> bool {
        if self.name == name {
            true
        } else {
            false
        }
    }
}

pub struct Block {
    pub position: Position,
    pub name: String,
    pub data: u8,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

