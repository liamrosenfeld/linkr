use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/favicon.ico")]
pub fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("static/favicon.ico")
}

#[get("/resource/<file..>", rank = 2)]
pub fn all_resources(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
