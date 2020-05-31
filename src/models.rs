use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::links;
use crate::schema::links::dsl::links as all_links;

#[derive(Queryable, Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub orig: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "links"]
pub struct NewLink {
    pub short: String,
    pub orig: String,
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

    pub fn update_by_id(id: i32, conn: &PgConnection, link: NewLink) -> bool {
        use crate::schema::links::dsl::{short as s, orig as o};

        let NewLink {
            short,
            orig
        } = link;

        diesel::update(all_links.find(id))
            .set((s.eq(short), o.eq(orig)))
            .get_result::<Link>(conn)
            .is_ok()
    }

    pub fn insert(link: NewLink, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(links::table)
            .values(&link)
            .execute(conn)
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Link::get(id, conn).is_err() {
            return false;
        };
        diesel::delete(all_links.find(id))
            .execute(conn)
            .is_ok()
    }
}
