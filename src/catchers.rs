use rocket::response::{NamedFile, Redirect};
use std::io;

#[catch(404)]
pub fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("static/404.html")
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(403)]
pub fn forbidden() -> &'static str {
    "Your user does not have permission to do that."
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}
