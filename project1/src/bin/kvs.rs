#[macro_use]
extern crate clap;
use clap::App;

extern crate kvs;
use kvs::KvStore;

fn main() {
	let mut kvstore = KvStore::new();

	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml)
		.about(crate_description!())
		.author(crate_authors!("\n"))
		.version(crate_version!())
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("get") {
		kvstore.get(matches.value_of("KEY").unwrap().to_string());
	} else if let Some(matches) = matches.subcommand_matches("rm") {
		kvstore.get(matches.value_of("KEY").unwrap().to_string());
	} else if let Some(matches) = matches.subcommand_matches("set") {
		let key = matches.value_of("KEY").unwrap().to_string();
		let value = matches.value_of("VALUE").unwrap().to_string();
		kvstore.set(key, value);
	} else {
		panic!("Some subcommand must be used");
	}
}
