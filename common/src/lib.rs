use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum WSMessageType{
    NewMessage,
    UserList,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub msg: String,
    pub user: String,
    pub created_at: NaiveDateTime,
    // pub id: usize
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WSMessage{
    pub message_type:WSMessageType,
    pub message:Option<Message>,
    pub users:Option<Vec<String>>
}