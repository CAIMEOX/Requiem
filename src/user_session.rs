use std::collections::HashMap;
use serde_json::Value;
use super::server::Server;
use ws::Sender;
pub struct PlayerMessage {
    pub message: String,
    pub sender: String,
    pub messageType: String,
    pub receiver: String,
}
pub type Callback = fn(&ws::Sender, &mut Session,&Value);
pub struct Session {
    pub name: String,
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
