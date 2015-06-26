use std::str::CharIndices;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
	Identifier(String),
	Value(i32),
}

pub fn tokenize(input: &str) -> Vec<Token> {
	
	let mut lexer = Lexer::new(input);
	let mut output = vec![];

	loop {

		match lexer.next() {
			Some(t) => {

				output.push(t);

			}
			None => break
		}
	}
	output
}

pub struct Lexer<'a> {
	input: &'a str,
	iter: Peekable<CharIndices<'a>>,
	pos: usize,
	line: u32,
	column: u32,
}

impl<'a> Lexer<'a> {
	fn new(input : &str) -> Lexer {
		Lexer {
			input: input,
			iter: input.char_indices().peekable(),
			pos: 0,
			line: 1,
			column: 1
		}
	}
	fn advance(&mut self) -> Option<char> {

		if let Some((i, c)) = self.iter.next() {
			match c  {
				'\n' => {
					self.column = 1;
					self.line  += 1;
				},

				'\t' => {
					self.column  += 4;
					
				},
				_ => self.column += 1

			}
			self.pos = i + 1;
			return Some(c);
		}
		None
	}
	fn advance_while<P: Fn(char)-> bool>(&mut self, p: P) {
		loop {

			match self.peek() {
				Some(c) => {

					match p(c) {
						false => break,
						true => {}
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
	fn handle_alphabetic(&mut self) -> Token {
		let start = self.pos;
		self.advance_while(is_alphabetic);
		
		Token::Identifier(self.input[start..self.pos].to_string())
	}
	fn handle_number(&mut self) -> Token {
		let start = self.pos;
		self.advance_while(is_numeric);
		
		Token::Value(self.input[start..self.pos].parse().unwrap())
	}

	fn handle_whitespace(&mut self) {
		self.advance_while(is_whitespace);
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		
		let token = match self.peek() {
			Some(c) if is_alphabetic(c) => self.handle_alphabetic(),
			Some(c) if is_numeric(c) => self.handle_number(),
			Some(c) if is_whitespace(c) => {
				self.handle_whitespace(); 
				match self.next() {
					Some(t) => return Some(t),
					None => return None,
				}
			},
			None => return None,
			_ =>  return None,

		};
		Some(token)
		
	}
}

fn is_numeric(c: char) -> bool {
	match c {
		'0'...'9' => true,
		_ => false
	}
}
fn is_alphabetic(c: char) -> bool {
	match c {
		'a' ... 'z' | 'A' ... 'Z' | ':' => true,
		_ => false
	}
}
fn is_whitespace(c: char) -> bool {
	match c {
		' ' | '\t' | '\n' => true,
		_ => false
	}
}
fn is_comment(c: char) -> bool {
	match c {
		'#' => true,
		_ => false
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::{is_numeric, is_alphabetic};
	#[test]
	fn advance(){
		let mut lexer = Lexer::new("test 14 @\ntesting\ntest\ttesting");
		let mut output = String::new();

		loop {
			match lexer.advance() {
				Some(c) => {
					output.push(c);

				},
				None => break
			}
		}
		assert_eq!(output, "test 14 @\ntesting\ntest\ttesting");
		assert_eq!(lexer.line, 3);
		assert_eq!(lexer.column, 16);
	}
	#[test]
	fn advance_while_numeric(){
		let mut lexer = Lexer::new("4532test s34");
		let start = lexer.pos;
		lexer.advance_while(is_numeric);
		assert_eq!(&lexer.input[start..lexer.pos], "4532");
	}
	#[test]
	fn advance_while_alphabetic(){
		let mut lexer = Lexer::new("test3 3test");
		let start = lexer.pos;
		lexer.advance_while(is_alphabetic);
		assert_eq!(&lexer.input[start..lexer.pos], "test");
	}
	#[test]
	fn peek(){
		let mut lexer = Lexer::new("The quick brown fox jumps over the 64 lazy dog");
		let mut output = String::new();

		loop {

			match lexer.peek() {
				Some(c) => {
					output.push(c);

				}
				None => break
			}
			lexer.advance();
		}
		assert_eq!(output, "The quick brown fox jumps over the 64 lazy dog");
		
	}
	#[test]
	fn iter(){
		let mut lexer = Lexer::new("The 42\n 42");
		let mut output = vec![];

		loop {
			match lexer.next() {
				Some(t) => {

					output.push(t);

				}
				None => break
			}
		}
		assert_eq!(output, vec![Token::Identifier("The".to_string()), Token::Value(42), Token::Value(42)]);
	}
	
}

