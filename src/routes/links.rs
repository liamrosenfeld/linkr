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
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use chrono::Utc;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

use crate::db::Conn as DbConn;
use crate::models::links::Link;
use crate::models::users::User;

#[derive(FromForm)]
pub struct NewLink {
    short: String,
    long: String,
    notes: String,
}

const RESERVED_LINKS: [&str; 9] = [
    "",
    "api",
    "login",
    "resource",
    "new_user",
    "setup",
    "manage_links",
    "manage_users",
    "manage_account",
];

#[post("/new", data = "<link_form>")]
pub fn shorten(
    conn: DbConn,
    link_form: Form<NewLink>,
    user: User,
) -> Result<Flash<Redirect>, Status> {
    let new_link = link_form.into_inner();

    // check if the short is alphanumeric
    if !new_link.short.chars().all(char::is_alphanumeric) {
        return Ok(Flash::error(
            Redirect::to("/"),
            "Shorts can only contain alphanumeric characters",
        ));
    }

    // check if the short is reserved by this site
    if RESERVED_LINKS
        .iter()
        .any(|&reserved| reserved == new_link.short)
    {
        return Ok(Flash::error(
            Redirect::to("/"),
            "That short is reserved by this website",
        ));
    }

    // check that the long is a valid url
    let prefix_correct =
        new_link.long.starts_with("http://") || new_link.long.starts_with("https://");
    if !prefix_correct {
        return Ok(Flash::error(
            Redirect::to("/"),
            "That long does not begin with https:// or http://",
        ));
    }

    // create link to insert
    let link = Link {
        short: new_link.short,
        long: new_link.long,
        notes: new_link.notes,
        created_at: Utc::now(),
        created_by: user.id,
    };

    // send database request and respond accordingly
    match Link::insert(link, &conn) {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Link created!")),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(Flash::error(
            Redirect::to("/"),
            "That short is already in use",
        )),
        Err(_) => Ok(Flash::error(
            Redirect::to("/"),
            "There was an internal server error",
        )),
    }
}

#[derive(FromForm)]
pub struct Short {
    short: String,
}

#[post("/delete", data = "<short_form>")]
pub fn delete(conn: DbConn, short_form: Form<Short>, user: User) -> Status {
    let short = short_form.into_inner().short;

    match check_can_edit(&user, &short, &conn) {
        Ok(_) => {}
        Err(err) => return err,
    }

    match Link::delete(&short, &conn) {
        Ok(_) => Status::Ok,
        Err(Error::NotFound) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct UpdateLong {
    short: String,
    long: String,
}

#[post("/update", data = "<update_form>")]
pub fn update(conn: DbConn, update_form: Form<UpdateLong>, user: User) -> Status {
    let update = update_form.into_inner();

    match check_can_edit(&user, &update.short, &conn) {
        Ok(_) => {}
        Err(err) => return err,
    }

    match Link::update(&update.short, &update.long, &conn) {
        Ok(_) => Status::Ok,
        Err(Error::NotFound) => Status::NotFound,
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Status::Conflict,
        Err(_) => Status::InternalServerError,
    }
}

fn check_can_edit(user: &User, short: &str, conn: &DbConn) -> Result<(), Status> {
    if !user.manage_links {
        let link_user = match Link::get(&short, &conn) {
            Ok(link) => link.created_by,
            Err(Error::NotFound) => return Err(Status::NotFound),
            Err(_) => return Err(Status::InternalServerError),
        };

        if link_user != user.id {
            return Err(Status::Forbidden);
        }
    }

    Ok(())
}
