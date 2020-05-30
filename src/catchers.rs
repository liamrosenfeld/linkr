use rocket::response::NamedFile;
use std::io;

#[catch(404)]
pub fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("public/404.html")
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
