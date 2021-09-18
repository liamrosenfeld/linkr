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

use rocket::serde::Serialize;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;

use crate::db::DbConn;
use crate::schema::users;

#[derive(Queryable, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pw_hash: String,
    pub orig: bool,
    pub manage_links: bool,
    pub manage_users: bool,
    pub disabled: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub pw_hash: String,
    pub orig: bool,
    pub manage_links: bool,
    pub manage_users: bool,
}

#[derive(AsChangeset)]
#[table_name = "users"]
struct PermissionForm {
    manage_links: bool,
    manage_users: bool,
}

impl User {
    pub async fn get(id: i32, db: &DbConn) -> QueryResult<User> {
        db.run(move |conn| users::table.find(id).get_result(conn))
            .await
    }

    pub async fn get_by_name(name: String, db: &DbConn) -> QueryResult<User> {
        use crate::schema::users::dsl::username;
        db.run(move |conn| {
            users::table
                .distinct()
                .filter(username.eq(name))
                .get_result(conn)
        })
        .await
    }

    pub async fn all(db: &DbConn) -> QueryResult<Vec<User>> {
        db.run(move |conn| users::table.order(users::id.desc()).load(conn))
            .await
    }

    pub async fn insert(user: InsertableUser, db: &DbConn) -> QueryResult<User> {
        db.run(move |conn| {
            diesel::insert_into(users::table)
                .values(user)
                .get_result::<User>(conn)
        })
        .await
    }

    pub async fn delete(id: i32, db: &DbConn) -> QueryResult<usize> {
        User::get(id, db).await?;
        db.run(move |conn| diesel::delete(users::table.find(id)).execute(conn))
            .await
    }

    pub async fn disable(id: i32, db: &DbConn) -> QueryResult<usize> {
        use crate::schema::users::dsl::disabled;
        db.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(disabled.eq(true))
                .execute(conn)
        })
        .await
    }

    pub async fn enable(id: i32, db: &DbConn) -> QueryResult<usize> {
        use crate::schema::users::dsl::disabled;
        db.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(disabled.eq(false))
                .execute(conn)
        })
        .await
    }

    pub async fn update_permissions(
        id: i32,
        manage_links: bool,
        manage_users: bool,
        db: &DbConn,
    ) -> QueryResult<usize> {
        db.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(&PermissionForm {
                    manage_links,
                    manage_users,
                })
                .execute(conn)
        })
        .await
    }

    pub async fn update_username(id: i32, new_name: String, db: &DbConn) -> QueryResult<usize> {
        use crate::schema::users::dsl::username;
        db.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(username.eq(new_name))
                .execute(conn)
        })
        .await
    }

    pub async fn update_password(id: i32, new_pw_hash: String, db: &DbConn) -> QueryResult<usize> {
        use crate::schema::users::dsl::pw_hash;
        db.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(pw_hash.eq(new_pw_hash))
                .execute(conn)
        })
        .await
    }

    pub async fn count(db: &DbConn) -> QueryResult<i64> {
        db.run(move |conn| users::table.select(diesel::dsl::count_star()).first(conn))
            .await
    }
}
