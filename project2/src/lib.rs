#![deny(missing_docs)]
//! A key value store module

use std::collections::HashMap;
pub use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::Path;


// /// Placeholder documentation
// pub enum Result<T, E> {
// 	/// Placeholder documentation
// 	Ok(T),
// 	/// Placeholder documentation
// 	Err(E),
// }

#[derive(Default, Debug)]
/// Struct for a key-value store of String keys and values
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to
/// disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
/// ```
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

	/// Placeholder documentation
	pub fn open(_path: &Path) -> Result<Self> {
		unimplemented!();
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
	pub fn get(&mut self, key: String) -> Result<Option<String>> {
		Ok(self.storage.get(&key).cloned())
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
	pub fn remove(&mut self, key: String) -> Result<String> {
		match self.storage.remove(&key) {
			Some(value) => Ok(value),
			None => Err(Error::new(ErrorKind::Other, "Placeholder")),
		}
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
	pub fn set(&mut self, key: String, value: String) -> Result<String> {
		Ok(self.storage.insert(key, value).unwrap_or_default())
	}
}
