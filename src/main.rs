// Copyright (C) 2020 Liam Rosenfeld
//
// This file is part of Linkr (https://github.com/liamrosenfeld/linkr).
//
// Linkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Linkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Linkr. If not, see <http://www.gnu.org/licenses/>.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::Rocket;
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
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                routes::pages::link,
                routes::pages::index,
                routes::pages::new_user,
                routes::pages::setup,
                routes::pages::login,
                routes::pages::manage_links,
                routes::pages::manage_users,
                routes::pages::manage_account,
                routes::static_files::all_resources
            ],
        )
        .mount(
            "/api/links/",
            routes![
                routes::links::shorten,
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
                routes::users::disable_current,
                routes::users::delete_by_id,
                routes::users::disable_by_id,
                routes::users::enable_by_id,
                routes::users::update_permissions,
                routes::users::update_own_username,
                routes::users::update_username,
                routes::users::update_password
            ],
        )
        .register(catchers![
            catchers::unauthorized,
            catchers::forbidden,
            catchers::not_found,
            catchers::internal_error,
            catchers::service_unavailable
        ])
}

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows migrations to be run during
// run-time instead of before compile-time. That is the only way for the `DATABASE_URL`
// from docker to be available and running.
embed_migrations!();

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = match rocket
        .state::<db::Pool>()
        .expect("This needs to be after manage(state: pool)")
        .get()
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Could not connect to database: {:?}", e);
            return Err(rocket);
        }
    };

    match embedded_migrations::run(&conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            eprintln!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn main() {
    rocket().launch();
}
