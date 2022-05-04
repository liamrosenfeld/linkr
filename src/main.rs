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

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_db_pools;

use rocket::fairing;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

mod auth;
mod crypto;
mod db;
mod models;
mod routes;

#[launch]
fn rocket() -> Rocket<Build> {
    // setup rocket
    rocket::custom(db::db_configurator())
        .attach(db::Db::init())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            run_db_migrations,
        ))
        .mount(
            "/",
            routes![
                routes::client::link,
                routes::client::index,
                routes::client::new_user,
                routes::client::setup,
                routes::client::login,
                routes::client::manage_links,
                routes::client::manage_users,
                routes::client::manage_account,
                routes::client::resources,
                routes::client::svelte_gen
            ],
        )
        .mount(
            "/api/links/",
            routes![
                routes::links::new,
                routes::links::new_bulk,
                routes::links::get_all,
                routes::links::get_for_user,
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
                routes::users::get_all,
                routes::users::get_current,
                routes::users::delete,
                routes::users::disable,
                routes::users::enable,
                routes::users::update_permissions,
                routes::users::update_username,
                routes::users::update_password
            ],
        )
        .register(
            "/",
            catchers![
                routes::client::unauthorized,
                routes::client::forbidden,
                routes::client::not_found,
                routes::client::internal_error,
                routes::client::service_unavailable
            ],
        )
}

async fn run_db_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match db::Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
