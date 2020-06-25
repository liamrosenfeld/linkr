use scrypt;

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
    let param = scrypt::ScryptParams::recommended();
    scrypt::scrypt_simple(pw, &param).expect("System is misconfigured so OsRng does not work")
}

impl User {
    pub fn verify(self: &Self, password: &str) -> bool {
        scrypt::scrypt_check(password, &self.pw_hash).is_ok()
    }
}