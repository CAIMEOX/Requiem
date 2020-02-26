extern crate serde;
extern crate serde_json;
extern crate ws;
use self::ws::CloseCode;
use super::builder::generate;
use super::commander;
use super::mctype::config::{Config, Options};
use super::parse::parse;
use super::user_session::{Callback, Session};
use super::utils::add_pos;
use crate::commander::{set_blocks, tell_raw};
use crate::mctype::geometry::{Block, Position};
use serde::Serialize;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::thread;
use uuid::Uuid;

fn build_header(purpose: String) -> Header {
    Header {
        messagePurpose: purpose,
        requestId: Uuid::new_v4().to_simple().to_string(),
        version: 1,
    }
}

pub struct Server<'a> {
    pub sender: ws::Sender,
    pub session: Session<'a>,
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
            .command_map
            .insert(request.header.requestId.clone(), cmd);
        session
            .command_callbacks
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
        self.send_command_only(format!(
            "tellraw @s {{\"rawtext\":[{{\"text\":\"{t}\"}}]}}",
            t = text
        ));
    }
}
impl Server<'_> {
    fn on_chat_meesage(&mut self, message: &str) {
        let args = parse(message);
        if let Ok(a) = args.1 {
            let mut blocks = generate(args.0, a, &self.session.config, &self.sender);
            match blocks {
                Ok(mut b) => {
                    add_pos(&mut b, self.session.config.position.clone());
                    let cmds = set_blocks(b);
                }
                Err(e) => {
                    self.send_command_only(tell_raw("@w",&e.to_string()));
                },
            }
        } else if let Err(e) = args.1 {
            println!("Cannot parse cfg: {}", e);
        }
    }

    fn send_command_queue(&mut self, cmds: Vec<String>) {
        for c in cmds {
            self.send_command_only(c);
        }
    }

    fn send_command(&mut self, cmd: String, callback: Callback) -> Result<(), Error> {
        let request = Request {
            header: build_header("commandRequest".to_string()),
            body: CrBody {
                version: 1,
                commandLine: cmd.clone(),
            },
        };
        self.session
            .command_map
            .insert(request.header.requestId.clone(), cmd);
        self.session
            .command_callbacks
            .insert(request.header.requestId.clone(), callback);
        let packet = serde_json::to_string(&request)?;
        self.sender.send(packet).unwrap();
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
        self.sender.send(packet).unwrap();
        Ok(())
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

impl ws::Handler for Server<'_> {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.send_command_only(tell_raw("@s", "RustBuilder connected!!"));
        //Subscrible PlayerMessage
        self.subscribe("PlayerMessage".to_string(), |_, _, _|{});
        fn recv_testfor(_sender: &ws::Sender, session: &mut Session, v: &Value) {
            println!("Testfor: {}", v);
            session.name = v["body"]["properties"]["sender"].to_string();
        }
        self.send_command("testfor @s".to_string(), recv_testfor);
        fn set_position(sender: &ws::Sender, session: &mut Session, v: &Value) {
            let pos = &v["body"]["details"]["position"].as_object();
            if let Some(pos) = pos {
                if let (x, y, z) = (pos["x"].as_i64(), pos["y"].as_i64(), pos["z"].as_i64()) {
                    session.config.position = Position {
                        x: x.unwrap() as i32,
                        y: y.unwrap() as i32,
                        z: z.unwrap() as i32,
                    }
                };
                sender.tell_raw(format!("Position got: {:?}", session.config.position));
            } else {
                println!("SetPosition Error: {:?}", &v["body"]["details"])
            }
            //            if let (&Value::Number(x),&Value::Number(y),&Value::Number(z)) = (&pos["x"], &pos["y"], &pos["z"]) {
        }
        self.send_command("querytarget @s".to_string(), set_position);
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
                        .command_map
                        .remove(r["header"]["requestId"].as_str().unwrap())
                        .unwrap();
                    let f = self
                        .session
                        .command_callbacks
                        .get(r["header"]["requestId"].as_str().unwrap())
                        .unwrap();
                    f(&self.sender, &mut self.session, &r);
                }
                "event" => {
                    if r["body"]["eventName"].as_str().unwrap() == "PlayerMessage" {
                        if let Some(msg) = r["body"]["properties"]["message"].as_str() {
                            self.on_chat_meesage(msg)
                        }
                    }
                }
                "error" => {
                    //Resend
                    let cmd = self
                        .session
                        .command_map
                        .get(r["header"]["requestId"].as_str().unwrap())
                        .unwrap();
                    self.resend_command(String::from(cmd), r["header"]["requestId"].to_string());
                }
                _ => panic!("Unknown event {}!", s),
            },
            _ => panic!("Undefined behavior!"),
        }
        println!("onMessage: {} ; {}", r["header"], r["body"]);
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
