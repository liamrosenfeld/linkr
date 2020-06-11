use scrypt;

use crate::users_models::{NewUser, User};

impl NewUser {
    pub fn new_from_plain(username: String, password: String) -> Option<NewUser> {
        let param = scrypt::ScryptParams::recommended();

        // scrypt_simple includes a salt
        let pw_hash = match scrypt::scrypt_simple(&password, &param) {
            Ok(hash) => hash,
            Err(_) => {
                return None;
            }
        };

        let new_user = NewUser { username, pw_hash };

        Some(new_user)
    }

    pub fn verify(user: User, password: String) -> bool {
        scrypt::scrypt_check(&password, &user.pw_hash).is_ok()
    }
}
