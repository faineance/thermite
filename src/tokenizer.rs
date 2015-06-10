use std::str::CharIndices;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
	Identifier(String),
	Value(i32),
	Register(usize),

}

pub fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut tokenizer = Tokenizer::new(input);

	tokens
}

pub struct Tokenizer<'a> {
	iter: Peekable<CharIndices<'a>>,
	line: u32,
	column: u32,
}

impl<'a> Tokenizer<'a> {
	fn new(input : &str) -> Tokenizer {
		Tokenizer {
			iter: input.char_indices().peekable(),
			line: 1,
			column: 1
		}
	}
	fn advance(&mut self) -> Option<char> {
		if let Some((_, c)) = self.iter.next() {
			match c  {
				'\n' => {
					self.column = 1;
					self.line += 1;
				}
				'\t' => {
					self.column += 4;
					
				}
				_ => self.column += 1

			}
			return Some(c);
		}
		None
	}
	fn advance_while<P: Fn(char) -> bool>(&mut self, p: P) {
		loop {
			match self.peek() {
				Some(c) => {
					match p(c) {
						false => break,
						true => continue
					}
				}
				None => break
			}
			self.advance();
		}
	}
	fn peek(&mut self) -> Option<char> {
		if let Some(&(_, c)) = self.iter.peek() {
			return Some(c);
		}
		None
	}
	fn is_number(&self, c: char) -> bool {
		match c {
			'0'...'9' => true,
			_ => false
		}
	}
	fn is_alphabetic(&self, c: char) -> bool {
		match c {
			'a' ... 'z' | 'A' ... 'Z' => true,
			_ => false
		}
	}
	fn handle_alphabetic(&mut self) -> Token {
		//todo
		unimplemented!()
	}
	fn handle_number(&mut self) -> Token {
		//todo
		unimplemented!()
	}
	fn handle_symbol(&mut self) -> Token {
		//todo
		unimplemented!()
	}
}

impl<'a> Iterator for Tokenizer<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		let token = match self.peek() {
			Some(c) if self.is_alphabetic(c) => self.handle_alphabetic(),
			Some(c) if self.is_number(c) => self.handle_number(),
			Some(c) if !self.is_number(c) && !self.is_alphabetic(c) => self.handle_symbol(),
			_ =>  unreachable!()
		};
		Some(token)
		
	}
}