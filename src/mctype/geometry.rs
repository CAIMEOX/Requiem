impl isBlock {
    fn is_block(&self,name: &str) -> bool {
        if self.name == name {
            true
        }else{
            false
        }
    }
}

pub struct block {
    pub position:position,
    pub name: str,
    pub data: u8
}

pub struct position {
    x:i32,
    y:i32,
    z:i32
}

