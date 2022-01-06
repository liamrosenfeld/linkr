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

use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::Error;

use crate::crypto::encrypt_pw;
use crate::db::Db;
use crate::models::users::{InsertableUser, User};

/* ----------------------------------- new ---------------------------------- */

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub manage_links: bool,
    pub manage_users: bool,
}

#[post("/new", data = "<new_user_form>")]
pub async fn new(
    new_user_form: Json<NewUser>,
    user: Option<User>,
    cookies: &CookieJar<'_>,
    mut conn: Connection<Db>,
) -> Result<status::Created<()>, status::Custom<&'static str>> {
    // if user is authorized and has manage user permission, allow creation of not original
    // if user is authorized and they do not have manage user permission, return forbidden
    // if user is not authorized and there are no users, allow creation of original
    // if user is not authorized and there are existing users, return unauthorized
    // if unexpected database error occurs, return internal server error
    let orig = match user {
        Some(user) => {
            if !user.manage_users {
                return Err(status::Custom(Status::Forbidden, "You cannot manage users"));
            } else {
                false
            }
        }
        None => match User::count(&mut conn).await {
            Ok(count) => {
                if count == 0 {
                    true
                } else {
                    return Err(status::Custom(
                        Status::Unauthorized,
                        "You are not logged in",
                    ));
                }
            }
            Err(_) => {
                return Err(status::Custom(
                    Status::InternalServerError,
                    "Could not verify that your user exists",
                ));
            }
        },
    };

    // validate user info
    let user_info = new_user_form.into_inner();
    if user_info.username.is_empty() {
        return Err(status::Custom(
            Status::BadRequest,
            "Username cannot be blank",
        ));
    }
    if user_info.password.is_empty() {
        return Err(status::Custom(
            Status::BadRequest,
            "Password cannot be blank",
        ));
    }

    // encrypt password
    let new_user = InsertableUser::new_from_plain(user_info, orig);

    // insert user
    return match User::insert(new_user, &mut conn).await {
        Ok(new_id) => {
            if orig {
                cookies.add_private(Cookie::new("user_id", new_id.to_string()));
            }
            Ok(status::Created::new(new_id.to_string()))
        }
        Err(Error::Database(database_err)) => {
            if database_err.code().expect("No database error code") == "23505" {
                Err(status::Custom(Status::Conflict, "Username already taken"))
            } else {
                Err(status::Custom(
                    Status::InternalServerError,
                    "Could not create new user",
                ))
            }
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Could not create new user",
        )),
    };
}

/* ----------------------------- log in and out ----------------------------- */

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<login_json>")]
pub async fn login(
    login_json: Json<Login>,
    cookies: &CookieJar<'_>,
    mut conn: Connection<Db>,
) -> Result<status::Custom<Json<PublicUser>>, status::Unauthorized<&'static str>> {
    let login = login_json.into_inner();
    match User::get_by_name(login.username, &mut conn).await {
        Ok(selected_user) => {
            if selected_user.disabled {
                return Err(status::Unauthorized(Some("That user is disabled")));
            }
            if selected_user.verify(&login.password) {
                cookies.add_private(Cookie::new("user_id", selected_user.id.to_string()));
                return Ok(status::Custom(
                    Status::Ok,
                    Json(PublicUser::from_user(selected_user)),
                ));
            } else {
                return Err(status::Unauthorized(Some("Invalid username/password")));
            }
        }
        Err(_) => Err(status::Unauthorized(Some("Invalid username/password"))),
    }
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("user_id"));
    return Status::Ok;
}

/* ----------------------------- getting ----------------------------- */

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
    pub orig: bool,
    pub manage_links: bool,
    pub manage_users: bool,
    pub disabled: bool,
}

impl PublicUser {
    fn from_user(user: User) -> PublicUser {
        PublicUser {
            id: user.id,
            username: user.username,
            orig: user.orig,
            manage_links: user.manage_links,
            manage_users: user.manage_users,
            disabled: user.disabled,
        }
    }
}

#[get("/all")]
pub async fn get_all(
    user: User,
    mut conn: Connection<Db>,
) -> Result<status::Custom<Json<Vec<PublicUser>>>, status::Custom<()>> {
    if !user.manage_users {
        return Err(status::Custom(Status::Forbidden, ()));
    }
    match User::all(&mut conn).await {
        Ok(all_users) => {
            let public_users: Vec<PublicUser> = all_users
                .into_iter()
                .map(|user| PublicUser::from_user(user))
                .collect();
            Ok(status::Custom(Status::Accepted, Json(public_users)))
        }
        Err(_) => Err(status::Custom(Status::InternalServerError, ())),
    }
}

#[get("/current")]
pub async fn get_current(user: User) -> status::Custom<Json<PublicUser>> {
    status::Custom(Status::Ok, Json(PublicUser::from_user(user)))
}

/* -------------------------------- destruct -------------------------------- */

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteRequest {
    id: i32,
    password: Option<String>,
}

#[delete("/delete", data = "<request_form>")]
pub async fn delete(
    request_form: Json<DeleteRequest>,
    user: User,
    mut conn: Connection<Db>,
) -> Result<status::Custom<()>, status::Custom<String>> {
    let request = request_form.into_inner();
    if let Err(err) = check_destruct(&request, &user, &mut conn, "delete").await {
        return Err(err);
    }
    let res = User::delete(request.id, &mut conn).await;
    match_destruct_result_other(res)
}

#[patch("/disable", data = "<request_form>")]
pub async fn disable(
    request_form: Json<DeleteRequest>,
    user: User,
    mut conn: Connection<Db>,
) -> Result<status::Custom<()>, status::Custom<String>> {
    let request = request_form.into_inner();
    if let Err(err) = check_destruct(&request, &user, &mut conn, "disable").await {
        return Err(err);
    }
    let res = User::disable(request.id, &mut conn).await;
    match_destruct_result_other(res)
}

#[patch("/enable", data = "<request_form>")]
pub async fn enable(
    request_form: Json<DeleteRequest>,
    user: User,
    mut conn: Connection<Db>,
) -> Result<status::Custom<()>, status::Custom<String>> {
    let request = request_form.into_inner();
    if let Err(err) = check_destruct(&request, &user, &mut conn, "enable").await {
        return Err(err);
    }
    let res = User::enable(request.id, &mut conn).await;
    match_destruct_result_other(res)
}

pub async fn check_destruct(
    request: &DeleteRequest,
    current_user: &User,
    conn: &mut Connection<Db>,
    verb: &str,
) -> Result<(), status::Custom<String>> {
    return match User::get(request.id, conn).await {
        Ok(delete_user) => {
            // Action user is original -> No
            if delete_user.orig {
                Err(status::Custom(
                    Status::MethodNotAllowed,
                    format!("You cannot {} the original user", verb),
                ))
            }
            // Current user is action user -> Check password confirmation
            else if current_user.id == request.id {
                // check password
                match &request.password {
                    Some(pw) => {
                        if !current_user.verify(pw) {
                            Err(status::Custom(
                                Status::Unauthorized,
                                "Incorrect password".to_string(),
                            ))
                        } else {
                            Ok(())
                        }
                    }
                    None => Err(status::Custom(
                        Status::Unauthorized,
                        format!("You need your password to {} yourself", verb),
                    )),
                }
            }
            // Else -> Check if current user can manage users
            else if current_user.manage_users {
                Ok(())
            } else {
                Err(status::Custom(
                    Status::Forbidden,
                    "You cannot manage users.".to_string(),
                ))
            }
        }
        Err(Error::RowNotFound) => Err(status::Custom(
            Status::NotFound,
            "That user cannot be found.".to_string(),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "An internal server error occurred.".to_string(),
        )),
    };
}

fn match_destruct_result_other(
    result: sqlx::Result<()>,
) -> Result<status::Custom<()>, status::Custom<String>> {
    match result {
        Ok(()) => Ok(status::Custom(Status::Ok, ())),
        Err(Error::Database(database_error)) => {
            if database_error.code().expect("No database error code") == "23505" {
                Err(status::Custom(
                    Status::Conflict,
                    "That username is taken".to_string(),
                ))
            } else {
                Err(status::Custom(
                    Status::InternalServerError,
                    "An internal server error occurred".to_string(),
                ))
            }
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "An internal server error occurred.".to_string(),
        )),
    }
}

/* --------------------------------- update --------------------------------- */

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PermissionsUpdate {
    user_id: i32,
    manage_links: bool,
    manage_users: bool,
}

#[patch("/update/permissions", data = "<permissions_form>")]
pub async fn update_permissions(
    permissions_form: Json<PermissionsUpdate>,
    user: User,
    mut conn: Connection<Db>,
) -> status::Custom<()> {
    let permissions = permissions_form.into_inner();

    if !user.manage_users {
        return status::Custom(Status::Forbidden, ());
    }

    match User::update_permissions(
        permissions.user_id,
        permissions.manage_links,
        permissions.manage_users,
        &mut conn,
    )
    .await
    {
        Ok(_) => status::Custom(Status::Ok, ()),
        Err(Error::RowNotFound) => status::Custom(Status::NotFound, ()),
        Err(_) => status::Custom(Status::InternalServerError, ()),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UsernameUpdate {
    user_id: i32,
    new_name: String,
}

#[patch("/update/username", data = "<username_form>", rank = 1)]
pub async fn update_username(
    username_form: Json<UsernameUpdate>,
    user: User,
    mut conn: Connection<Db>,
) -> Result<status::Custom<()>, status::Custom<&'static str>> {
    let username_update = username_form.into_inner();

    // anyone can change their own username
    // users with manage_users can update all usernames
    if username_update.user_id != user.id && !user.manage_users {
        return Err(status::Custom(
            Status::Forbidden,
            "You do not have access to edit that user's username",
        ));
    }

    match User::update_username(username_update.user_id, username_update.new_name, &mut conn).await
    {
        Ok(_) => Ok(status::Custom(Status::Ok, ())),
        Err(Error::RowNotFound) => {
            Err(status::Custom(Status::NotFound, "That user does not exist"))
        }
        Err(Error::Database(database_error)) => {
            if database_error.code().expect("No database error code") == "23505" {
                Err(status::Custom(Status::Conflict, "That username is taken"))
            } else {
                Err(status::Custom(
                    Status::InternalServerError,
                    "An internal server error occurred",
                ))
            }
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "An internal server error occurred",
        )),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordUpdate {
    current_pw: String,
    new_pw: String,
}

#[patch("/update/password", data = "<pw_form>")]
pub async fn update_password(
    pw_form: Json<PasswordUpdate>,
    user: User,
    cookies: &CookieJar<'_>,
    mut conn: Connection<Db>,
) -> Result<status::Custom<()>, status::Custom<&'static str>> {
    let passwords = pw_form.into_inner();

    if !user.verify(&passwords.current_pw) {
        return Err(status::Custom(
            Status::Unauthorized,
            "Incorrect current password",
        ));
    }

    if passwords.new_pw == passwords.current_pw {
        return Err(status::Custom(
            Status::Conflict,
            "New password cannot be current password",
        ));
    }

    let pw_hash = encrypt_pw(&passwords.new_pw);

    match User::update_password(user.id, pw_hash, &mut conn).await {
        Ok(_) => {
            cookies.remove_private(Cookie::named("user_id"));
            Ok(status::Custom(Status::Ok, ()))
        }
        Err(_) => Err(status::Custom(
            Status::Conflict,
            "An internal server error occurred",
        )),
    }
}
