use std::{collections::HashMap, hash::Hash};

/// スコープを表現したHashMap
#[derive(Debug, Clone)]
pub struct Environment<K, V> {
    pub env: HashMap<K, V>,
    pub outer: Option<Box<Environment<K, V>>>,
}

impl<K: Eq + PartialEq + Hash + Clone, V: Clone> Default for Environment<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + PartialEq + Hash + Clone, V: Clone> Environment<K, V> {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            outer: None,
        }
    }

    /// 現在のスコープにアイテムを追加する
    pub fn register_item(&mut self, key: K, value: V) -> Option<V> {
        self.env.insert(key, value)
    }

    pub fn get_item(&self, key: &K) -> Option<&V> {
        if let Some(value) = self.env.get(key) {
            Some(value)
        } else if let Some(outer) = &self.outer {
            outer.get_item(key)
        } else {
            None
        }
    }

    /// スコープを深くする
    pub fn create_deeper_scope(&mut self) -> Environment<K, V> {
        self.outer = Some(Box::new(self.clone()));
        self.env = HashMap::default();

        self.clone()
    }

    /// スコープを浅くする
    /// 一番外側のスコープになった場合はNoneを返す
    pub fn create_shallow_scope(&mut self) -> Option<Environment<K, V>> {
        if let Some(outer) = &self.outer {
            self.env = outer.env.clone();
            self.outer = outer.outer.clone();

            Some(self.clone())
        } else {
            None
        }
    }
}
