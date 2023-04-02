use std::{usize};
type Tokenizer<T> = fn(text: &str) -> Option<(T, usize)>;
type Skip = fn(text: &str) -> usize;
pub struct Lexer<T>
{
	lexers: std::vec::Vec<Tokenizer<T>>,
	skipper: Skip
}
impl<T> Lexer<T> {
	pub fn new(lexers: &[Tokenizer<T>], skipper: Skip) -> Self {
		return Self {lexers: std::vec::Vec::<Tokenizer<T>>::from(lexers), skipper: skipper };
	}

	pub fn add(&mut self, lexer: Tokenizer<T>) -> () {
		self.lexers.push(lexer);
	}

	// Tries to consume starting characters, in order to generate a token
	// Will not attempt to skip characters
	// Returns optionally token and number of consumed characters, if successful
	pub fn tokenize_skipless(&self, text: &str) -> Option<(T, usize)>
	{
		for lexer in &self.lexers {
			match lexer(text) {
				Some((token, consumed)) => return Some((token, consumed)),
				_ => continue
			}
		}
		return None;
	}

	// Tries to consume starting characters, in order to generate a token
	// Will attempt to skip characters before trying to generate a token
	// Returns optionally token and number of consumed characters, if successful
	pub fn tokenize(&self, text: &str) -> Option<(T, usize, usize)> {
		let skip = (self.skipper)(text);
		match self.tokenize_skipless(&text[skip..]) {
			Some((token, consumed)) => return Some((token, skip, consumed)),
			None => return None
		}
	}

	pub fn tokenize_into_list(&self, text: &str, out: &mut std::vec::Vec<(T, usize, usize)>) -> usize {
		let mut acc: usize = 0usize;
		while acc < text.len() {
			match self.tokenize(&text[acc..]) {
				Some((token, skip, consumed)) => {
					out.push((token, acc + skip, consumed));
					acc += skip + consumed;
				},
				_ => break
			}
		}
		return acc;
	}
}