use crate::shortener::Shortener;
use std::collections::HashMap;

pub struct Repository {
    urls: HashMap<String, String>,
    shortener: Shortener,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            urls: HashMap::new(),
            shortener: Shortener::new(),
        }
    }

    pub fn store(&mut self, url: &str) -> String {
        let id = self.shortener.next_id();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }

    pub fn lookup(&self, id: &str) -> Option<&String> {
        self.urls.get(id)
    }
}
