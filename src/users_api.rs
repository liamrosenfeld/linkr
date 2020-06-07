use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::Redirect;

use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::db::Conn as DbConn;
use crate::users_models::User;

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/new", data = "<user_form>")]
pub fn new(
    mut cookies: Cookies<'_>,
    conn: DbConn,
    user_form: Form<Login>,
) -> Result<Redirect, Status> {
    let user_info = user_form.into_inner();

    let new_user = match User::new_from_plain(user_info.username, user_info.password) {
        Some(new) => new,
        None => {
            return Err(Status::InternalServerError);
        }
    };

    match User::insert(&new_user, &conn) {
        Ok(_) => {
            cookies.add_private(Cookie::new("user_id", new_user.username));
            Ok(Redirect::to("/"))
        }
        Err(err) => Err(error_status(err)),
    }
}

#[derive(FromForm)]
pub struct Username {
    username: String,
}

#[post("/delete", data = "<username_form>")]
pub fn delete(conn: DbConn, username_form: Form<Username>) -> Status {
    let username = username_form.into_inner().username;
    match User::delete(&username, &conn) {
        Ok(_) => Status::Ok,
        Err(err) => error_status(err),
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
