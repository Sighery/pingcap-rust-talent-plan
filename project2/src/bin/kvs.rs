#[macro_use]
extern crate structopt;
use structopt::StructOpt;

use kvs::Result;

mod cli;
use cli::{KvsCli, KvsCommand};

fn main() -> Result<()> {
	let opts = KvsCli::from_args();

	match opts.commands {
		KvsCommand::Get { .. } => panic!("unimplemented"),
		KvsCommand::Rm { .. } => panic!("unimplemented"),
		KvsCommand::Set { .. } => panic!("unimplemented"),
	};
}
