use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

use serde::Serialize;

#[derive(Queryable, Serialize, Insertable, FromForm)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub pw_hash: String,
}

impl User {
    pub fn get(username: &str, conn: &PgConnection) -> QueryResult<User> {
        all_users.find(username).get_result::<User>(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<User>> {
        all_users
            .order(users::username.desc())
            .get_results::<User>(conn)
    }

    pub fn insert(user: &User, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table).values(user).execute(conn)
    }

    pub fn delete(username: &str, conn: &PgConnection) -> QueryResult<usize> {
        if User::get(username, conn).is_err() {
            return Err(Error::NotFound);
        };
        diesel::delete(all_users.find(username)).execute(conn)
    }
}
