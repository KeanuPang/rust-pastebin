use rocket::Data;
use std::env;
use std::fs::File;
use std::path::Path;

use crate::constants;
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
pub fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = paste_id::PasteId::new(constants::ID_LENGTH, constants::BASE62);

    let upload_size = paste.stream_to_file(Path::new(&id.file_path()))?;
    info!("//= upload {} bytes with id: {}", upload_size, id);

    let rocket_host = env::var("ROCKET_HOST").expect("invalid variable ROCKET_HOST");
    let rocket_port = env::var("ROCKET_PORT").expect("invalid variable ROCKET_PORT");
    let url = format!(
        "http://{host}:{port}/{id}\n",
        host = rocket_host,
        port = rocket_port,
        id = id
    );
    return Ok(url);
}

#[get("/<id>")]
pub fn retrieve(id: paste_id::PasteId) -> Option<File> {
    return File::open(id.file_path()).ok();
}
