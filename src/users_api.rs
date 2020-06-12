use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::auth::Auth;
use crate::db::Conn as DbConn;
use crate::users_models::{NewUser, User};

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/new", data = "<user_form>")]
pub fn new(mut cookies: Cookies<'_>, conn: DbConn, user_form: Form<Login>) -> Flash<Redirect> {
    let user_info = user_form.into_inner();

    let new_user = match NewUser::new_from_plain(user_info.username, user_info.password) {
        Some(new) => new,
        None => return Flash::error(Redirect::to("/signup"), "An internal server error occurred"),
    };

    match User::insert(&new_user, &conn) {
        Ok(new_user) => {
            cookies.add_private(Cookie::new("user_id", new_user.id.to_string()));
            Flash::success(Redirect::to("/"), "Account created")
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Flash::error(Redirect::to("/signup"), "Username already taken")
        }
        Err(_) => Flash::error(Redirect::to("/signup"), "An internal server error occurred"),
    }
}

#[post("/login", data = "<user_form>")]
pub fn login(mut cookies: Cookies<'_>, conn: DbConn, user_form: Form<Login>) -> Flash<Redirect> {
    let login = user_form.into_inner();
    match User::get_by_name(&login.username, &conn) {
        Ok(selected_user) => {
            if selected_user.verify(&login.password) {
                cookies.add_private(Cookie::new("user_id", selected_user.id.to_string()));
                return Flash::success(Redirect::to("/"), "Logged in");
            } else {
                return Flash::error(Redirect::to("/login"), "Invalid username/password");
            }
        }
        Err(_) => return Flash::error(Redirect::to("/login"), "Invalid username/password"),
    };
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/login")
}

#[delete("/delete")]
pub fn delete(auth: Auth, mut cookies: Cookies<'_>, conn: DbConn) -> Status {
    match User::delete(auth.user_id, &conn) {
        Ok(_) => {
            cookies.remove_private(Cookie::named("user_id"));
            Status::Ok
        }
        Err(Error::NotFound) => Status::Unauthorized,
        Err(_) => Status::InternalServerError,
    }
}
