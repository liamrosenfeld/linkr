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
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

mod auth;
mod catchers;
mod crypto;
mod db;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> Rocket<Build> {
    // setup rocket
    rocket::custom(db::db_configurator())
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_db_migrations))
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
        .register(
            "/",
            catchers![
                catchers::unauthorized,
                catchers::forbidden,
                catchers::not_found,
                catchers::internal_error,
                catchers::service_unavailable
            ],
        )
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!();

    let conn = db::DbConn::get_one(&rocket)
        .await
        .expect("Could not connect to database");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("Failed to run database migrations");

    rocket
}
