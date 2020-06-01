use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::Status;

use rocket_contrib::json::Json;
use serde_json::Value;

use diesel::result::Error;
use diesel::result::DatabaseErrorKind;

use crate::db::Conn as DbConn;
use crate::models::{Link, NewLink};

use std::convert::TryInto;

/* --------------------------------- lookup --------------------------------- */

#[get("/<short>")]
pub fn lookup(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get_by_short(short, &conn) {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(err) => Err(error_status(err))
    }
}

/* ----------------------------------- api ---------------------------------- */

#[post("/shorten", data = "<link_form>")]
pub fn shorten(conn: DbConn, link_form: Form<NewLink>) -> Status {
    let link = link_form.into_inner();
    match Link::insert(link, &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err)
    }
}

#[derive(FromForm)]
pub struct ID {
    id: usize
}

#[post("/delete", data = "<id_form>")]
pub fn delete(conn: DbConn, id_form: Form<ID>) -> Status {
    let id = id_form.into_inner().id;
    match Link::delete_by_id(id.try_into().unwrap(), &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err)
    }
}

#[get("/all")]
pub fn all(conn: DbConn) -> Result<Json<Value>, Status> {
    match Link::all(&conn) {
        Ok(links) => Ok(Json(json!(links))),
        Err(err) => Err(error_status(err))
    }
}

/* --------------------------------- helpers -------------------------------- */

fn error_status(err: Error) -> Status {
    match err {
        Error::NotFound => Status::NotFound,
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => Status::Conflict,
        _ => Status::InternalServerError
    }
}
