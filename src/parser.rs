use std::io::prelude::*;
use std::io::Bytes;
use std::fs::File;
use std::iter::Iterator;

pub struct TextParser {
	buffer: Bytes<File>,
}

impl TextParser {
	pub fn new(input_file: File) -> TextParser {
		TextParser { buffer: input_file.bytes(),  }
	}
}

impl Iterator for TextParser {
	type Item = String;
	fn next(&mut self) -> Option<String> {	
		let mut result = String::new();
		loop { 
			match self.buffer.next().and_then(|c| { c.ok() }).map(|c| c as char) {
				None if result.len() == 0 => return None,
				None => return Some(result),
				Some(' ') | Some('\r') | Some('\n') | Some('\t') if result.len() == 0 =>  (),
				Some(' ') | Some('\r') | Some('\n') | Some('\t') => return Some(result),
				Some(c) => result.push(c)
 			}
		}
	}
}