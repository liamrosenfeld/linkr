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
    let links = match Link::all_for_user(auth.user_id, &conn) {
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
            "name": user.username,
            "manage_links": user.manage_links,
            "manage_users": user.manage_users
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
    Ok(Template::render("pages/index", &context))
}

#[get("/manage_links")]
pub fn manage_links(conn: DbConn, auth: Auth) -> Result<Template, Status> {
    // links for table
    let links = match Link::all(&conn) {
        Ok(links) => links,
        Err(_) => return Err(Status::InternalServerError),
    };

    // user from auth (from cookie)
    let user = User::get(auth.user_id, &conn);
    let user_info = match user {
        Ok(user) => {
            if !user.manage_links {
                return Err(Status::Forbidden);
            }
            json!({
                "name": user.username,
                "manage_links": user.manage_links,
                "manage_users": user.manage_users
            })
        }
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
    };

    // render template
    let context = json!({
        "links": links,
        "user": user_info
    });
    Ok(Template::render("pages/manage_links", &context))
}

#[get("/manage_users")]
pub fn manage_users(conn: DbConn, auth: Auth) -> Result<Template, Status> {
    // links for table
    let users = match User::all(&conn) {
        Ok(users) => users,
        Err(_) => return Err(Status::InternalServerError),
    };

    // user from auth (from cookie)
    let user = User::get(auth.user_id, &conn);
    let user_info = match user {
        Ok(user) => {
            if !user.manage_users {
                return Err(Status::Forbidden);
            }
            json!({
                "name": user.username,
                "manage_links": user.manage_links,
                "manage_users": user.manage_users
            })
        }
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
    };

    // render template
    let context = json!({
        "users": users,
        "user": user_info
    });
    Ok(Template::render("pages/manage_users", &context))
}

#[get("/manage_account")]
pub fn manage_account(
    auth: Auth,
    flash: Option<FlashMessage>,
    conn: DbConn,
) -> Result<Template, Status> {
    // user from auth (from cookie)
    let user = User::get(auth.user_id, &conn);
    let user_info = match user {
        Ok(user) => {
            if !user.manage_users {
                return Err(Status::Forbidden);
            }
            json!({
                "id": user.id,
                "name": user.username,
                "manage_links": user.manage_links,
                "manage_users": user.manage_users
            })
        }
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
    };

    // get flash
    let flash_json = match flash {
        Some(flash) => json!({
            "type": flash.name(),
            "msg": flash.msg(),
        }),
        None => json!(null),
    };

    // render template
    let context = json!({
        "user": user_info,
        "flash": flash_json
    });
    Ok(Template::render("pages/manage_account", &context))
}

#[get("/new_user")]
pub fn new_user(auth: Auth, flash: Option<FlashMessage>, conn: DbConn) -> Result<Template, Status> {
    // check permission
    let user = User::get(auth.user_id, &conn);
    match user {
        Ok(user) => {
            if !user.manage_users {
                return Err(Status::Forbidden);
            }
        }
        Err(Error::NotFound) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(template_with_flash("pages/new_user", &flash))
}

#[get("/login")]
pub fn login(
    flash: Option<FlashMessage>,
    auth: Option<Auth>,
    mut cookies: Cookies,
    conn: DbConn,
) -> Result<Template, Redirect> {
    match auth {
        Some(auth) => match User::get(auth.user_id, &conn) {
            Ok(_) => Err(Redirect::to("/")),
            Err(_) => {
                cookies.remove_private(Cookie::named("user_id"));
                Ok(template_with_flash("pages/login", &flash))
            }
        },
        None => Ok(template_with_flash("pages/login", &flash)),
    }
}

#[get("/setup")]
pub fn setup(flash: Option<FlashMessage>, conn: DbConn) -> Result<Template, Status> {
    match User::all(&conn) {
        Ok(users) => {
            if users.len() == 0 {
                Ok(template_with_flash("pages/setup", &flash))
            } else {
                Err(Status::Conflict)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

fn template_with_flash(template: &'static str, flash: &Option<FlashMessage>) -> Template {
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
