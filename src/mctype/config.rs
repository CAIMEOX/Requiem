use super::geometry::{Block, Position};
//Building Config
pub struct Options {
    pub radius: u32,
}

//User's profile
pub struct Config<'a> {
    pub position: Position,
    pub block: Block<'a>,
}
