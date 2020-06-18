use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::auth::Auth;
use crate::db::Conn as DbConn;
use crate::users_models::{InsertableUser, User};

#[derive(FromForm)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub manage_links: bool,
    pub manage_users: bool,
}

#[post("/new", data = "<user_form>")]
pub fn new(mut cookies: Cookies<'_>, conn: DbConn, user_form: Form<NewUser>) -> Flash<Redirect> {
    let user_info = user_form.into_inner();

    let new_user = match InsertableUser::new_from_plain(user_info) {
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

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
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

#[derive(FromForm)]
pub struct ID {
    id: i32,
}

#[post("/delete", data = "<id_form>", rank = 1)]
pub fn delete_by_id(id_form: Form<ID>, auth: Auth, conn: DbConn) -> Status {
    let id = id_form.into_inner().id;

    if id == auth.user_id {
        return Status::MethodNotAllowed;
    }

    match User::get(auth.user_id, &conn) {
        Ok(_) => match User::delete(id, &conn) {
            Ok(_) => Status::Ok,
            Err(Error::NotFound) => Status::NotFound,
            Err(_) => Status::InternalServerError,
        },
        Err(Error::NotFound) => Status::Unauthorized,
        Err(_) => Status::InternalServerError,
    }
}

#[delete("/delete", rank = 2)]
pub fn delete_current(auth: Auth, mut cookies: Cookies<'_>, conn: DbConn) -> Status {
    match User::delete(auth.user_id, &conn) {
        Ok(_) => {
            cookies.remove_private(Cookie::named("user_id"));
            Status::Ok
        }
        Err(Error::NotFound) => Status::Unauthorized,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct PermissionsUpdate {
    user_id: i32,
    manage_links: bool,
    manage_users: bool,
}

#[post("/update/permissions", data = "<permissions_form>", rank = 1)]
pub fn update_permissions(
    permissions_form: Form<PermissionsUpdate>,
    auth: Auth,
    conn: DbConn,
) -> Status {
    let permissions = permissions_form.into_inner();

    match User::get(auth.user_id, &conn) {
        Ok(user) => {
            if !user.manage_users {
                return Status::Forbidden;
            }
        }
        Err(Error::NotFound) => return Status::Unauthorized,
        Err(_) => return Status::InternalServerError,
    }

    match User::update_permissions(
        permissions.user_id,
        permissions.manage_links,
        permissions.manage_users,
        &conn,
    ) {
        Ok(_) => Status::Ok,
        Err(Error::NotFound) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}
