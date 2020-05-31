use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::Status;

use rocket_contrib::json::Json;
use serde_json::Value;

use diesel::result::Error;
use diesel::result::DatabaseErrorKind;

use crate::db::Conn as DbConn;
use crate::models::{Link, NewLink};

/* --------------------------------- lookup --------------------------------- */

#[get("/<short>")]
pub fn lookup(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get_by_short(short, &conn) {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(Error::NotFound) => Err(Status::NotFound),
        _ => Err(Status::InternalServerError)
    }
}

/* ----------------------------------- api ---------------------------------- */

#[post("/shorten", data = "<link_form>")]
pub fn shorten(conn: DbConn, link_form: Form<NewLink>) -> Status {
    let link = link_form.into_inner();
    match Link::insert(link, &conn) {
        Ok(_) => Status::Ok,
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Status::Conflict,
        Err(_) => Status::InternalServerError
    }
}

#[get("/all")]
pub fn all(conn: DbConn) -> Result<Json<Value>, Status> {
    match Link::all(&conn) {
        Ok(links) => Ok(Json(json!(links))),
        Err(_) => Err(Status::InternalServerError)
    }
}
