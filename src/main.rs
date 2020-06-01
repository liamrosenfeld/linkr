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
mod models;
mod db;
mod static_files;
mod routes;
mod catchers;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![routes::lookup, static_files::all, static_files::index])
        .mount("/api", routes![routes::shorten, routes::all, routes::delete])
        .register(catchers![catchers::not_found, catchers::internal_error])

}

fn main() {
    rocket().launch();
}
