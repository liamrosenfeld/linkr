use scrypt;

use crate::users_models::{ User, NewUser };

impl NewUser {
    pub fn new_from_plain(username: String, password: String) -> Option<NewUser> {
        let param = scrypt::ScryptParams::new(15, 8, 1).expect("Default Param Issue");

        // scrypt_simple includes a salt
        let hashed_password = match scrypt::scrypt_simple(&password, &param) {
            Ok(hash) => hash,
            Err(_) => { return None; }
        };
    
        let new_user = NewUser {
            username,
            password: hashed_password
        };
    
        Some(new_user)
    }
}

impl User {
    pub fn verify(user: User, password: String) -> bool {
        scrypt::scrypt_check(&password, &user.password).is_ok()
    }
}
