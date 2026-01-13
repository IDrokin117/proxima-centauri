use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;

pub struct Database(HashMap<String, String>);

impl Database {
    pub fn new_persistence() -> Database {
        let users = HashMap::from([
            ("drokin_ii".to_string(), "o953zY7lnkYMEl5D".to_string()),
            ("admin".to_string(), "12345".to_string()),
        ]);
        Database(users)
    }

    pub fn is_authenticated(&self, user: &str, password: &str) -> bool {
        self.0.get(user).is_some_and(|pass| pass == password)
    }
}

pub fn parse_proxy_auth_token(token: &[u8]) -> Result<(String, String)> {
    let token_str = std::str::from_utf8(token)?;

    let encoded_cred = token_str
        .strip_prefix("Basic ")
        .ok_or_else(|| anyhow!("Invalid auth format: expected 'Basic ...'"))?;

    let decoded = general_purpose::STANDARD.decode(encoded_cred)?;
    let credentials = String::from_utf8(decoded)?;

    credentials
        .split_once(':')
        .map(|(u, p)| (u.to_string(), p.to_string()))
        .ok_or_else(|| anyhow!("Invalid credentials format: expected 'user:password'"))
}

pub fn authenticate(user: &str, password: &str, database: &Database) -> bool {
    database.is_authenticated(user, password)
}
