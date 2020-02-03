extern crate serde;
extern crate serde_json;
extern crate ws;
use super::user_session::{Callback, Session};
use super::mctype::config::{Config,Options};
use serde::Serialize;
use serde_json::{Error, Value};
use std::collections::HashMap;
use uuid::Uuid;
use std::thread;
use crate::mctype::geometry::Position;
use self::ws::CloseCode;



pub struct Server {
    pub sender: ws::Sender,
    pub session: Session,
}

#[derive(Serialize)]
struct Request<T> {
    header: Header,
    body: T,
}

#[derive(Serialize)]
struct CrBody {
    version: u8,
    commandLine: String,
}

#[derive(Serialize)]
struct EveBody {
    eventName: String,
}

trait SendCommand {
    fn send_command(&self, cmd: String, session: &mut Session, cb: Callback) -> Result<(), Error>;
    fn send_command_only(&self, cmd: String) -> Result<(), Error>;
    fn tell_raw(&self, text: String);
}

impl SendCommand for ws::Sender {
    fn send_command(
        &self,
        cmd: String,
        session: &mut Session,
        callback: Callback,
    ) -> Result<(), Error> {
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CrBody {
                version: 1,
                commandLine: cmd.clone(),
            },
        };
        session
            .commandMap
            .insert(request.header.requestId.clone(), cmd);
        session
            .commandCallbacks
            .insert(request.header.requestId.clone(), callback);
        let packet = serde_json::to_string(&request)?;
        self.send(packet).unwrap();
        Ok(())
    }

    fn send_command_only(&self, cmd: String) -> Result<(), Error> {
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CrBody {
                version: 1,
                commandLine: cmd.clone(),
            },
        };
        let packet = serde_json::to_string(&request)?;
        self.send(packet).unwrap();
        Ok(())
    }

    fn tell_raw(&self, text: String) {
        self.send_command_only(format!("tellraw @s {{\"rawtext\":[{{\"text\":\"{t}\"}}]}}",t = text));
    }
}
impl Server {
    fn send_command(&mut self, cmd: String, callback: Callback) -> Result<(), Error> {
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CrBody {
                version: 1,
                commandLine: cmd.clone(),
            },
        };
        self.session
            .commandMap
            .insert(request.header.requestId.clone(), cmd);
        self.session
            .commandCallbacks
            .insert(request.header.requestId.clone(), callback);
        let packet = serde_json::to_string(&request)?;
        self.sender.send(packet).unwrap();
        Ok(())
    }

    fn send_command_only(&self, cmd: String) -> Result<(), Error>{
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CrBody {
                version: 1,
                commandLine: cmd.clone(),
            },
        };
        let packet = serde_json::to_string(&request)?;
        self.sender.send(packet).unwrap();
        Ok(())
    }

    fn tell_raw(&self, text: String){
        self.send_command_only(format!("tellraw @s {{\"rawtext\":[{{\"text\":\"{t}\"}}]}}",t = text));
    }

    fn resend_command(&self, cmd: String, id: String) -> Result<(), Error> {
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CrBody {
                version: 1,
                commandLine: cmd,
            },
        };
        let packet = serde_json::to_string(&request)?;
        self.sender.send(packet).unwrap();
        Ok(())
    }

    fn subscribe(&mut self, event: String, handler: Callback) -> Result<(), Error> {
        self.session.handlers.insert(event.clone(), handler);
        let request = Request {
            header: build_header("subscribe".to_string()),
            body: EveBody { eventName: event },
        };
        let packet = serde_json::to_string(&request)?;
        self.sender.send(packet);
        Ok(())
    }

    fn unsubscribe(&mut self, event: &str) {
        self.session.handlers.remove(event);
    }
}

impl ws::Handler for Server {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.session = Session {
            name: "".to_string(),
            config: Config{
                position: Position{
                    x: 1,
                    y: 0,
                    z: 1
                }
            },
            options: Options{
                radius: 5
            },
            connected: true,
            handlers: HashMap::new(),
            commandCallbacks: HashMap::new(),
            commandMap: Default::default(),
        };
        fn recv_pm(sender: &ws::Sender, session: &mut Session, response: &Value) {
            match &response["body"]["properties"]["message"] {
                Value::String(s) if s == "whoami" => {
                    println!("WHOAMI");
                    fn f(sender: &Server, session: &Session, v: &Value) {}
                    //sender.send_command("say hi".to_string(),f);
                }
                _ => println!("WTF: {}", response["body"]["properties"]["message"]),
            }
            //            println!("[{}]{}",response["body"]["properties"]["sender"],response["body"]["properties"]["message"])
        }
        self.subscribe("PlayerMessage".to_string(), recv_pm);
        fn recvTestfor(sender: &ws::Sender, session: &mut Session, v: &Value) {
            println!("Testfor: {}", v);
            session.name = v["body"]["properties"]["sender"].to_string();
        }

        self.send_command("testfor @s".to_string(), recvTestfor);
        fn setPosition(sender: &ws::Sender, session: &mut Session, v: &Value){
            let pos = &v["body"]["details"]["position"].as_object().unwrap();
//            if let (&Value::Number(x),&Value::Number(y),&Value::Number(z)) = (&pos["x"], &pos["y"], &pos["z"]) {
            if let (x, y, z) = (pos["x"].as_i64(), pos["y"].as_i64(), pos["z"].as_i64()){
                session.config.position = Position {
                    x: x.unwrap() as i32,
                    y: y.unwrap() as i32,
                    z: z.unwrap() as i32
                }
            };
            sender.tell_raw(format!("Position got: {:?}", session.config.position));
        }
        self.send_command("querytarget @s".to_string(), setPosition);
        if let Some(ip_addr) = shake.remote_addr()? {
            println!("Connection opened from {}.", ip_addr)
        } else {
            println!("Unable to obtain client's IP address.")
        }

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let r: Value = serde_json::from_str(&msg.to_string()).unwrap();
        match &r["header"]["messagePurpose"] {
            Value::String(s) => match &s[..] {
                "commandResponse" => {
                    let cmd = self
                        .session
                        .commandMap
                        .remove(r["header"]["requestId"].as_str().unwrap())
                        .unwrap();
                    let f = self
                        .session
                        .commandCallbacks
                        .get(r["header"]["requestId"].as_str().unwrap())
                        .unwrap();
                    f(&self.sender, &mut self.session, &r);
                }
                "event" => {
                    let f = self
                        .session
                        .handlers
                        .get(r["body"]["eventName"].as_str().unwrap())
                        .unwrap();
                    f(&self.sender, &mut self.session, &r);
                }
                "error" => {
                    let cmd = self
                        .session
                        .commandMap
                        .get(r["header"]["requestId"].as_str().unwrap())
                        .unwrap();
                    self.resend_command(String::from(cmd), r["header"]["requestId"].to_string());
                }
                _ => panic!("Unknown event {}!", s),
            },
            _ => panic!("Undefined behavior!"),
        }
        println!("REC MSG: {} ; {}", r["header"], r["body"]);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.session.connected = false;
        println!("Connection closing due to ({:?}) {}", code, reason);
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("{:?}", err);
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
