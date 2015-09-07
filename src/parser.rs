use std::io::prelude::*;
use std::fs::File;
use std::iter::Iterator;
use regex::Regex;
use regex::FindMatches;

pub struct TextParser {
	pub buffer: String,
	re: Regex,
}

impl TextParser {
	pub fn new(input_file: &mut File) -> Result<TextParser, String> {
		let mut parser = TextParser { buffer: String::new(),  re: Regex::new(r"(\w+)\b").unwrap() };
		match input_file.read_to_string(&mut parser.buffer) {
			Ok(_) => Ok(parser),
			Err(e) => { 
				println!("Error reading file {}", e); 
				return Err("Error reading file".to_string()); 
			}
		}
	}
	
	pub fn words_iter(&self) -> FindMatches {
		return self.re.find_iter(&self.buffer);
	}
}
