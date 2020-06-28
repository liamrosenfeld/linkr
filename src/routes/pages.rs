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

use crate::auth::Auth;
use crate::db::Conn as DbConn;
use crate::models::links::Link;
use crate::models::users::User;

use diesel::result::Error;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde_json::value::Value;

#[get("/<short>", rank = 3)]
pub fn link(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get(&short, &conn) {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(Error::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
pub fn index(auth: Auth, flash: Option<FlashMessage>, conn: DbConn) -> Result<Template, Status> {
    // user from auth (from cookie)
    let user = get_user(auth.user_id, &conn)?;

    // links for table
    let links = match Link::all_for_user(auth.user_id, &conn) {
        Ok(links) => links,
        Err(_) => return Err(Status::InternalServerError),
    };

    // render template
    let context = json!({
        "links": links,
        "user": user,
        "flash": flash_json(&flash),
    });
    Ok(Template::render("pages/index", &context))
}

#[get("/manage_links")]
pub fn manage_links(
    auth: Auth,
    flash: Option<FlashMessage>,
    conn: DbConn,
) -> Result<Template, Status> {
    // user from auth (from cookie)
    let user = get_user(auth.user_id, &conn)?;
    if !user.manage_links {
        return Err(Status::Forbidden);
    }

    // links for table
    let links = match Link::all(&conn) {
        Ok(links) => links,
        Err(_) => return Err(Status::InternalServerError),
    };

    // render template
    let context = json!({
        "links": links,
        "user": user,
        "flash": flash_json(&flash)
    });
    Ok(Template::render("pages/manage_links", &context))
}

#[get("/manage_users")]
pub fn manage_users(
    auth: Auth,
    flash: Option<FlashMessage>,
    conn: DbConn,
) -> Result<Template, Status> {
    // user from auth (from cookie)
    let user = get_user(auth.user_id, &conn)?;
    if !user.manage_users {
        return Err(Status::Forbidden);
    }

    // links for table
    let users = match User::all(&conn) {
        Ok(users) => users,
        Err(_) => return Err(Status::InternalServerError),
    };

    // render template
    let context = json!({
        "users": users,
        "user": user,
        "flash": flash_json(&flash)
    });
    Ok(Template::render("pages/manage_users", &context))
}

#[get("/manage_account")]
pub fn manage_account(
    auth: Auth,
    flash: Option<FlashMessage>,
    conn: DbConn,
) -> Result<Template, Status> {
    let user = get_user(auth.user_id, &conn)?;
    let context = json!({
        "user": user,
        "flash": flash_json(&flash)
    });
    Ok(Template::render("pages/manage_account", &context))
}

#[get("/new_user")]
pub fn new_user(auth: Auth, flash: Option<FlashMessage>, conn: DbConn) -> Result<Template, Status> {
    // check permission
    let user = get_user(auth.user_id, &conn)?;
    if !user.manage_users {
        return Err(Status::Forbidden);
    }

    let context = json!({
        "user": user,
        "flash": flash_json(&flash)
    });
    Ok(Template::render("pages/new_user", &context))
}

#[get("/login")]
pub fn login(
    auth: Option<Auth>,
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    conn: DbConn,
) -> Result<Template, Redirect> {
    match auth {
        Some(auth) => match User::get(auth.user_id, &conn) {
            Ok(_) => Err(Redirect::to("/")),
            Err(_) => {
                cookies.remove_private(Cookie::named("user_id"));
                drop(cookies); // need to drop before accessing flash
                let context = just_flash_context(&flash);
                Ok(Template::render("pages/login", &context))
            }
        },
        None => {
            drop(cookies); // need to drop before accessing flash
            let context = just_flash_context(&flash);
            Ok(Template::render("pages/login", &context))
        }
    }
}

#[get("/setup")]
pub fn setup(flash: Option<FlashMessage>, conn: DbConn) -> Result<Template, Status> {
    match User::count(&conn) {
        Ok(count) => {
            if count == 0 {
                let context = just_flash_context(&flash);
                Ok(Template::render("pages/setup", &context))
            } else {
                Err(Status::Forbidden)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/* --------------------------------- helpers -------------------------------- */

fn get_user(id: i32, conn: &DbConn) -> Result<User, Status> {
    let user = User::get(id, conn);
    match user {
        Ok(user) => Ok(user),
        Err(Error::NotFound) => Err(Status::Unauthorized),
        Err(_) => Err(Status::InternalServerError),
    }
}

fn flash_json(flash: &Option<FlashMessage>) -> Value {
    match flash {
        Some(flash) => json!({
            "type": flash.name(),
            "msg": flash.msg(),
        }),
        None => json!(null),
    }
}

fn just_flash_context(flash: &Option<FlashMessage>) -> Value {
    match flash {
        Some(flash) => json!({
            "flash": {
                "type": flash.name(),
                "msg": flash.msg(),
            }
        }),
        None => json!(null),
    }
}
