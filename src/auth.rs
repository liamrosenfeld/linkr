use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct Auth {
    pub user_id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Auth, Self::Error> {
        let auth = request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
            .map(|user_id| Auth { user_id });

        match auth {
            Some(auth) => Outcome::Success(auth),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
