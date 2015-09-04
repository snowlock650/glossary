extern crate rustc_serialize;
extern crate docopt;
extern crate indexerlib;

use docopt::Docopt; 
use std::fs::File;
use indexerlib::parser::TextParser;

static USAGE: &'static str = "
Usage: phrase_freq [-b BLOCKSIZE] <textfile> <phrase> <output>

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
	
	let input_file = File::open(args.arg_textfile).ok().unwrap();
	let mut input_reader = TextParser::new(input_file);
	loop {
		match input_reader.next() {
			None => break,
			Some(c) => println!("{}", c)
		}
	}
}