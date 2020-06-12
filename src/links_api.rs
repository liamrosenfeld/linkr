use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use rocket_contrib::json::Json;
use serde_json::Value;

use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::auth::Auth;
use crate::db::Conn as DbConn;
use crate::links_models::{Link, NewLink};
use crate::users_models::User;

use std::convert::TryInto;

/* --------------------------------- lookup --------------------------------- */

#[get("/<short>", rank = 3)]
pub fn lookup(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get_by_short(short, &conn) {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(err) => Err(error_status(err)),
    }
}

/* ----------------------------------- api ---------------------------------- */

const RESERVED_LINKS: [&str; 3] = ["api", "login", "resource"];

#[post("/new", data = "<link_form>")]
pub fn shorten(
    conn: DbConn,
    link_form: Form<NewLink>,
    auth: Auth,
) -> Result<Flash<Redirect>, Status> {
    let _user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::Unauthorized),
    };

    let link = link_form.into_inner();

    // check if the short is alphanumeric
    if !link.short.chars().all(char::is_alphanumeric) {
        return Ok(Flash::error(
            Redirect::to("/"),
            "Shorts can only contain alphanumeric characters",
        ));
    }

    // check if the short is reserved by this site
    if RESERVED_LINKS
        .iter()
        .any(|&reserved| reserved == link.short)
    {
        return Ok(Flash::error(
            Redirect::to("/"),
            "That short is reserved by this website",
        ));
    }

    // send database request and respond accordingly
    match Link::insert(link, &conn) {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Link created!")),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(Flash::error(
            Redirect::to("/"),
            "That short is already in use",
        )),
        Err(_) => Ok(Flash::error(
            Redirect::to("/"),
            "There was an internal server error",
        )),
    }
}

#[derive(FromForm)]
pub struct ID {
    id: usize,
}

#[post("/delete", data = "<id_form>")]
pub fn delete(conn: DbConn, id_form: Form<ID>, auth: Auth) -> Status {
    let _user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(_) => return Status::Unauthorized,
    };

    let id = id_form.into_inner().id;

    match Link::delete_by_id(id.try_into().unwrap(), &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err),
    }
}

#[derive(FromForm)]
pub struct Update {
    id: usize,
    long: String,
}

#[post("/update", data = "<update_form>")]
pub fn update(conn: DbConn, update_form: Form<Update>, auth: Auth) -> Status {
    let _user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(_) => return Status::Unauthorized,
    };

    let update = update_form.into_inner();

    match Link::update_by_id(update.id.try_into().unwrap(), update.long, &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err),
    }
}

#[get("/all")]
pub fn all(conn: DbConn) -> Result<Json<Value>, Status> {
    match Link::all(&conn) {
        Ok(links) => Ok(Json(json!(links))),
        Err(err) => Err(error_status(err)),
    }
}

/* --------------------------------- helpers -------------------------------- */

fn error_status(err: Error) -> Status {
    match err {
        Error::NotFound => Status::NotFound,
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => Status::Conflict,
        _ => Status::InternalServerError,
    }
}
