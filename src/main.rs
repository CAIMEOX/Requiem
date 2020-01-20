extern crate env_logger;
extern crate meorslib;
extern crate ws;

use meorslib::server;
use meorslib::user_session::Session;
use ws::{listen, Handler, Handshake};

fn main() {
    env_logger::init();
    println!("Server is running at ::8081");
    /*if let Err(e) = listen("localhost:18880", |out|{
        move |msg| {
            println!("Server got: {}", msg);
            out.send(format!("Repeat: {}",msg))
        }
    }){
        println!("Failed to create websocket server");
    }*/

    ws::listen("localhost:8081", |out| server::Server {
        sender: out,
        config: server::Config {},
        session: Session {
            name: "".to_string(),
            connected: false,
            handlers: Default::default(),
            commandCallbacks: Default::default(),
            commandMap: Default::default()
        },
    })
    .unwrap();
}
