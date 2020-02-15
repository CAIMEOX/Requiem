impl Block<'_> {
    fn is_block(&self, name: &str) -> bool {
        if self.name == name {
            true
        } else {
            false
        }
    }
}
#[derive(Debug)]
pub struct Block<'a> {
    pub position: Position,
    pub name: &'a str,
    pub data: u8,
}

#[derive(Debug,Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
