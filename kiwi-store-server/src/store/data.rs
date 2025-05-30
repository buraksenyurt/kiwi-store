use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
#[allow(dead_code)]
/// Represents a simple in-memory key-value store
pub struct DataStore {
    context: Arc<Mutex<HashMap<String, String>>>,
}

#[allow(dead_code)]
impl DataStore {
    /// Creates a new instance of `DataStore`
    ///
    /// # Returns
    /// Returns a new `DataStore` instance with an empty context.
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::store::data::DataStore;
    ///
    /// let store = DataStore::new();
    /// ```
    pub fn new() -> Self {
        DataStore {
            context: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a key-value pair in the store.
    ///
    /// # Arguments
    /// * `key` - Key variable
    /// * `value` - Value variable
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::store::data::DataStore;
    ///
    /// let store = DataStore::new();
    /// store.set("UseHttps", "Off").await;
    /// ```
    pub async fn set(&self, key: &str, value: &str) {
        let mut context = self.context.lock().await;
        context.insert(key.to_string(), value.to_string());
    }

    /// Remove a key from the store.
    ///
    /// # Arguments
    /// * `key` - Key variable
    ///
    /// # Returns
    /// Returns `true` if the key was removed, `false` if it did not exist.
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::store::data::DataStore;
    ///
    /// let store = DataStore::new();
    /// store.set("UseHttps", "Off").await;
    /// let removed = store.remove("UseHttps").await;
    ///
    /// assert!(removed);
    /// ```
    pub async fn remove(&self, key: &str) -> bool {
        let mut context = self.context.lock().await;
        context.remove(key).is_some()
    }

    /// Get the value associated with a key.
    ///
    /// # Arguments
    /// * `key` - Key variable
    ///
    /// # Returns
    /// Returns `Some(value)` if the key exists, or `None` if it does not.
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::store::data::DataStore;
    ///
    /// let store = DataStore::new();
    /// store.set("UseHttps", "Off").await;
    /// let value = store.get("UseHttps").await;
    ///
    /// assert_eq!(value, Some("Off".to_string()));
    /// ```
    pub async fn get(&self, key: &str) -> Option<String> {
        let context = self.context.lock().await;
        context.get(key).cloned()
    }

    /// Get all keys in the store.
    ///
    /// # Returns
    /// Returns a vector of keys present in the store.
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::store::data::DataStore;
    ///
    /// let store = DataStore::new();
    /// store.set("UseHttps", "Off").await;
    /// let keys = store.keys().await;
    ///
    /// assert!(keys.contains(&"UseHttps".to_string()));
    /// ```
    pub async fn keys(&self) -> Vec<String> {
        let context = self.context.lock().await;
        context.keys().cloned().collect()
    }

    /// Get statistics about the store.
    /// This includes the number of keys and the total size of the store in a human-readable format.
    ///
    /// # Returns
    /// Returns a string containing the number of keys and the size of the store.
    ///
    /// # Example
    /// ```rust
    /// use kiwi_store_server::store::data::DataStore;
    /// let store = DataStore::new();
    /// store.set("UseHttps", "Off").await;
    /// let stats = store.stats().await;
    /// assert!(stats.contains("Keys: 1"));
    /// assert!(stats.contains("Size:"));
    /// ```
    pub async fn stats(&self) -> String {
        let context = self.context.lock().await;
        if context.is_empty() {
            return "Keys(0), Size(0 B)".to_string();
        }

        let key_count = context.len();
        if key_count == 0 {
            return "Keys(0), Size(0 B)".to_string();
        }
        let total_bytes: usize = context
            .values()
            .map(|v| v.len())
            .map(|len| len * std::mem::size_of::<char>())
            .sum();

        let size = if total_bytes < 1024 {
            format!("{} B", total_bytes)
        } else if total_bytes < 1024 * 1024 {
            format!("{:.2} KB", total_bytes as f64 / 1024.0)
        } else {
            format!("{:.2} MB", total_bytes as f64 / (1024.0 * 1024.0))
        };

        format!("Keys({}), Size({})", key_count, size)
    }

    pub async fn is_empty(&self) -> bool {
        let context = self.context.lock().await;
        context.is_empty()
    }
}
