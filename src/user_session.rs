use super::mctype::config::{Config, Options};
use super::server::Server;
use serde_json::Value;
use std::collections::HashMap;
use ws::Sender;

pub struct PlayerMessage {
    pub message: String,
    pub sender: String,
    pub message_type: String,
    pub receiver: String,
}

pub type Callback = fn(&ws::Sender, &mut Session, &Value);
pub struct Session<'a> {
    pub name: String,
    pub config: Config<'a>,
    pub options: Options,
    pub connected: bool,
    pub handlers: HashMap<String, Callback>,
    pub command_callbacks: HashMap<String, Callback>,
    pub command_map: HashMap<String, String>,
}

impl Session<'_> {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
pub enum Event {
    PlayerMessage(PlayerMessage),
}

pub enum Response {}
