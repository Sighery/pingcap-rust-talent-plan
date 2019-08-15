use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
	storage: HashMap<String, String>,
}

impl KvStore {
	pub fn new() -> KvStore {
		KvStore { storage: HashMap::new() }
	}

	pub fn set(&mut self, key: String, value: String) {
		// self.storage.insert(key, value);
		panic!("unimplemented");
	}

	pub fn get(&mut self, key: String) -> Option<String> {
		// match self.storage.get(&key) {
		// 	None => None,
		// 	Some(value) => Some(value.clone()),
		// }
		panic!("unimplemented");
	}

	pub fn remove(&mut self, key: String) {
		// self.storage.remove(&key);
		panic!("unimplemented");
	}
}
