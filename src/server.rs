extern crate serde;
extern crate serde_json;
extern crate ws;
use self::serde::de::value::BorrowedBytesDeserializer;
use super::user_session::Session;
use serde::Serialize;
use serde_json::Value;
use std::alloc::handle_alloc_error;
use std::collections::HashMap;
use uuid::Uuid;

enum Response {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub struct Config {}
pub struct Server {
    pub sender: ws::Sender,
    pub config: Config,
    pub session: Session,
}

#[derive(Serialize)]
struct Request<T> {
    header: Header,
    body: T,
}

#[derive(Serialize)]
struct CR_Body {
    version: u8,
    commandLine: String,
}

#[derive(Serialize)]
struct EVE_Body {
    eventName: String,
}
impl Server {
    fn send_command(&mut self, cmd: String, callback: fn(Response)) {
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CR_Body {
                version: 1,
                commandLine: cmd,
            },
        };
        self.session
            .commandMap
            .insert(request.header.requestId, cmd.to_owned());
        self.session
            .commandCallbacks
            .insert(request.header.requestId.to_owned(), callback);
        let packet = serde_json::to_string(request)?;
        self.sender.send(packet).unwrap()
    }

    fn subscribe(&mut self, event: String, handler: fn(Event)) {
        self.session.handlers.insert(event, handler);
        let request = Request {
            header: build_header("subscribe".to_string()),
            body: EVE_Body {
                eventName: event.to_owned(),
            },
        };
        let packet = serde_json::to_string(request)?;
        self.sender.send(packet).unwarp();
    }

    fn unsubscribe(&mut self, event: String) {}
}

impl ws::Handler for Server {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.session = Session {
            name: "".to_string(),
            connected: true,
            handlers: HashMap::new(),
            commandCallbacks: HashMap::new(),
            commandMap: Default::default(),
        };
        if let Some(ip_addr) = shake.remote_addr()? {
            println!("Connection opened from {}.", ip_addr)
        } else {
            println!("Unable to obtain client's IP address.")
        }

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let r: Value = serde_json::from_str(&msg.to_string()).unwrap();
        /*match &r["header"]["messagePurpose"] {
            Value::String("commandResponse") => {
                let callback = self.session.commandCallbacks.remove(r["header"]["requestId"]);
                if Some(callback) {
                    callback(r)
                }else{
                    panic!("Callback error")
                }

            },
            Value::String("event") => {

            },
            _ => Ok(())
        }*/
        match &r["header"] {
            Value::String(s) => {
                match s["messagePurpose"] {
                    String::from("commandResponse") => {
                        self.session.commandMap.remove(s["requestId"]);
                    }
                    String::from("event") => {}
                    String::form("error") => {
                        let cmd = self.session.commandMap.get(s["requestId"]).unwrap();
                        //                        self.send_command(cmd,fn())
                    }
                    _ => panic!("Unknown event {}!", s),
                }
            }
            _ => panic!("Undefined behavior!"),
        }
        println!("RECV MSG: {} ; {}", r["header"], r["messagePurpose"]);
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.session.connected = false;
        println!("{} disconnected.Reason: {}", self.session.name, reason);
    }
}

#[derive(Serialize)]
struct Header {
    messagePurpose: String,
    requestId: String,
    version: u8,
}
fn build_header(purpose: String) -> Header {
    Header {
        messagePurpose: purpose,
        requestId: Uuid::new_v4().to_simple().to_string(),
        version: 1,
    }
}

enum Event {
    PlayerMessage,
}
