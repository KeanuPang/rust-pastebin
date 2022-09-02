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
mod managers;
mod middlewares;
mod models;
mod routers;

use managers::{manager_message_id, manager_rocksdb};
use middlewares::{middleware_counter, middleware_logger};
use rocket::Rocket;
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

    if std::env::var("RUST_LOG").is_err() {
        builder.filter_level(log::LevelFilter::Info);
    }

    builder.init();
}

fn setup_resources() {
    let upload_folder = constants::UPLOAD_FOLDER;
    std::fs::create_dir_all(upload_folder).expect("cant create uploading folder");
}

fn setup_middlewares(rocket: Rocket) -> Rocket {
    return rocket
        .attach(middleware_logger::Logger::default())
        .attach(middleware_counter::Counter::default());
}

fn setup_managers(rocket: Rocket) -> Rocket {
    return rocket
        .manage(manager_rocksdb::RocksDBManager::init())
        .manage(manager_message_id::MessageID::generate());
}

fn get_instance() -> Rocket {
    let mut rocket = rocket::ignite().register(catchers![catcher::not_found]);

    rocket = setup_middlewares(rocket);
    rocket = setup_managers(rocket);

    rocket = rocket
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
        );

    return rocket;
}
