extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt; 
use std::env; 

static USAGE: &'static str = "
Usage: phrase_freq [options] [-b BLOCKSIZE] <textfile> <phrase> <output>

Options:
    -b, --block-size BLOCKSIZE       The number of words in a data frame
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_textfile: String,
	arg_phrase: String,
	arg_output: String,
	flag_block_size: u32,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
							.and_then(|d| d.decode())
							.unwrap_or_else(|e| e.exit());

	println!("phrase_freq: {} {} {}", args.arg_textfile, args.arg_phrase, args.arg_output);
}