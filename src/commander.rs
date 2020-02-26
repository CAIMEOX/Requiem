use super::mctype::geometry::{Block, Module, Position};

pub fn set_block(block: Block) -> String {
    format!(
        "setblock {x} {y} {z} {b} {d}",
        x = block.position.x,
        y = block.position.y,
        z = block.position.z,
        b = block.name,
        d = block.data
    )
}

pub fn set_blocks(blocks: Vec<Block>) -> Vec<String> {
    let mut cmds = vec![];
    for b in blocks {
        cmds.push(set_block(b));
    }
    cmds
}

pub fn fill(module: Module) -> String {
    let Position {
        x: bx,
        y: by,
        z: bz,
    } = module.begin;
    let Position {
        x: ex,
        y: ey,
        z: ez,
    } = module.end;
    format!(
        "fill {} {} {} {} {} {} {} {}",
        bx, by, bz, ex, ey, ez, module.name, module.data
    )
}

pub fn query_target(target: &str) -> String {
    format!("querytarget {}", target)
}

pub fn test_for(target: &str) -> String {
    format!("testfor {}", target)
}

pub fn tell_raw(target: &str, message: &str) -> String {
    format!("tellraw {} {{\"rawtext\":[{}]}}", target, {
        let lines = message.split("\n");
        let count = lines.clone().count();
        let mut msg = "".to_string();
        for (index, text) in lines.enumerate() {
            msg += format!(
                "{{\"text\":\"{}\"}}{a}",
                text,
                a = {
                    if index < count - 1 {
                        ","
                    } else {
                        ""
                    }
                }
            )
            .as_ref()
        }
        msg
    })
}

pub fn enable_encryption(key: &[u8], salt: &str) -> String {
    format!("enableencryption {:?} {}", key, salt)
}

pub fn close_chat() -> String {
    "closechat".to_string()
}

pub fn get_top_solid_block(pos: Position) -> String {
    let Position { x, y, z } = pos;
    format!("gettopsolidblock {} {} {}", x, y, z)
}

pub fn get_local_player_name() -> String {
    "getlocalplayername".to_string()
}

pub fn test_for_block(pos: Position, name: &str, data: u8) -> String {
    let Position { x, y, z } = pos;
    format!("testforblock {} {} {} {} {}", x, y, z, name, data)
}

pub fn clone(begin: Position, end: Position, destination: Position, mode: &str) -> String {
    let Position {
        x: bx,
        y: by,
        z: bz,
    } = begin;
    let Position {
        x: ex,
        y: ey,
        z: ez,
    } = end;
    let Position {
        x: dx,
        y: dy,
        z: dz,
    } = destination;
    format!(
        "clone {} {} {} {} {} {} {} {} {} {}",
        bx, by, bz, ex, ey, ez, dx, dy, dz, mode
    )
}
