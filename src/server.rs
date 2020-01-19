extern crate ws;
extern crate serde_json;
use serde_json::{Result, Value};
use std::collections::HashMap;
enum Response {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

struct Config {

}
struct Server {
    sender: Sender,
    config: Config,
    session: Session
}


impl ws::Handler for Server {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.session = Session{
            connected:true,
            handlers:HashMap::new(),
            commandCallbacks:HashMap::new()
        };
        if let Some(ip_addr) = shake.remote_addr()? {
            println!("Connection opened from {}.", ip_addr)
        } else {
            println!("Unable to obtain client's IP address.")
        }
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<()>{
        let r: Response = serde_json::from_str(msg)?;
        match r["header"]["messagePurpose"] {
            "commandResponse" => {
                let callback = self.session.commandCallbacks.remove(r["header"]["requestId"]);
                callback(r)
            },
            "event" => {

            },
            _ => ()
        }
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.session.connected = false;
        println!("{} disconnected.Reason: {}", self.session.name, reason);
    }
}