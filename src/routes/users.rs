// Copyright (C) 2020 Liam Rosenfeld
//
// This file is part of Linkr (https://github.com/liamrosenfeld/linkr).
//
// Linkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Linkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Linkr. If not, see <http://www.gnu.org/licenses/>.

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{Flash, Redirect};
use rocket_sync_db_pools::diesel::result::DatabaseErrorKind;
use rocket_sync_db_pools::diesel::result::Error;
use rocket_sync_db_pools::diesel::QueryResult;

use crate::crypto::encrypt_pw;
use crate::db::DbConn;
use crate::models::users::{InsertableUser, User};

/* ----------------------------------- new ---------------------------------- */

#[derive(FromForm)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub manage_links: bool,
    pub manage_users: bool,
}

#[post("/new", data = "<new_user_form>")]
pub async fn new(
    new_user_form: Form<NewUser>,
    user: Option<User>,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    // if user is authorized and has manage user permission, allow creation of not original
    // if user is authorized and they do not have manage user permission, return forbidden
    // if user is not authorized and there are no users, allow creation of original
    // if user is not authorized and there are existing users, return unauthorized
    // if unexpected database error occurs, return internal server error
    let orig = match user {
        Some(user) => {
            if !user.manage_users {
                return Err(Status::Forbidden);
            } else {
                false
            }
        }
        None => match User::count(&conn).await {
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
    let failure_to = if orig { "/setup" } else { "/new_user" };
    let success_to = if orig { "/" } else { "/new_user" };

    // validate user info
    let user_info = new_user_form.into_inner();
    if user_info.username.is_empty() {
        return Ok(Flash::error(
            Redirect::to(failure_to),
            "Your username cannot be blank",
        ));
    }
    if user_info.password.is_empty() {
        return Ok(Flash::error(
            Redirect::to(failure_to),
            "Your password cannot be blank",
        ));
    }

    // encrypt password
    let new_user = InsertableUser::new_from_plain(user_info, orig);

    // insert user
    match User::insert(new_user, &conn).await {
        Ok(new_user) => {
            if orig {
                cookies.add_private(Cookie::new("user_id", new_user.id.to_string()));
            }
            Ok(Flash::success(
                Redirect::to(success_to),
                format!("Account created: {}", new_user.username),
            ))
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(Flash::error(
            Redirect::to(failure_to),
            "Username already taken",
        )),
        Err(_) => Ok(Flash::error(
            Redirect::to(failure_to),
            "An internal server error occurred",
        )),
    }
}

/* ----------------------------- log in and out ----------------------------- */

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<user_form>")]
pub async fn login(
    user_form: Form<Login>,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Flash<Redirect> {
    let login = user_form.into_inner();
    match User::get_by_name(login.username, &conn).await {
        Ok(selected_user) => {
            if selected_user.disabled {
                return Flash::error(Redirect::to("/login"), "That user is disabled");
            }
            if selected_user.verify(&login.password) {
                cookies.add_private(Cookie::new("user_id", selected_user.id.to_string()));
                return Flash::success(Redirect::to("/"), "Logged in");
            } else {
                return Flash::error(Redirect::to("/login"), "Invalid username/password");
            }
        }
        Err(_) => Flash::error(Redirect::to("/login"), "Invalid username/password"),
    }
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/login")
}

/* -------------------------------- destruct -------------------------------- */

#[derive(FromForm)]
pub struct ID {
    id: i32,
}

#[post("/delete", data = "<id_form>")]
pub async fn delete_by_id(id_form: Form<ID>, user: User, conn: DbConn) -> Flash<Redirect> {
    let action_id = id_form.into_inner().id;
    if let Err(flash) = check_destruct_other(action_id, &user, &conn, "delete").await {
        return flash;
    }
    match_destruct_result_other(User::delete(action_id, &conn).await, "delete")
}

#[post("/disable", data = "<id_form>")]
pub async fn disable_by_id(id_form: Form<ID>, user: User, conn: DbConn) -> Flash<Redirect> {
    let action_id = id_form.into_inner().id;
    if let Err(flash) = check_destruct_other(action_id, &user, &conn, "disable").await {
        return flash;
    }
    match_destruct_result_other(User::disable(action_id, &conn).await, "disable")
}

#[post("/enable", data = "<id_form>")]
pub async fn enable_by_id(id_form: Form<ID>, user: User, conn: DbConn) -> Flash<Redirect> {
    let action_id = id_form.into_inner().id;
    if let Err(flash) = check_destruct_other(action_id, &user, &conn, "enable").await {
        return flash;
    }
    match_destruct_result_other(User::enable(action_id, &conn).await, "enable")
}

#[derive(FromForm)]
pub struct Password {
    password: String,
}

#[post("/delete_current", data = "<pw_form>")]
pub async fn delete_current(
    pw_form: Form<Password>,
    user: User,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Flash<Redirect> {
    let pw = pw_form.into_inner().password;
    if let Err(flash) = check_destruct_current(&pw, &user, "delete") {
        return flash;
    }
    match_result_current(User::delete(user.id, &conn).await, cookies, "delete")
}

#[post("/disable_current", data = "<pw_form>")]
pub async fn disable_current(
    pw_form: Form<Password>,
    user: User,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Flash<Redirect> {
    let pw = pw_form.into_inner().password;
    if let Err(flash) = check_destruct_current(&pw, &user, "disable") {
        return flash;
    }
    match_result_current(User::disable(user.id, &conn).await, cookies, "disable")
}

fn match_destruct_result_other(result: QueryResult<usize>, verb: &'static str) -> Flash<Redirect> {
    match result {
        Ok(_) => Flash::success(Redirect::to("/manage_users"), format!("User {}d.", verb)),
        Err(Error::NotFound) => {
            Flash::error(Redirect::to("/manage_users"), "That user cannot be found.")
        }
        Err(_) => Flash::error(
            Redirect::to("/manage_users"),
            "An internal server error occurred.",
        ),
    }
}

fn match_result_current(
    result: QueryResult<usize>,
    cookies: &CookieJar<'_>,
    verb: &'static str,
) -> Flash<Redirect> {
    match result {
        Ok(_) => {
            cookies.remove_private(Cookie::named("user_id"));
            Flash::success(Redirect::to("/login"), format!("User {}d", verb))
        }
        Err(Error::NotFound) => Flash::error(Redirect::to("/"), "That user does not exist."),
        Err(_) => Flash::error(
            Redirect::to("/manage_account"),
            "An internal server error occurred",
        ),
    }
}

pub async fn check_destruct_other(
    action_id: i32,
    current_user: &User,
    conn: &DbConn,
    verb: &str,
) -> Result<(), Flash<Redirect>> {
    if action_id == current_user.id {
        return Err(Flash::error(
            Redirect::to("/manage_users"),
            format!("You can only {} yourself in account settings", verb),
        ));
    }

    if current_user.manage_users {
        match User::get(action_id, conn).await {
            // block if user deleted is original
            Ok(delete_user) => {
                if delete_user.orig {
                    return Err(Flash::error(
                        Redirect::to("/manage_users"),
                        format!("You cannot {} the original user", verb),
                    ));
                }
            }
            Err(Error::NotFound) => {
                return Err(Flash::error(
                    Redirect::to("/manage_users"),
                    "That user cannot be found.",
                ))
            }
            Err(_) => {
                return Err(Flash::error(
                    Redirect::to("/manage_users"),
                    "An internal server error occurred.",
                ))
            }
        }
    } else {
        return Err(Flash::error(Redirect::to("/"), "You cannot manage users."));
    }

    return Ok(());
}

pub fn check_destruct_current(
    pw: &str,
    user: &User,
    verb: &'static str,
) -> Result<(), Flash<Redirect>> {
    // block if user to delete is original
    if user.orig {
        return Err(Flash::error(
            Redirect::to("/manage_users"),
            format!("You cannot {} the original user", verb),
        ));
    }

    // check password
    if pw == "" {
        return Err(Flash::error(
            Redirect::to("/manage_account"),
            "Enter your current password to delete your account",
        ));
    }
    if !user.verify(pw) {
        return Err(Flash::error(
            Redirect::to("/manage_account"),
            "Incorrect password",
        ));
    }
    return Ok(());
}

/* --------------------------------- update --------------------------------- */

#[derive(FromForm)]
pub struct PermissionsUpdate {
    user_id: i32,
    manage_links: bool,
    manage_users: bool,
}

#[post("/update/permissions", data = "<permissions_form>")]
pub async fn update_permissions(
    permissions_form: Form<PermissionsUpdate>,
    user: User,
    conn: DbConn,
) -> Status {
    let permissions = permissions_form.into_inner();

    if !user.manage_users {
        return Status::Forbidden;
    }

    match User::update_permissions(
        permissions.user_id,
        permissions.manage_links,
        permissions.manage_users,
        &conn,
    )
    .await
    {
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
pub async fn update_username(
    username_form: Form<UsernameUpdate>,
    user: User,
    conn: DbConn,
) -> Status {
    let username_update = username_form.into_inner();

    // anyone can change their own username
    // users with manage_users can update all usernames
    if username_update.user_id != user.id && !user.manage_users {
        return Status::Forbidden;
    }

    match User::update_username(username_update.user_id, username_update.new_name, &conn).await {
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
pub async fn update_own_username(
    username_form: Form<NewUsername>,
    user: User,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    let new_name = username_form.into_inner().username;

    match User::update_username(user.id, new_name, &conn).await {
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
pub async fn update_password(
    pw_form: Form<PasswordUpdate>,
    user: User,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Status> {
    let passwords = pw_form.into_inner();

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

    match User::update_password(user.id, pw_hash, &conn).await {
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
