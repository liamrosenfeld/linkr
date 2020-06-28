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

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

use serde::Serialize;

#[derive(Queryable, Serialize, FromForm)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pw_hash: String,
    pub orig: bool,
    pub manage_links: bool,
    pub manage_users: bool,
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
    pub fn get(id: i32, conn: &PgConnection) -> QueryResult<User> {
        all_users.find(id).get_result::<User>(conn)
    }

    pub fn get_by_name(name: &str, conn: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::username;

        all_users
            .distinct()
            .filter(username.eq(name))
            .get_result::<User>(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<User>> {
        all_users.order(users::id.desc()).get_results::<User>(conn)
    }

    pub fn insert(user: &InsertableUser, conn: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(conn)
    }

    pub fn delete(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        if User::get(id, conn).is_err() {
            return Err(Error::NotFound);
        };
        diesel::delete(all_users.find(id)).execute(conn)
    }

    pub fn update_permissions(
        id: i32,
        manage_links: bool,
        manage_users: bool,
        conn: &PgConnection,
    ) -> QueryResult<usize> {
        diesel::update(all_users.find(id))
            .set(&PermissionForm {
                manage_links,
                manage_users,
            })
            .execute(conn)
    }

    pub fn update_username(id: i32, new_name: &str, conn: &PgConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::username;

        diesel::update(all_users.find(id))
            .set(username.eq(new_name))
            .execute(conn)
    }

    pub fn update_password(id: i32, new_pw_hash: &str, conn: &PgConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::pw_hash;

        diesel::update(all_users.find(id))
            .set(pw_hash.eq(new_pw_hash))
            .execute(conn)
    }

    pub fn count(conn: &PgConnection) -> QueryResult<i64> {
        all_users.select(diesel::dsl::count_star()).first(conn)
    }
}
