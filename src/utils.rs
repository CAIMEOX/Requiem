extern crate chrono;
use chrono::Local;
use crate::mctype::geometry::{Block, Position};

pub fn now<T: std::fmt::Display>(t: T) -> String {
    let date = Local::now();
    format!("{} {}", date.format("[%H:%M:%S]"), t)
}

pub fn add_pos(blocks: &mut Vec<Block>, pos: Position){
    for b in blocks {
        b.position = Position {
            x: b.position.x + pos.x,
            y: b.position.y + pos.y,
            z: b.position.z + pos.z
        }
    }
}

pub fn str2i32(v: Vec<&str>) -> Vec<i32> {
    v.into_iter()
        .map(|x|{
            x.parse::<i32>().unwrap_or(0)
        })
        .collect()
}