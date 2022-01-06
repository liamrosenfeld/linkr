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

use dotenv::dotenv;
use rocket::figment::{
    util::map,
    value::{Map, Value},
    Figment,
};
use rocket_db_pools::sqlx;
use std::env;

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::PgPool);

pub fn db_configurator() -> Figment {
    // create get database url
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let db: Map<_, Value> = map! {
        "url" => database_url.into()
    };

    rocket::Config::figment().merge(("databases", map!["db" => db]))
}
