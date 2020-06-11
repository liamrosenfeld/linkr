use rocket::response::NamedFile;
use std::io;

use crate::db::Conn as DbConn;
use crate::links_models::Link;
use crate::users_models::User;

use rocket::http::{Cookies, Status};
use rocket::request::FlashMessage;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
    mut cookies: Cookies,
) -> Result<Template, Status> {
    // links for table
    let links = match Link::all(&conn) {
        Ok(links) => links,
        Err(_) => return Err(Status::InternalServerError),
    };

    // flash message
    let flash_json = match flash {
        Some(flash) => json!({
            "type": flash.name(),
            "msg": flash.msg(),
        }),
        None => json!(null),
    };

    // user from cookie
    let user_id = cookies
        .get_private("user_id")
        .and_then(|cookie| cookie.value().parse::<i32>().ok());

    let user_info = match user_id {
        Some(id) => match User::get(id, &conn) {
            Ok(user) => json!({
                "name": user.username
            }),
            Err(_) => json!(null),
        },
        None => json!(null),
    };

    // render template
    let context = json!({
        "links": links,
        "flash": flash_json,
        "user": user_info
    });
    Ok(Template::render("index", &context))
}

#[get("/signup")]
pub fn login() -> io::Result<NamedFile> {
    NamedFile::open("public/signup.html")
}
