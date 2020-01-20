use std::collections::HashMap;

struct PlayerMessage {
    message: String,
    sender: String,
    messageType: String,
    receiver: String,
}

pub struct Session {
    pub name: String,
    pub connected: bool,
    pub handlers: HashMap<String, fn(Event)>,
    pub commandCallbacks: HashMap<String, fn(String)>,
    pub commandMap: HashMap<String, String>,
}

pub enum Event {
    PlayerMessage(PlayerMessage),
}

pub enum Response {}
