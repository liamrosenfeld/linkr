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
mod crypto;
mod db;
mod models;
mod routes;
mod schema;

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
                routes::links::lookup,
                routes::pages::index,
                routes::pages::new_user,
                routes::pages::setup,
                routes::pages::login,
                routes::pages::manage_links,
                routes::pages::manage_users,
                routes::pages::manage_account,
                routes::static_files::favicon,
                routes::static_files::all_resources
            ],
        )
        .mount(
            "/api/links/",
            routes![
                routes::links::shorten,
                routes::links::all,
                routes::links::delete,
                routes::links::update
            ],
        )
        .mount(
            "/api/users/",
            routes![
                routes::users::new,
                routes::users::login,
                routes::users::logout,
                routes::users::delete_current,
                routes::users::delete_by_id,
                routes::users::update_permissions,
                routes::users::update_own_username,
                routes::users::update_username,
                routes::users::update_password
            ],
        )
        .register(catchers![
            catchers::not_found,
            catchers::internal_error,
            catchers::unauthorized,
            catchers::forbidden
        ])
}

fn main() {
    rocket().launch();
}
