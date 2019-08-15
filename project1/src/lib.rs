#![deny(missing_docs)]
//! A key value store module

use std::collections::HashMap;

#[derive(Default, Debug)]
/// Struct for a key-value store of String keys and values
pub struct KvStore {
	storage: HashMap<String, String>,
}

impl KvStore {
	/// Constructs a new `KvStore`
	///
	/// # Examples
	///
	/// ```
	/// use kvs::KvStore;
	///
	/// let mut kvstore = KvStore::new();
	/// ```
	pub fn new() -> Self {
		Self {
			storage: HashMap::new(),
		}
	}

	/// Retrieve a value from the store, from a key String
	///
	/// # Examples
	///
	/// ```
	/// use kvs::KvStore;
	///
	/// let mut kvstore = KvStore::new();
	/// match kvstore.get("test_key".to_string()) {
	/// 	Some(value) => println!("Retrieved value {}", value),
	/// 	None => println!("Key doesn't exist"),
	/// }
	/// ```
	pub fn get(&mut self, key: String) -> Option<String> {
		match self.storage.get(&key) {
			None => None,
			Some(value) => Some(value.clone()),
		}
	}

	/// Remove a given key from the store
	///
	/// # Examples
	///
	/// ```
	/// use kvs::KvStore;
	///
	/// let mut kvstore = KvStore::new();
	/// kvstore.remove("test_key".to_string());
	/// ```
	pub fn remove(&mut self, key: String) {
		self.storage.remove(&key);
	}

	/// Set a given key and value Strings in the store
	///
	/// # Examples
	///
	/// ```
	/// use kvs::KvStore;
	///
	/// let mut kvstore = KvStore::new();
	/// kvstore.set("test_key".to_string(), "test_value".to_string());
	/// ```
	pub fn set(&mut self, key: String, value: String) {
		self.storage.insert(key, value);
	}
}
