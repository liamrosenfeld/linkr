use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

use crate::db::Conn as DbConn;
use crate::links_models::Link;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Result<Template, Status> {
    let links = match Link::all(&conn) {
        Ok(links) => links,
        Err(_) => return Err(Status::InternalServerError),
    };

    let flash_json = match flash {
        Some(flash) => json!({
            "type": flash.name(),
            "msg": flash.msg(),
        }),
        None => json!(null),
    };

    let context = json!({
        "links": links,
        "flash": flash_json,
    });
    Ok(Template::render("index", &context))
}

#[get("/signup")]
pub fn login() -> io::Result<NamedFile> {
    NamedFile::open("public/signup.html")
}

#[get("/favicon.ico")]
pub fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("public/favicon.ico")
}

#[get("/resource/<file..>", rank = 2)]
pub fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}
