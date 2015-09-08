use std::io::prelude::*;
use std::fs::File;
use std::iter::Iterator;
use regex::Regex;
use regex::FindMatches;

pub struct TextParser {
	buffer: String,
	re: Regex,
}

impl TextParser {
	pub fn new(input_file: &mut File) -> Result<TextParser, String> {
		let mut parser = TextParser { buffer: String::new(),  re: Regex::new(r"(\w+)\b").unwrap() };
		match input_file.read_to_string(&mut parser.buffer) {
			Ok(_) => {
				Ok(parser)
			},
			Err(e) => { 
				println!("Error reading file {}", e); 
				return Err("Error reading file".to_string()); 
			},
		}
	}

	pub fn word_iter(&mut self) -> WordIterator {
		WordIterator::new(&self.re, &self.buffer)
	}
}

pub struct WordIterator<'r, 't> {
	re: &'r Regex,
	text: &'t String,
	iter: FindMatches<'r, 't>
}

impl<'r, 't> WordIterator<'r, 't> {
	pub fn new(re: &'r Regex, text: &'t String) -> WordIterator<'r, 't> {
		WordIterator { re: re, text: text, iter: re.find_iter(text) }
	}
}

impl<'r, 't> Iterator for WordIterator<'r, 't> {
	type Item = String;
	fn next(&mut self) -> Option<String> {
		unsafe {
			self.iter.next().map(|v| self.text.slice_unchecked(v.0, v.1).to_string())
		}
	}
}

