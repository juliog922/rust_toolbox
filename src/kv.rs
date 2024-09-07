use std::collections::HashMap;

/// A simple in-memory key-value store.
///
/// `KvStore` allows you to store, retrieve, and delete key-value pairs
/// using an in-memory `HashMap`. Keys and values are represented as `String`.
#[derive(Debug)]
pub struct KvStore {
    /// Internal storage for key-value pairs.
    ///
    /// Uses a `HashMap` to store the keys and values, where both the key and value are `String`.
    store: HashMap<String, String>,
}

impl Default for KvStore {
    /// Creates a default instance of `KvStore`.
    ///
    /// This implementation uses the `new()` function to create a new, empty `KvStore`.
    /// It allows the `KvStore` to be initialized using `KvStore::default()` or with
    /// the `#[derive(Default)]` attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_toolbox::KvStore;
    /// let store: KvStore = KvStore::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
impl KvStore {
    /// Creates a new, empty `KvStore`.
    ///
    /// This function initializes the internal `HashMap` to store key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_toolbox::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the store.
    ///
    /// If the key already exists, the value is updated.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key.
    /// * `value` - A `String` representing the value to be associated with the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_toolbox::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Retrieves the value associated with the given key.
    ///
    /// If the key exists in the store, it returns `Some(String)` containing the value.
    /// If the key does not exist, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key.
    ///
    /// # Returns
    ///
    /// * `Some(String)` if the key is found.
    /// * `None` if the key is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_toolbox::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Removes a key-value pair from the store.
    ///
    /// If the key exists, the key-value pair is removed. If the key does not exist, the store is unchanged.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key to be removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_toolbox::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// store.remove("key1".to_string());
    /// assert_eq!(store.get("key1".to_string()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
