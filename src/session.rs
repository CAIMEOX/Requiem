use std::collections::HashMap;


struct PlayerMessage {
    message:String,
    sender:String,
    messageType:String,
    receiver:String
}

mod struct Session {
    pub name:String,
    pub connected:bool,
    pub handlers:HashMap<String,fn()>,
    pub commandCallbacks:HashMap<String,fn()>
}