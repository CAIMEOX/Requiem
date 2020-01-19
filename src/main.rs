extern crate ws;
extern crate env_logger;

use ws::{listen,Handler,Handshake};

struct Server {
    
}

fn main() {
    env_logger::init();
    println!("Server is running at ::18880");
    if let Err(e) = listen("localhost:18880", |out|{
        move |msg| {
            println!("Server got: {}", msg);
            out.send(format!("Repeat: {}",msg))
        }
    }){
        println!("Failed to create websocket server");
    }
}
