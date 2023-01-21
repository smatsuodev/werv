use std::collections::HashMap;
use wervc_object::Object;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(outer: Option<Box<Environment>>) -> Environment {
        Environment {
            store: HashMap::new(),
            outer,
        }
    }

    pub fn set_outer(&mut self, outer: Environment) {
        self.outer = Some(Box::new(outer));
    }

    pub fn outer(self) -> Option<Environment> {
        self.outer.map(|env| *env)
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        if let Some(v) = self.store.get(key) {
            return Some(v);
        }

        if let Some(outer) = &self.outer {
            return outer.get(key);
        }

        None
    }

    pub fn insert(&mut self, key: String, value: Object) -> Option<Object> {
        self.store.insert(key, value)
    }
}
