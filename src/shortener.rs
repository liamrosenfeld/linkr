use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub struct Shortener {
    id: u64,
}

impl Shortener {
    pub fn new() -> Shortener {
        Shortener { id: 0 }
    }

    pub fn next_id(&mut self) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(self.id);
        self.id += 1;
        format!("{}", hasher.finish())
    }
}
