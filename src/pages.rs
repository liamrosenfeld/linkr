use crate::auth::Auth;
use crate::db::Conn as DbConn;
use crate::links_models::Link;
use crate::users_models::User;

use diesel::result::Error;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
    auth: Auth,
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

    // user from auth (from cookie)
    let user = User::get(auth.user_id, &conn);

    let user_info = match user {
        Ok(user) => json!({
            "name": user.username
        }),
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
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
pub fn signup(flash: Option<FlashMessage<'_, '_>>) -> Template {
    template_with_flash("signup", flash)
}

#[get("/login")]
pub fn login(
    auth: Option<Auth>,
    mut cookies: Cookies,
    flash: Option<FlashMessage<'_, '_>>,
    conn: DbConn,
) -> Result<Template, Redirect> {
    match auth {
        Some(auth) => match User::get(auth.user_id, &conn) {
            Ok(_) => Err(Redirect::to("/")),
            Err(_) => {
                cookies.remove_private(Cookie::named("user_id"));
                Ok(template_with_flash("login", flash))
            }
        },
        None => Ok(template_with_flash("login", flash)),
    }
}

fn template_with_flash(template: &'static str, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let flash_json = match flash {
        Some(flash) => json!({
            "type": flash.name(),
            "msg": flash.msg(),
        }),
        None => json!(null),
    };

    let context = json!({
        "flash": flash_json,
    });

    Template::render(template, &context)
}
