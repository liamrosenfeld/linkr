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
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Redirect;
use std::path::PathBuf;

/* ----------------------------- link forwarding ----------------------------- */

#[get("/<short>", rank = 3)]
pub async fn link(conn: DbConn, short: String) -> Result<Redirect, Status> {
    match Link::get(short.to_string(), &conn).await {
        Ok(link) => Ok(Redirect::permanent(link.long)),
        Err(Error::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

/* ----------------------------- helpers ----------------------------- */

// debug mode -> serve relative to crate root
#[cfg(debug_assertions)]
const FRONTEND_PATH: &str = rocket::fs::relative!("frontend/build/");

// running in container -> serve from absolute pat
#[cfg(not(debug_assertions))]
const FRONTEND_PATH: &str = "/app/static";

async fn open_page_index(page: &'static str) -> NamedFile {
    let dir = PathBuf::from(format!("{}{}{}", FRONTEND_PATH, page, "/index.html"));
    NamedFile::open(dir)
        .await
        .expect(&format!("{} page missing", page))
}

/* ----------------------------- resources ----------------------------- */
#[get("/resource/<file..>", rank = 2)]
pub async fn resources(file: PathBuf) -> Option<NamedFile> {
    let mut dir = PathBuf::from(format!("{}{}", FRONTEND_PATH, "/resource"));
    dir.push(file);
    NamedFile::open(dir).await.ok()
}

#[get("/_app/<file..>", rank = 2)]
pub async fn svelte_gen(file: PathBuf) -> Option<NamedFile> {
    let mut dir = PathBuf::from(format!("{}{}", FRONTEND_PATH, "/_app"));
    dir.push(file);
    NamedFile::open(dir).await.ok()
}

/* ----------------------------- pages ----------------------------- */
#[get("/")]
pub async fn index(_user: User) -> NamedFile {
    let dir = PathBuf::from(format!("{}{}", FRONTEND_PATH, "/index.html"));
    NamedFile::open(dir).await.expect("index.html missing")
}

#[get("/manage_links")]
pub async fn manage_links(user: User) -> Result<NamedFile, Status> {
    if !user.manage_links {
        return Err(Status::Forbidden);
    }
    Ok(open_page_index("manage_links").await)
}

#[get("/manage_users")]
pub async fn manage_users(user: User) -> Result<NamedFile, Status> {
    if !user.manage_users {
        return Err(Status::Forbidden);
    }
    Ok(open_page_index("manage_users").await)
}

#[get("/manage_account")]
pub async fn manage_account(_user: User) -> NamedFile {
    open_page_index("manage_account").await
}

#[get("/new_user")]
pub async fn new_user(user: User) -> Result<NamedFile, Status> {
    if !user.manage_users {
        return Err(Status::Forbidden);
    }
    Ok(open_page_index("new_user").await)
}

#[get("/login")]
pub async fn login(user: Option<User>) -> Result<NamedFile, Redirect> {
    match user {
        Some(_) => Err(Redirect::to("/")),
        None => Ok(open_page_index("login").await),
    }
}

#[get("/setup")]
pub async fn setup(conn: DbConn) -> Result<NamedFile, Status> {
    match User::count(&conn).await {
        Ok(count) => {
            if count == 0 {
                Ok(open_page_index("setup").await)
            } else {
                Err(Status::Forbidden)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/* ----------------------------- catchers ----------------------------- */
#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}

#[catch(403)]
pub async fn forbidden() -> NamedFile {
    open_page_index("403").await
}

#[catch(404)]
pub async fn not_found() -> NamedFile {
    open_page_index("404").await
}

#[catch(500)]
pub async fn internal_error() -> NamedFile {
    open_page_index("500").await
}

#[catch(503)]
pub async fn service_unavailable() -> NamedFile {
    open_page_index("503").await
}
