extern crate rustc_serialize;
extern crate docopt;
extern crate indexerlib;
extern crate itertools;

use docopt::Docopt; 
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use indexerlib::parser::TextParser;
use indexerlib::counter::WordCountByFrame;
use itertools::Itertools;

static USAGE: &'static str = "
Usage: phrase_freq [-b BLOCKSIZE] <textfile> <output>

Options:
    -b, --block-size BLOCKSIZE       The number of words in a data frame
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_textfile: String,
	arg_output: String,
	flag_block_size: usize,
}

fn count_words(input_file: &mut File, output_file: &mut File, est_frame_cnt: usize, frame_size: usize) {
	let input = match TextParser::new(input_file) {
		Ok(parser) => Some(parser),
		Err(e) => {
			println!("Error opening file because {}", e);
			None
		},
	};
	
	if let Some(mut parser) = input {
		let mut counts = WordCountByFrame::new(est_frame_cnt, frame_size);
		for word in parser.word_iter() { 
			counts.incr_word(word.to_lowercase());
		}

		let counter = counts.get_word_counts();

		let mut writer = BufWriter::new(output_file);
		for (key, array) in counter.iter() {
			let mut line: String = String::new();
			let subline = array.iter().join("\t");
			line.push_str(key);
			line.push('\t');
			line.push_str(&subline);
			line.push('\n');
			writer.write(line.as_bytes()).unwrap();
		}
	}
}

fn main() {
	let args: Args = Docopt::new(USAGE)
							.and_then(|d| d.decode())
							.unwrap_or_else(|e| e.exit());
	

	println!("phrase_freq: {} {}", args.arg_textfile, args.arg_output);
	
	//If the files failed to open, then panic
	let mut input_file = File::open(args.arg_textfile).ok().unwrap();
	let mut output_file = File::create(args.arg_output).ok().unwrap();
	
	let metadata = input_file.metadata().ok().unwrap();
	let est_frame_cnt: usize = metadata.len() as usize / args.flag_block_size;
	
	count_words(&mut input_file, &mut output_file, est_frame_cnt as usize, args.flag_block_size as usize);
	
}