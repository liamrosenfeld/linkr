use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
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
