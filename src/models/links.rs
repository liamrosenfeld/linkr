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

use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;

use crate::schema::links;
use crate::schema::links::dsl::links as all_links;

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "links"]
pub struct Link {
    pub short: String,
    pub long: String,
    pub notes: String,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
}

impl Link {
    pub fn get(short: &str, conn: &PgConnection) -> QueryResult<Link> {
        all_links.find(short).get_result::<Link>(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Link>> {
        all_links
            .order(links::created_at.desc())
            .get_results::<Link>(conn)
    }

    pub fn all_for_user(user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Link>> {
        use crate::schema::links::dsl::created_by;

        all_links
            .filter(created_by.eq(user_id))
            .order(links::created_at.desc())
            .get_results::<Link>(conn)
    }

    pub fn update(short: &str, new_long: &str, conn: &PgConnection) -> QueryResult<Link> {
        use crate::schema::links::dsl::long;

        diesel::update(all_links.find(short))
            .set(long.eq(new_long))
            .get_result::<Link>(conn)
    }

    pub fn insert(link: Link, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(links::table)
            .values(&link)
            .execute(conn)
    }

    pub fn delete(short: &str, conn: &PgConnection) -> QueryResult<usize> {
        if Link::get(short, conn).is_err() {
            return Err(Error::NotFound);
        };
        diesel::delete(all_links.find(short)).execute(conn)
    }
}

/* ------------------------------- formatters ------------------------------- */

mod date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Serializer};

    const FORMAT: &'static str = "%D";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}
