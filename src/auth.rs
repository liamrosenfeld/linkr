// Copyright (C) 2020 Liam Rosenfeld
//
// This file is part of Linkr (https://github.com/liamrosenfeld/linkr).
//
// Linkr is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Linkr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Linkr. If not, see <http://www.gnu.org/licenses/>.

use diesel::result::Error;
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome, Request};

use crate::db::DbConn;
use crate::models::users::User;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get user_id from cookie
        let user_id = match request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
        {
            Some(id) => id,
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };

        // get database connection
        let conn = request
            .guard::<DbConn>()
            .await
            .expect("database needs to be connected");

        // get user from database with id and block if disabled
        match User::get(user_id, &conn).await {
            Ok(user) => {
                if user.disabled {
                    Outcome::Failure((Status::Unauthorized, ()))
                } else {
                    Outcome::Success(user)
                }
            }
            Err(Error::NotFound) => {
                // user id is not valid, so it mist be removed to prevent an infinite loop
                request.cookies().remove_private(Cookie::named("user_id"));
                Outcome::Failure((Status::Unauthorized, ()))
            }
            Err(_) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}
