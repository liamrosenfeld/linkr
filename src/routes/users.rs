use rocket::http::{Cookie, Cookies, Status};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::auth::Auth;
use crate::crypto::encrypt_pw;
use crate::db::Conn as DbConn;
use crate::models::users::{InsertableUser, User};

#[derive(FromForm)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub manage_links: bool,
    pub manage_users: bool,
}

#[post("/new", data = "<user_form>")]
pub fn new(
    user_form: Form<NewUser>,
    auth: Option<Auth>,
    mut cookies: Cookies<'_>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    // if user is authorized and has manage user permission, allow creation of not original
    // if user is authorized and they do not have manage user permission, return forbidden
    // if user is not authorized and there are no users, allow creation of original
    // if user is not authorized and there are existing users, return unauthorized
    // if unexpected database error occurs, return internal server error
    let orig = match auth {
        Some(auth) => match User::get(auth.user_id, &conn) {
            Ok(user) => {
                if !user.manage_users {
                    return Err(Status::Forbidden);
                } else {
                    false
                }
            }
            Err(Error::NotFound) => return Err(Status::Unauthorized),
            Err(_) => return Err(Status::InternalServerError),
        },
        None => match User::count(&conn) {
            Ok(count) => {
                if count == 0 {
                    true
                } else {
                    return Err(Status::Unauthorized);
                }
            }
            Err(_) => return Err(Status::InternalServerError),
        },
    };

    let user_info = user_form.into_inner();
    let new_user = InsertableUser::new_from_plain(user_info, orig);

    match User::insert(&new_user, &conn) {
        Ok(new_user) => {
            if orig {
                cookies.add_private(Cookie::new("user_id", new_user.id.to_string()));
            }
            let to = if orig { "/" } else { "/new_user" };
            Ok(Flash::success(
                Redirect::to(to),
                format!("Account created: {}", new_user.username),
            ))
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(Flash::error(
            Redirect::to("/new_user"),
            "Username already taken",
        )),
        Err(_) => Ok(Flash::error(
            Redirect::to("/new_user"),
            "An internal server error occurred",
        )),
    }
}

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<user_form>")]
pub fn login(user_form: Form<Login>, mut cookies: Cookies<'_>, conn: DbConn) -> Flash<Redirect> {
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

#[post("/delete", data = "<id_form>")]
pub fn delete_by_id(id_form: Form<ID>, auth: Auth, conn: DbConn) -> Status {
    let id = id_form.into_inner().id;

    if id == auth.user_id {
        return Status::MethodNotAllowed;
    }

    match User::get(auth.user_id, &conn) {
        Ok(current_user) => {
            if current_user.manage_users {
                match User::get(id, &conn) {
                    // block if user deleted is original
                    Ok(delete_user) => {
                        if delete_user.orig {
                            return Status::MethodNotAllowed;
                        }
                    }
                    Err(Error::NotFound) => return Status::NotFound,
                    Err(_) => return Status::InternalServerError,
                }

                // delete user
                match User::delete(id, &conn) {
                    Ok(_) => Status::Ok,
                    Err(Error::NotFound) => Status::NotFound,
                    Err(_) => Status::InternalServerError,
                }
            } else {
                Status::Forbidden
            }
        }
        Err(Error::NotFound) => Status::Unauthorized,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct Password {
    password: String,
}

#[post("/delete_current", data = "<pw_form>")]
pub fn delete_current(
    pw_form: Form<Password>,
    auth: Auth,
    mut cookies: Cookies<'_>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    // block if user deleted is original
    let user = match User::get(auth.user_id, &conn) {
        Ok(delete_user) => {
            if delete_user.orig {
                return Err(Status::MethodNotAllowed);
            }
            delete_user
        }
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => {
            return Ok(Flash::error(
                Redirect::to("/manage_account"),
                "An internal server error occurred",
            ))
        }
    };

    // check password
    let pw = pw_form.into_inner().password;
    if !user.verify(&pw) {
        return Ok(Flash::error(
            Redirect::to("/manage_account"),
            "Incorrect password",
        ));
    }

    // delete user
    match User::delete(auth.user_id, &conn) {
        Ok(_) => {
            cookies.remove_private(Cookie::named("user_id"));
            Ok(Flash::success(Redirect::to("/login"), "User deleted"))
        }
        Err(Error::NotFound) => Err(Status::Unauthorized),
        Err(_) => Ok(Flash::error(
            Redirect::to("/manage_account"),
            "An internal server error occurred",
        )),
    }
}

#[derive(FromForm)]
pub struct PermissionsUpdate {
    user_id: i32,
    manage_links: bool,
    manage_users: bool,
}

#[post("/update/permissions", data = "<permissions_form>")]
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

#[derive(FromForm)]
pub struct UsernameUpdate {
    user_id: i32,
    new_name: String,
}

#[post("/update/username", data = "<username_form>", rank = 1)]
pub fn update_username(username_form: Form<UsernameUpdate>, auth: Auth, conn: DbConn) -> Status {
    let username_update = username_form.into_inner();

    // anyone can change their own username
    if username_update.user_id != auth.user_id {
        // users with manage_users can update all usernames
        match User::get(auth.user_id, &conn) {
            Ok(user) => {
                if !user.manage_users {
                    return Status::Forbidden;
                }
            }
            Err(Error::NotFound) => return Status::Unauthorized,
            Err(_) => return Status::InternalServerError,
        }
    }

    match User::update_username(username_update.user_id, &username_update.new_name, &conn) {
        Ok(_) => Status::Ok,
        Err(Error::NotFound) => Status::NotFound,
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Status::Conflict,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct NewUsername {
    username: String,
}

#[post("/update/username", data = "<username_form>", rank = 2)]
pub fn update_own_username(
    username_form: Form<NewUsername>,
    auth: Auth,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    let new_name = username_form.into_inner().username;

    match User::update_username(auth.user_id, &new_name, &conn) {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/manage_account"),
            "Username Updated!",
        )),
        Err(Error::NotFound) => Err(Status::Unauthorized),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(Flash::error(
            Redirect::to("/manage_account"),
            "That username is taken",
        )),
        Err(_) => Ok(Flash::error(
            Redirect::to("/manage_account"),
            "An internal server error occurred",
        )),
    }
}

#[derive(FromForm)]
pub struct PasswordUpdate {
    current_pw: String,
    new_pw: String,
}

#[post("/update/password", data = "<pw_form>")]
pub fn update_password(
    pw_form: Form<PasswordUpdate>,
    auth: Auth,
    mut cookies: Cookies<'_>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    let passwords = pw_form.into_inner();

    let user = match User::get(auth.user_id, &conn) {
        Ok(user) => user,
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => {
            return Ok(Flash::error(
                Redirect::to("/manage_account"),
                "An internal server error occurred",
            ))
        }
    };

    if !user.verify(&passwords.current_pw) {
        return Ok(Flash::error(
            Redirect::to("/manage_account"),
            "Incorrect current password",
        ));
    }

    if passwords.new_pw == passwords.current_pw {
        return Ok(Flash::error(
            Redirect::to("/manage_account"),
            "New password cannot be current password",
        ));
    }

    let pw_hash = encrypt_pw(&passwords.new_pw);

    match User::update_password(auth.user_id, &pw_hash, &conn) {
        Ok(_) => {
            cookies.remove_private(Cookie::named("user_id"));
            Ok(Flash::success(Redirect::to("/login"), "Password changed!"))
        }
        Err(_) => Ok(Flash::error(
            Redirect::to("/manage_account"),
            "An internal server error occurred",
        )),
    }
}
