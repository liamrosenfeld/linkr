use scrypt;

use crate::users_api::NewUser;
use crate::users_models::{InsertableUser, User};

impl InsertableUser {
    pub fn new_from_plain(new_user: NewUser) -> Option<InsertableUser> {
        let param = scrypt::ScryptParams::recommended();

        // scrypt_simple includes a salt
        let pw_hash = match scrypt::scrypt_simple(&new_user.password, &param) {
            Ok(hash) => hash,
            Err(_) => {
                return None;
            }
        };

        let insertable_user = InsertableUser {
            username: new_user.username,
            pw_hash,
            manage_links: new_user.manage_links,
            manage_users: new_user.manage_users,
        };

        Some(insertable_user)
    }
}

impl User {
    pub fn verify(self: &Self, password: &str) -> bool {
        scrypt::scrypt_check(password, &self.pw_hash).is_ok()
    }
}
