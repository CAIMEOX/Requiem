use super::mctype::geometry::{Block,Position};

pub fn set_block(block: Block) -> String{
    format!("setblock {x} {y} {z} {b} {d}", x = block.position.x, y = block.position.y, z = block.position.z, b = block.name, d = block.data)
}

pub fn set_blocks(blocks: Vec<Block>) -> Vec<String> {
    let mut cmds = vec![];
    for b in blocks {
        cmds.push(set_block(b));
    }
    cmds
}

pub fn fill() -> String {
    "".to_string()
}

pub fn query_target(target: &str) -> String {
    format!("querytarget {}", target)
}

pub fn testfor(target: &str) -> String {
    format!("testfor {}", target)
}

pub fn tell_raw(target: &str, message: &str) -> String {
    format!("tellraw {} {{\"rawtext\":[{}]}}", target, {
        let lines = message.split("\n");
        let count = lines.clone().count();
        let mut msg = "".to_string();
        for (index, text) in lines.enumerate() {
            msg += format!("{{\"text\":\"{}\"}}{a}", text, a = {
                if index < count - 1 {
                    ","
                } else { "" }
            }).as_ref()
        }
        msg
    })
}