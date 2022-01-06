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

use rocket_db_pools::{sqlx, Connection};

use crate::db::Db;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pw_hash: String,
    pub orig: bool,
    pub manage_links: bool,
    pub manage_users: bool,
    pub disabled: bool,
}

pub struct InsertableUser {
    pub username: String,
    pub pw_hash: String,
    pub orig: bool,
    pub manage_links: bool,
    pub manage_users: bool,
}

impl User {
    pub async fn get(id: i32, conn: &mut Connection<Db>) -> sqlx::Result<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&mut **conn)
            .await
    }

    pub async fn get_by_name(name: String, conn: &mut Connection<Db>) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            "SELECT DISTINCT * FROM users WHERE username = $1",
            name
        )
        .fetch_one(&mut **conn)
        .await
    }

    pub async fn all(conn: &mut Connection<Db>) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&mut **conn)
            .await
    }

    pub async fn insert(user: InsertableUser, conn: &mut Connection<Db>) -> sqlx::Result<i32> {
        let res = sqlx::query!(
            r"INSERT
            INTO users (username, pw_hash, orig, manage_links, manage_users)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id",
            user.username,
            user.pw_hash,
            user.orig,
            user.manage_links,
            user.manage_users
        )
        .fetch_one(&mut **conn)
        .await?;
        Ok(res.id)
    }

    pub async fn delete(id: i32, conn: &mut Connection<Db>) -> sqlx::Result<()> {
        // check that the user exists
        sqlx::query!("SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&mut **conn)
            .await?;

        // delete that user
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&mut **conn)
            .await?;

        Ok(())
    }

    pub async fn disable(id: i32, conn: &mut Connection<Db>) -> sqlx::Result<()> {
        sqlx::query!("UPDATE users SET disabled = true WHERE id = $1", id)
            .execute(&mut **conn)
            .await?;
        Ok(())
    }

    pub async fn enable(id: i32, conn: &mut Connection<Db>) -> sqlx::Result<()> {
        sqlx::query!("UPDATE users SET disabled = false WHERE id = $1", id)
            .execute(&mut **conn)
            .await?;
        Ok(())
    }

    pub async fn update_permissions(
        id: i32,
        manage_links: bool,
        manage_users: bool,
        conn: &mut Connection<Db>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r"
            UPDATE users
            SET manage_links = $1, manage_users = $2
            WHERE id = $3",
            manage_links,
            manage_users,
            id
        )
        .execute(&mut **conn)
        .await?;
        Ok(())
    }

    pub async fn update_username(
        id: i32,
        new_name: String,
        conn: &mut Connection<Db>,
    ) -> sqlx::Result<()> {
        sqlx::query!("UPDATE users SET username = $1 WHERE id = $2", new_name, id)
            .execute(&mut **conn)
            .await?;
        Ok(())
    }

    pub async fn update_password(
        id: i32,
        new_pw_hash: String,
        conn: &mut Connection<Db>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE users SET pw_hash = $1 WHERE id = $2",
            new_pw_hash,
            id
        )
        .execute(&mut **conn)
        .await?;
        Ok(())
    }

    pub async fn count(conn: &mut Connection<Db>) -> sqlx::Result<i64> {
        let res = sqlx::query!("SELECT COUNT(id) FROM users")
            .fetch_one(&mut **conn)
            .await?;
        Ok(res.count.unwrap_or(0))
    }
}
