#[macro_use]
extern crate structopt;
use structopt::StructOpt;

extern crate kvs;
use kvs::KvStore;

mod cli;
use cli::{KvsCli, KvsCommand};

fn main() {
	let mut kvstore = KvStore::new();

	let opts = KvsCli::from_args();

	match opts.commands {
		KvsCommand::Get { key } => {
			let copy_key = key.clone();
			match kvstore.get(key) {
				Some(value) => println!("Value for key {} is {}", copy_key, value),
				None => println!("No key {} in the store", copy_key),
			}
		}
		KvsCommand::Rm { key } => kvstore.remove(key),
		KvsCommand::Set { key, value } => kvstore.set(key, value),
	};
}
