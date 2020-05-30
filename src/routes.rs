use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::Status;

use diesel::result::Error;

use crate::db::Conn as DbConn;
use crate::models::{Link, NewLink};

/* --------------------------------- lookup --------------------------------- */

#[get("/<short>")]
pub fn lookup(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get_by_short(short, &conn) {
        Ok(link) => Ok(Redirect::permanent(link.orig)),
        Err(Error::NotFound) => Err(Status::new(404, "NotFound")),
        _ => Err(Status::new(500, "InternalError"))
    }
}

/* ----------------------------------- api ---------------------------------- */

#[post("/shorten", data = "<link_form>")]
pub fn shorten(conn: DbConn, link_form: Form<NewLink>) -> Result<String, String> {
    let link = link_form.into_inner();
    if Link::insert(link, &conn) {
        Ok("Added!".to_string())
    } else {
        Err("Could not add".to_string())
    }
}
