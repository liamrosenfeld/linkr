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
use rocket_db_pools::{sqlx, Connection};

use crate::db::Db;

#[derive(sqlx::FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Link {
    pub short: String,
    pub long: String,
    pub notes: String,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
}

impl Link {
    pub async fn get(short: String, conn: &mut Connection<Db>) -> sqlx::Result<Link> {
        sqlx::query_as!(Link, "SELECT * FROM links WHERE short = $1", short)
            .fetch_one(&mut **conn)
            .await
    }

    pub async fn all(conn: &mut Connection<Db>) -> sqlx::Result<Vec<Link>> {
        sqlx::query_as!(Link, "SELECT * FROM links")
            .fetch_all(&mut **conn)
            .await
    }

    pub async fn all_for_user(user_id: i32, conn: &mut Connection<Db>) -> sqlx::Result<Vec<Link>> {
        sqlx::query_as!(Link, "SELECT * FROM links WHERE created_by = $1", user_id)
            .fetch_all(&mut **conn)
            .await
    }

    pub async fn update(
        short: String,
        new_long: String,
        conn: &mut Connection<Db>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE links SET long = $1 WHERE short = $2",
            new_long,
            short
        )
        .execute(&mut **conn)
        .await?;
        Ok(())
    }

    pub async fn insert(link: Link, conn: &mut Connection<Db>) -> sqlx::Result<()> {
        sqlx::query!(
            r"INSERT INTO links (short, long, notes, created_at, created_by)
            VALUES ($1, $2, $3, $4, $5)",
            link.short,
            link.long,
            link.notes,
            link.created_at,
            link.created_by
        )
        .execute(&mut **conn)
        .await?;
        Ok(())
    }

    pub async fn delete(short: String, conn: &mut Connection<Db>) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM links WHERE short = $1", short)
            .execute(&mut **conn)
            .await?;
        Ok(())
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
