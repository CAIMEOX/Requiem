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

pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}
