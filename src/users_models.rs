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
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub pw_hash: String,
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

    pub fn insert(user: &NewUser, conn: &PgConnection) -> QueryResult<User> {
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
}
