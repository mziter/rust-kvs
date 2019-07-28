#![deny(missing_docs)]
//! Key-Value Store
//!
//! For persistant storage and quick lookup of key value pairs.

use std::collections::HashMap;

/// Key-Value store for storing values associated with provided keys for fast lookup
pub struct KvStore {
    keys: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new instance of `KvStore`.
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    /// Get the value associated with the provided key from the store.
    ///
    /// Returns Option::Some(String) when a value is found for the key
    /// Returns Option::None when a value is not found for the key
    pub fn get(&self, key: String) -> Option<String> {
        self.keys.get(&key).map(|s| s.to_owned())
    }

    /// Sets the provided value for the provided key in the store.
    pub fn set(&mut self, key: String, value: String) {
        self.keys.insert(key, value);
    }

    /// Removes the provided key (any value) from the store.
    pub fn remove(&mut self, key: String) {
        self.keys.remove(&key);
    }
}
