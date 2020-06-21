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
    pub manage_links: bool,
    pub manage_users: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub pw_hash: String,
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
}
