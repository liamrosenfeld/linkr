#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use rocket_contrib::templates::Template;
use std::env;

mod auth;
mod catchers;
mod db;
mod links_api;
mod links_models;
mod pages;
mod schema;
mod static_files;
mod users_api;
mod users_crypto;
mod users_models;

fn rocket() -> rocket::Rocket {
    // create db pool from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);

    // setup rocket
    rocket::ignite()
        .manage(pool)
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                links_api::lookup,
                pages::index,
                pages::signup,
                pages::login,
                pages::manage_links,
                static_files::favicon,
                static_files::all_resources,
            ],
        )
        .mount(
            "/api/links/",
            routes![
                links_api::shorten,
                links_api::all,
                links_api::delete,
                links_api::update
            ],
        )
        .mount(
            "/api/users/",
            routes![
                users_api::new,
                users_api::login,
                users_api::logout,
                users_api::delete
            ],
        )
        .register(catchers![
            catchers::not_found,
            catchers::internal_error,
            catchers::unauthorized
        ])
}

fn main() {
    rocket().launch();
}
