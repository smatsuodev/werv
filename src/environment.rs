use std::collections::HashMap;

use crate::object::Object;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<&Object> {
        self.store.get(&key)
    }

    pub fn insert(&mut self, key: String, value: Object) -> Option<Object> {
        self.store.insert(key, value)
    }
}
