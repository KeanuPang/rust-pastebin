#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Data;
use std::env;
use std::fs::File;
use std::path::Path;

mod constants;
mod models;
use models::paste_id::PasteId;

#[cfg(test)]
mod tests;

fn main() {
    setup_logger();

    rocket_instance().launch();
}

fn setup_logger() {
    use env_logger::{Builder, Target};

    dotenvy::dotenv().ok();
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

fn rocket_instance() -> rocket::Rocket {
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
    let id = PasteId::new(constants::IDLENGTH, constants::BASE62);

    let filename = format!("upload/{id}", id = id);
    paste.stream_to_file(Path::new(&filename))?;

    let rocket_host = env::var("ROCKET_HOST").unwrap_or("".to_string());
    let rocket_port = env::var("ROCKET_PORT").unwrap_or("".to_string());
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
    let filename = format!("upload/{id}", id = id);
    return File::open(&filename).ok();
}
