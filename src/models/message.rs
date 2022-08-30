use std::collections::HashMap;
use std::sync::Mutex;

pub type ID = usize;

pub type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Option<ID>,
    pub contents: String,
}
