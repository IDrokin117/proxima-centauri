use std::collections::HashMap;

pub(crate) struct Database(HashMap<String, String>);

impl Database {
    pub(crate) fn new_persistence() -> Database {
        let users = HashMap::from([
            ("drokin_ii".to_string(), "o953zY7lnkYMEl5D".to_string()),
            ("admin".to_string(), "12345".to_string()),
        ]);
        Database(users)
    }
    pub(crate) fn is_authenticated(&self, user: &str, password: &str) -> bool {
        self.0.get(user).is_some_and(|pass| pass == password)
    }
}
