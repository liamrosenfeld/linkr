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

use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt
};
use rand_core::OsRng;
use crate::models::users::{InsertableUser, User};
use crate::routes::users::NewUser;

impl InsertableUser {
    pub fn new_from_plain(new_user: NewUser, orig: bool) -> InsertableUser {
        InsertableUser {
            username: new_user.username,
            pw_hash: encrypt_pw(&new_user.password),
            orig,
            manage_links: new_user.manage_links,
            manage_users: new_user.manage_users,
        }
    }
}

pub fn encrypt_pw(pw: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Scrypt.hash_password_simple(pw.as_bytes(), salt.as_ref()).unwrap().to_string()
}

impl User {
    pub fn verify(self: &Self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.pw_hash).unwrap();
        Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}
