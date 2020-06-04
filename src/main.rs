#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate rocket_contrib;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod schema;
mod db;
mod static_files;
mod links_models;
mod users_models;
mod users_crypto;
mod links_api;
mod users_api;
mod catchers;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![links_api::lookup, static_files::all, static_files::index, static_files::login])
        .mount("/api/links/", routes![links_api::shorten, links_api::all, links_api::delete, links_api::update])
        .mount("/api/users/", routes![users_api::new, users_api::delete])
        .register(catchers![catchers::not_found, catchers::internal_error])
}

fn main() {
    rocket().launch();
}
