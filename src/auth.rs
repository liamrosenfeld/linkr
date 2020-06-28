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
