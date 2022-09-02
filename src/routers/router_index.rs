use rocket::{Data, State};
use std::env;
use std::io::Read;

use crate::constants;
use crate::manager_rocksdb::RocksDBManager;
use crate::models::paste_id;

#[get("/")]
pub fn index() -> &'static str {
    "
    USAGE
        POST / 
        
            accepts raw data in the body of the request and responds with a URL of 
            a page containing the body's content

        GET /<id>

            retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
pub fn upload(paste: Data, db_manager: State<RocksDBManager>) -> Option<String> {
    let id = paste_id::PasteId::new(constants::ID_LENGTH, constants::BASE62);

    let mut buffer = String::new();
    if paste.open().read_to_string(&mut buffer).is_err() {
        error!("//= invalid file from data...");
        return None;
    }

    if db_manager.save(&id.id(), &buffer) {
        let rocket_host = env::var("ROCKET_HOST").expect("invalid variable ROCKET_HOST");
        let rocket_port = env::var("ROCKET_PORT").expect("invalid variable ROCKET_PORT");
        let url = format!(
            "http://{host}:{port}/{id}\n",
            host = rocket_host,
            port = rocket_port,
            id = id
        );

        return Some(url);
    } else {
        error!("//= save id `{}` failed", id);
        return None;
    }
}

#[get("/<id>")]
pub fn retrieve(id: paste_id::PasteId, db_manager: State<RocksDBManager>) -> Option<String> {
    return db_manager.find(&id.id());
}
