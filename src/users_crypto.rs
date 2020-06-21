use scrypt;

use crate::users_api::NewUser;
use crate::users_models::{InsertableUser, User};

impl InsertableUser {
    pub fn new_from_plain(new_user: NewUser) -> InsertableUser {
        let pw_hash = encrypt_pw(&new_user.password);

        InsertableUser {
            username: new_user.username,
            pw_hash,
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
