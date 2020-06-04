use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::schema::links;
use crate::schema::links::dsl::links as all_links;

use serde::Serialize;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub long: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "links"]
pub struct NewLink {
    pub short: String,
    pub long: String,
}

impl Link {
    pub fn get(id: i32, conn: &PgConnection) -> QueryResult<Link> {
        all_links
            .find(id)
            .get_result::<Link>(conn)
    }

    pub fn get_by_short(short: String, conn: &PgConnection) -> QueryResult<Link> {
        use crate::schema::links::dsl::short as s;

        all_links
            .filter(s.eq(short))
            .get_result::<Link>(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Link>> {
        all_links
            .order(links::id.desc())
            .get_results::<Link>(conn)
    }

    pub fn update_by_id(id: i32, new_long: String, conn: &PgConnection) -> QueryResult<Link> {
        use crate::schema::links::dsl::long;

        diesel::update(all_links.find(id))
            .set( long.eq(new_long))
            .get_result::<Link>(conn)
    }

    pub fn insert(link: NewLink, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(links::table)
            .values(&link)
            .execute(conn)
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        if Link::get(id, conn).is_err() {
            return Err(Error::NotFound);
        };
        diesel::delete(all_links.find(id))
            .execute(conn)
    }
}
