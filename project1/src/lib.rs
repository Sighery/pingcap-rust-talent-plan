use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct KvStore {
	storage: HashMap<String, String>,
}

impl KvStore {
	pub fn new() -> Self {
		Self { storage: HashMap::new() }
	}

	pub fn set(&mut self, key: String, value: String) {
		self.storage.insert(key, value);
	}

	pub fn get(&mut self, key: String) -> Option<String> {
		match self.storage.get(&key) {
			None => None,
			Some(value) => Some(value.clone()),
		}
	}

	pub fn remove(&mut self, key: String) {
		self.storage.remove(&key);
	}
}
