use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorContext {
    code: u32,
    message: &'static str,
    details: &'static str,
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}

#[catch(403)]
pub fn forbidden() -> Template {
    let context = ErrorContext {
        code: 403,
        message: "Your user does not have permission to do that.",
        details: "If you feel that is in error, contact who set up your account",
    };
    Template::render("pages/error", json!(context))
}

#[catch(404)]
pub fn not_found() -> Template {
    let context = ErrorContext {
        code: 404,
        message: "That link does not exist",
        details: "Please check that you entered it correctly",
    };
    Template::render("pages/error", json!(context))
}

#[catch(500)]
pub fn internal_error() -> Template {
    let context = ErrorContext {
        code: 500,
        message: "There was an internal server error",
        details: "Looks like we messed up, please report an error",
    };
    Template::render("pages/error", json!(context))
}

#[catch(503)]
pub fn service_unavailable() -> Template {
    let context = ErrorContext {
        code: 503,
        message: "Linkr is temporarily unavailable",
        details: "Please try again shortly",
    };
    Template::render("pages/error", json!(context))
}
