use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

use serde::Serialize;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn get(id: i32, conn: &PgConnection) -> QueryResult<User> {
        all_users
            .find(id)
            .get_result::<User>(conn)
    }

    pub fn get_by_name(name: String, conn: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::username as n;

        all_users
            .filter(n.eq(name))
            .get_result::<User>(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<User>> {
        all_users
            .order(users::id.desc())
            .get_results::<User>(conn)
    }

    pub fn insert(user: NewUser, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        if User::get(id, conn).is_err() {
            return Err(Error::NotFound);
        };
        diesel::delete(all_users.find(id))
            .execute(conn)
    }
}
