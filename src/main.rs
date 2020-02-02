extern crate meorslib;
extern crate ws;
use meorslib::utils::now;
use meorslib::server;
use meorslib::user_session::Session;
use ws::{listen, Handler, Handshake};
use colored::*;
use std::collections::HashMap;


fn main() {
    println!("{}",now("Server is running at localhost:8081".yellow().bold()));

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
