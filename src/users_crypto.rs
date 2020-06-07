use scrypt;

use crate::users_models::User;

impl User {
    pub fn new_from_plain(username: String, password: String) -> Option<User> {
        let param = scrypt::ScryptParams::new(15, 8, 1).expect("Default Param Issue");

        // scrypt_simple includes a salt
        let pw_hash = match scrypt::scrypt_simple(&password, &param) {
            Ok(hash) => hash,
            Err(_) => {
                return None;
            }
        };

        let new_user = User { username, pw_hash };

        Some(new_user)
    }

    pub fn verify(user: User, password: String) -> bool {
        scrypt::scrypt_check(&password, &user.pw_hash).is_ok()
    }
}
