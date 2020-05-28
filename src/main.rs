#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use std::sync::RwLock;

mod repository;
mod shortener;
use repository::Repository;

/* --------------------------------- lookup --------------------------------- */

#[get("/<id>")]
fn lookup(repo: State<RwLock<Repository>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().lookup(&id) {
        Some(url) => Ok(Redirect::permanent(format!("{}", url))),
        _ => Err("Requested ID was not found."),
    }
}

/* ----------------------------------- api ---------------------------------- */

#[derive(FromForm)]
struct Url<'f> {
    url: &'f RawStr,
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<Repository>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url = format!("{}", url_form.url);
    let mut repo = repo.write().unwrap();
    let id = repo.store(&url);
    Ok(id.to_string())
}

/* -------------------------------- dashboard ------------------------------- */

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/* ---------------------------------- start --------------------------------- */

fn main() {
    rocket::ignite()
        .manage(RwLock::new(Repository::new()))
        .mount("/", routes![lookup, index])
        .mount("/api", routes![shorten])
        .launch();
}
