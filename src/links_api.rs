use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use rocket_contrib::json::Json;
use serde_json::Value;

use chrono::Utc;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::auth::Auth;
use crate::db::Conn as DbConn;
use crate::links_models::Link;
use crate::users_models::User;

/* --------------------------------- lookup --------------------------------- */

#[get("/<short>", rank = 3)]
pub fn lookup(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get(&short, &conn) {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(err) => Err(error_status(err)),
    }
}

/* ----------------------------------- api ---------------------------------- */

#[derive(FromForm)]
pub struct NewLink {
    short: String,
    long: String,
}

const RESERVED_LINKS: [&str; 3] = ["api", "login", "resource"];

#[post("/new", data = "<link_form>")]
pub fn shorten(
    conn: DbConn,
    link_form: Form<NewLink>,
    auth: Auth,
) -> Result<Flash<Redirect>, Status> {
    let user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::Unauthorized),
    };

    let new_link = link_form.into_inner();

    // check if the short is alphanumeric
    if !new_link.short.chars().all(char::is_alphanumeric) {
        return Ok(Flash::error(
            Redirect::to("/"),
            "Shorts can only contain alphanumeric characters",
        ));
    }

    // check if the short is reserved by this site
    if RESERVED_LINKS
        .iter()
        .any(|&reserved| reserved == new_link.short)
    {
        return Ok(Flash::error(
            Redirect::to("/"),
            "That short is reserved by this website",
        ));
    }

    // create link to insert
    let link = Link {
        short: new_link.short,
        long: new_link.long,
        created_at: Utc::now(),
        created_by: user.id,
    };

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
pub struct Short {
    short: String,
}

#[post("/delete", data = "<short_form>")]
pub fn delete(conn: DbConn, short_form: Form<Short>, auth: Auth) -> Status {
    let _user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(_) => return Status::Unauthorized,
    };

    let short = short_form.into_inner().short;

    match Link::delete(&short, &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err),
    }
}

#[post("/update", data = "<update_form>")]
pub fn update(conn: DbConn, update_form: Form<NewLink>, auth: Auth) -> Status {
    let _user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(_) => return Status::Unauthorized,
    };

    let update = update_form.into_inner();

    match Link::update(&update.short, &update.long, &conn) {
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
