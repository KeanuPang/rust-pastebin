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
        let url = format!(
            "{host}:{port}/{id}\n",
            host = env::var("ROCKET_HOST").expect("invalid variable ROCKET_HOST"),
            port = env::var("ROCKET_PORT").expect("invalid variable ROCKET_PORT"),
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

// refer example: https://github.com/mkaczanowski/pastebin/blob/master/src/main.rs
// #[get("/<id>")]
// pub fn retrieve<'r>(
//     id: paste_id::PasteId,
//     db_manager: State<RocksDBManager>,
// ) -> Option<Response<'r>> {
//     if let Some(paste) = db_manager.find(&id.id()) {
//         return Some(
//             Response::build()
//                 .status(Status::Ok)
//                 .header(ContentType::Plain)
//                 .sized_body(Cursor::new(paste))
//                 .finalize(),
//         );
//     }

//     return None;
// }
