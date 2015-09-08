use std::collections::hash_map::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::vec::Vec;
use std::string::String;

pub struct WordCountByFrame{
	counter: HashMap<String, Vec<i32>>,
	estimated_frames: usize,
	curr_frame: usize,
	curr_word_count: usize,
	total_word_count: usize,
	frame_size: usize,
}

impl WordCountByFrame {
	pub fn new(capacity: usize, fsize: usize) -> WordCountByFrame { 
		WordCountByFrame { 
			counter: HashMap::new(), 
			estimated_frames: capacity, 
			curr_frame: 0,
			curr_word_count: 0, 
			total_word_count: 0, 
			frame_size: fsize}
	}
	
	pub fn incr_word(&mut self, key: String) -> &WordCountByFrame {
		match self.counter.entry(key) {
			Vacant(entry) => {
				let mut array = Vec::with_capacity(self.estimated_frames);
				for _ in 0..self.curr_frame {
					array.push(0);
				}
				array.push(1);
				entry.insert(array);
			}, 
			
			Occupied(entry) => {
				let mut array = entry.into_mut();
				if array.len() <= self.curr_frame {
					for _ in array.len()..self.curr_frame {
						array.push(0);
					}
					array.push(1);
				}
				else {
					array[self.curr_frame] += 1;
				}
			}
		}
		self.incr_word_count()
	}
	
	pub fn get_word_counts(&mut self) -> &mut HashMap<String, Vec<i32>>
	{
		//zero padding
		for (_, array) in self.counter.iter_mut() {
			for _ in array.len()..(self.curr_frame) {
				array.push(0);
			}
		}
		&mut self.counter
	}
	
	fn incr_word_count(&mut self) -> &WordCountByFrame {
		self.curr_word_count += 1;
		self.total_word_count += self.curr_word_count;
		if self.curr_word_count % self.frame_size == 0 {
			self.curr_frame += 1;
			self.curr_word_count = 0;
		}
		return self;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::hash_map::HashMap;
	use std::collections::hash_map::Entry::{Vacant, Occupied};
	
	#[test]
	fn word_count_test() {
		let mut counts = WordCountByFrame::new(4, 5);
		let words = vec!["rabbit", "fox", "dog", "cat", "rabbit",
						 "fox", "fox", "dog", "cat", "rabbit",
						 "fox", "fox", "fox", "fox", "fox", 
						 "dog", "cat", "cat", "dog", "dog"];
		
		
		for word in words {
			counts.incr_word(word.to_string());
		}
		
		assert_eq!(counts.curr_frame, 4);
		let counter: &mut HashMap<String, Vec<i32>> = counts.get_word_counts();
		{
			let ans = vec![2, 1, 0, 0]; 
			match counter.entry("rabbit".to_string()) {
				Vacant(_) => assert!(false),
				Occupied(entry) => {
					let array = entry.get();
					assert_eq!(array.len(), ans.len());
					for i in (0..array.len()) {
						println!("{}", array[i]);
						assert_eq!(array[i], ans[i]);
					}
				}
			}
		}
		{
			let ans = vec![1, 1, 0, 3]; //no zero padded
			match counter.entry("dog".to_string()) {
				Vacant(_) => assert!(false),
				Occupied(entry) => {
					let array = entry.get();
					assert_eq!(array.len(), ans.len());
					for i in (0..array.len()) {
						println!("{}", array[i]);
						assert_eq!(array[i], ans[i]);
					}
				}
			}
		}		
	}
	
}