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

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize};

use chrono::Utc;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::db::DbConn;
use crate::models::links::Link;
use crate::models::users::User;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewLink {
    short: String,
    long: String,
    notes: String,
}

const RESERVED_LINKS: [&'static str; 10] = [
    "",
    "_app",
    "api",
    "login",
    "resource",
    "new_user",
    "setup",
    "manage_links",
    "manage_users",
    "manage_account",
];

#[post("/new", data = "<link_json>")]
pub async fn shorten(
    conn: DbConn,
    link_json: Json<NewLink>,
    user: User,
) -> Result<status::Created<()>, status::Custom<&'static str>> {
    let new_link = link_json.into_inner();

    // check if the short is alphanumeric
    if !new_link.short.chars().all(char::is_alphanumeric) {
        return Err(status::Custom(
            Status::BadRequest,
            "Shorts can only contain alphanumeric characters",
        ));
    }

    // check if the short is reserved by this site
    if RESERVED_LINKS
        .iter()
        .any(|&reserved| reserved == new_link.short)
    {
        return Err(status::Custom(
            Status::BadRequest,
            "That short is reserved by this website",
        ));
    }

    // check that the long is a valid url
    let prefix_correct =
        new_link.long.starts_with("http://") || new_link.long.starts_with("https://");
    if !prefix_correct {
        return Err(status::Custom(
            Status::BadRequest,
            "That long does not begin with https:// or http://",
        ));
    }

    // create link to insert
    let link = Link {
        short: new_link.short,
        long: new_link.long.clone(),
        notes: new_link.notes,
        created_at: Utc::now(),
        created_by: user.id,
    };

    // send database request and respond accordingly
    match Link::insert(link, &conn).await {
        Ok(_) => Ok(status::Created::new(new_link.long)),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(status::Custom(
            Status::Conflict,
            "That short is already in use",
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "There was an internal server error",
        )),
    }
}

#[get("/all")]
pub async fn get_all(
    conn: DbConn,
    user: User,
) -> Result<status::Custom<Json<Vec<Link>>>, status::Custom<()>> {
    // check permission
    if !user.manage_links {
        return Err(status::Custom(Status::Forbidden, ()));
    }

    // get from database
    match Link::all(&conn).await {
        Ok(links) => Ok(status::Custom(Status::Accepted, Json(links))),
        Err(_) => Err(status::Custom(Status::InternalServerError, ())),
    }
}

#[get("/for_user")]
pub async fn get_for_user(
    conn: DbConn,
    user: User,
) -> Result<status::Custom<Json<Vec<Link>>>, status::Custom<()>> {
    // get from database
    match Link::all_for_user(user.id, &conn).await {
        Ok(links) => Ok(status::Custom(Status::Accepted, Json(links))),
        Err(_) => Err(status::Custom(Status::InternalServerError, ())),
    }
}

#[delete("/delete", data = "<short>")]
pub async fn delete(conn: DbConn, short: String, user: User) -> status::Custom<()> {
    // let short = short_form.into_inner().short;

    match check_can_edit(&user, &short, &conn).await {
        Ok(_) => {}
        Err(err) => return err,
    }

    match Link::delete(short.to_string(), &conn).await {
        Ok(_) => status::Custom(Status::Ok, ()),
        Err(Error::NotFound) => status::Custom(Status::NotFound, ()),
        Err(_) => status::Custom(Status::InternalServerError, ()),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateLong {
    short: String,
    long: String,
}

#[patch("/update", data = "<update_form>")]
pub async fn update(conn: DbConn, update_form: Json<UpdateLong>, user: User) -> status::Custom<()> {
    let update = update_form.into_inner();

    match check_can_edit(&user, &update.short, &conn).await {
        Ok(_) => {}
        Err(err) => return err,
    }

    match Link::update(update.short.to_string(), update.long.to_string(), &conn).await {
        Ok(_) => status::Custom(Status::Ok, ()),
        Err(Error::NotFound) => status::Custom(Status::NotFound, ()),
        Err(_) => status::Custom(Status::InternalServerError, ()),
    }
}

async fn check_can_edit(user: &User, short: &str, conn: &DbConn) -> Result<(), status::Custom<()>> {
    if !user.manage_links {
        let link_user = match Link::get(short.to_string(), &conn).await {
            Ok(link) => link.created_by,
            Err(Error::NotFound) => return Err(status::Custom(Status::NotFound, ())),
            Err(_) => return Err(status::Custom(Status::InternalServerError, ())),
        };

        if link_user != user.id {
            return Err(status::Custom(Status::Forbidden, ()));
        }
    }

    Ok(())
}
