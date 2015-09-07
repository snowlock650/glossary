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
		Ok(reader) => Some(reader),
		Err(e) => None,
	};
	
	unsafe {
	if let Some(reader) = input {
		for word in reader.words_iter() {
			println!("{}", reader.buffer.slice_unchecked(word.0, word.1));
		}	
	}
	}
}