use rocksdb::{Options, DB};
use std::path::Path;
use std::sync::Arc;

use crate::constants;
// use crate::models::paste_id::PasteId;

// pub trait KVStore {
//     fn init() -> Self;
//     fn save(&self, k: &str, v: &str) -> bool;
//     fn find(&self, k: &str) -> Option<String>;
//     fn delete(&self, k: &str) -> bool;
// }

pub struct RocksDBManager {
    db: Arc<DB>,
}

impl RocksDBManager {
    pub fn init() -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);

        return RocksDBManager {
            db: Arc::new(DB::open(&options, Path::new(&constants::ROCKSDB_FOLDER)).unwrap()),
        };
    }

    pub fn save(&self, k: &str, v: &str) -> bool {
        return self.db.put(k.as_bytes(), v.as_bytes()).is_ok();
    }

    pub fn find(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v).unwrap();
                info!("//= get value from key {}: {}", k, result);
                return Some(result);
            }
            Ok(None) => {
                warn!("//= Finding {} returns None", k);
                return None;
            }
            Err(e) => {
                error!("//= Error retrieving value for {}: {}", k, e);
                return None;
            }
        }
    }

    // pub fn delete(&self, k: &str) -> bool {
    //     return self.db.delete(k.as_bytes()).is_ok();
    // }
}
