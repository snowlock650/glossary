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
	
	let mut input_file = File::open(args.arg_textfile).ok().unwrap();
	
	let input = match TextParser::new(&mut input_file) {
		Ok(parser) => Some(parser),
		Err(e) => {
			println!("Error opening file because {}", e);
			None
		},
	};
	
	match input {
		Some(mut parser) => {
			let mut reader = parser.word_iter();
			loop { 
				match reader.next() {
				None => break,
				Some(word) => println!("{}", word),
				}
			}
		}
		None => (),
	}
}