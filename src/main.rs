#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

mod catcher;
mod constants;
mod middlewares;
mod models;
mod routers;

use std::collections::HashMap;
use std::sync::Mutex;

use models::message;
use routers::{router_about, router_index};

fn main() {
    setup_env();
    setup_logger();
    setup_resources();

    get_instance().launch();
}

fn setup_env() {
    dotenvy::dotenv().ok();
}
fn setup_logger() {
    use env_logger::{Builder, Target};

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

fn setup_resources() {
    let upload_folder = constants::UPLOAD_FOLDER;
    std::fs::create_dir_all(upload_folder).expect("cant create uploading folder");
}

fn get_instance() -> rocket::Rocket {
    return rocket::ignite()
        .mount(
            "/",
            routes![
                router_index::index,
                router_index::upload,
                router_index::retrieve
            ],
        )
        .mount(
            "/message",
            routes![router_about::get, router_about::new, router_about::update],
        )
        .attach(middlewares::logger::Logger::default())
        .attach(middlewares::counter::Counter::default())
        .register(catchers![catcher::not_found])
        .manage(Mutex::new(HashMap::<message::ID, String>::new()));
}
