use rocket::request::Form;
use rocket::http::Status;

use rocket_contrib::json::Json;
use serde_json::Value;

use diesel::result::Error;
use diesel::result::DatabaseErrorKind;

use crate::db::Conn as DbConn;
use crate::users_models::{User, NewUser};

use std::convert::TryInto;

#[derive(FromForm)]
pub struct NewUserInfo {
    username: String,
    password: String
}

#[post("/new", data = "<user_form>")]
pub fn new(conn: DbConn, user_form: Form<NewUserInfo>) -> Status {
    let user_info = user_form.into_inner();

    let new_user = match NewUser::new_from_plain(user_info.username, user_info.password) {
        Some(new) => new,
        None => { return Status::InternalServerError; }
    };

    match User::insert(new_user, &conn) {
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
    match User::delete_by_id(id.try_into().unwrap(), &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err)
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
