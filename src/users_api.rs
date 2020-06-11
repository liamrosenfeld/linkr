use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::Redirect;

use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::db::Conn as DbConn;
use crate::users_models::{NewUser, User};

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

    let new_user = match NewUser::new_from_plain(user_info.username, user_info.password) {
        Some(new) => new,
        None => {
            return Err(Status::InternalServerError);
        }
    };

    match User::insert(&new_user, &conn) {
        Ok(new_user) => {
            cookies.add_private(Cookie::new("user_id", new_user.id.to_string()));
            Ok(Redirect::to("/"))
        }
        Err(err) => Err(error_status(err)),
    }
}

#[derive(FromForm)]
pub struct ID {
    id: i32,
}

#[post("/delete", data = "<id_form>")]
pub fn delete(conn: DbConn, id_form: Form<ID>) -> Status {
    let id = id_form.into_inner().id;
    match User::delete(id, &conn) {
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
