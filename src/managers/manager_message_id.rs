use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::message;

pub struct MessageID {}

impl MessageID {
    pub fn generate() -> Mutex<HashMap<usize, String>> {
        return Mutex::new(HashMap::<message::ID, String>::new());
    }
}
