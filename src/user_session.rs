use super::server::Server;
use serde_json::Value;
use std::collections::HashMap;
use ws::Sender;
use super::mctype::config::{Config,Options};

pub struct PlayerMessage {
    pub message: String,
    pub sender: String,
    pub messageType: String,
    pub receiver: String,
}

pub type Callback = fn(&ws::Sender, &mut Session, &Value);
pub struct Session {
    pub name: String,
    pub config: Config,
    pub options: Options,
    pub connected: bool,
    pub handlers: HashMap<String, Callback>,
    pub commandCallbacks: HashMap<String, Callback>,
    pub commandMap: HashMap<String, String>,
}

impl Session {
    pub fn setName(&mut self, name: String) {
        self.name = name;
    }
}
pub enum Event {
    PlayerMessage(PlayerMessage),
}

pub enum Response {}
