#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

use rocket::Data;
use std::env;
use std::fs::File;
use std::path::Path;

mod constants;
mod models;
use models::paste_id::PasteId;

fn main() {
    setup_logger();
    setup_resources();

    get_instance().launch();
}

fn setup_logger() {
    use env_logger::{Builder, Target};

    dotenvy::dotenv().ok();
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

fn setup_resources() {
    let upload_folder = constants::UPLOAD_FOLDER;
    std::fs::create_dir_all(upload_folder).unwrap();
}

fn get_instance() -> rocket::Rocket {
    return rocket::ignite().mount("/", routes![index, upload, retrieve]);
}

#[get("/")]
fn index() -> &'static str {
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
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = PasteId::new(constants::ID_LENGTH, constants::BASE62);

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
fn retrieve(id: PasteId) -> Option<File> {
    return File::open(id.file_path()).ok();
}
