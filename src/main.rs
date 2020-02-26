extern crate meorslib;
extern crate ws;
use meorslib::mctype::config::{Config, Options};
use meorslib::mctype::geometry::{Block, Position};
use meorslib::server;
use meorslib::user_session::Session;
use meorslib::utils::now;

fn main() {
    let addr = "0.0.0.0:32768";
    println!("Server is running at {}", addr);

    ws::listen(addr, |out| server::Server {
        sender: out,
        session: Session {
            name: "".to_string(),
            config: Config {
                position: Position { x: 0, y: 0, z: 0 },
                block: Block {
                    position: Position { x: 0, y: 0, z: 0 },
                    name: "iron_block",
                    data: 0,
                },
            },
            options: Options { radius: 0 },
            connected: false,
            handlers: Default::default(),
            command_callbacks: Default::default(),
            command_map: Default::default(),
        },
    })
    .unwrap_or_else(handle_err);
}

fn handle_err(e: ws::Error) {
    println!("{}", e);
}
