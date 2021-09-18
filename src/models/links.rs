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
use rocket::serde::Serialize;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;

use crate::db::DbConn;
use crate::models::users::User;
use crate::schema::links;

#[derive(Queryable, Insertable, Serialize, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "created_by")]
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
    pub async fn get(short: String, db: &DbConn) -> QueryResult<Link> {
        db.run(move |conn| links::table.find(short).get_result::<Link>(conn))
            .await
    }

    pub async fn all(db: &DbConn) -> QueryResult<Vec<Link>> {
        db.run(move |conn| {
            links::table
                .order(links::created_at.desc())
                .get_results::<Link>(conn)
        })
        .await
    }

    pub async fn all_for_user(user_id: i32, db: &DbConn) -> QueryResult<Vec<Link>> {
        use crate::schema::links::dsl::created_by;
        db.run(move |conn| {
            links::table
                .filter(created_by.eq(user_id))
                .order(links::created_at.desc())
                .get_results::<Link>(conn)
        })
        .await
    }

    pub async fn update(short: String, new_long: String, db: &DbConn) -> QueryResult<Link> {
        use crate::schema::links::dsl::long;
        db.run(move |conn| {
            diesel::update(links::table.find(short))
                .set(long.eq(new_long))
                .get_result::<Link>(conn)
        })
        .await
    }

    pub async fn insert(link: Link, db: &DbConn) -> QueryResult<usize> {
        db.run(move |conn| {
            diesel::insert_into(links::table)
                .values(&link)
                .execute(conn)
        })
        .await
    }

    pub async fn delete(short: String, db: &DbConn) -> QueryResult<usize> {
        // if Link::get(short, conn).is_err() {
        //     return Err(Error::NotFound);
        // };
        Link::get(short.clone(), db).await?;
        db.run(move |conn| diesel::delete(links::table.find(short)).execute(conn))
            .await
    }
}

/* ------------------------------- formatters ------------------------------- */

mod date_format {
    use chrono::{DateTime, Utc};
    use rocket::serde::Serializer;

    const FORMAT: &'static str = "%D";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}
