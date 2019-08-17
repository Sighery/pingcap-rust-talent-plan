#[derive(Debug, StructOpt)]
#[structopt(name = "kvs")]
/// CLI Interface for the kvs command
pub struct KvsCli {
	#[structopt(subcommand)]
	pub commands: KvsCommand,
}

#[derive(Debug, StructOpt)]
pub enum KvsCommand {
	#[structopt(name = "get")]
	/// Get the String value of a given String key
	Get {
		#[structopt(name = "KEY")]
		key: String,
	},

	#[structopt(name = "rm")]
	/// Remove a given String key
	Rm {
		#[structopt(name = "KEY")]
		key: String,
	},

	#[structopt(name = "set")]
	/// Set the String value of a String key
	Set {
		#[structopt(name = "KEY")]
		key: String,
		#[structopt(name = "VALUE")]
		value: String,
	},
}
