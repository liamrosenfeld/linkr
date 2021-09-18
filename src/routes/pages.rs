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

use crate::db::DbConn;
use crate::models::links::Link;
use crate::models::users::User;

use diesel::result::Error;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket_dyn_templates::Template;

#[get("/<short>", rank = 3)]
pub async fn link(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get(short.to_string(), &conn).await {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(Error::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
pub async fn index(
    user: User,
    flash: Option<FlashMessage<'_>>,
    conn: DbConn,
) -> Result<Template, Status> {
    // links for table
    let links = match Link::all_for_user(user.id, &conn).await {
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
pub async fn manage_links(
    user: User,
    flash: Option<FlashMessage<'_>>,
    conn: DbConn,
) -> Result<Template, Status> {
    // check permission
    if !user.manage_links {
        return Err(Status::Forbidden);
    }

    // links for table
    let links = match Link::all(&conn).await {
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
pub async fn manage_users(
    user: User,
    flash: Option<FlashMessage<'_>>,
    conn: DbConn,
) -> Result<Template, Status> {
    // check permission
    if !user.manage_users {
        return Err(Status::Forbidden);
    }

    // links for table
    let users = match User::all(&conn).await {
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
pub async fn manage_account(
    user: User,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Status> {
    let context = json!({
        "user": user,
        "flash": flash_json(&flash)
    });
    Ok(Template::render("pages/manage_account", &context))
}

#[get("/new_user")]
pub async fn new_user(user: User, flash: Option<FlashMessage<'_>>) -> Result<Template, Status> {
    // check permission
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
pub async fn login(
    user: Option<User>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Redirect> {
    match user {
        Some(_) => Err(Redirect::to("/")),
        None => {
            let context = just_flash_context(&flash);
            Ok(Template::render("pages/login", &context))
        }
    }
}

#[get("/setup")]
pub async fn setup(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Result<Template, Status> {
    match User::count(&conn).await {
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

fn flash_json(flash: &Option<FlashMessage<'_>>) -> Value {
    match flash {
        Some(flash) => json!({
            "type": flash.kind(),
            "msg": flash.message(),
        }),
        None => json!(null),
    }
}

fn just_flash_context(flash: &Option<FlashMessage<'_>>) -> Value {
    match flash {
        Some(flash) => json!({
            "flash": {
                "type": flash.kind(),
                "msg": flash.message(),
            }
        }),
        None => json!(null),
    }
}
