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
	let mut tokenizer = Lexer::new(input);

	loop {
		match tokenizer.next() {
			Some(t) => {

				tokens.push(t);

			},
			None => break 
		}
	}
	tokens
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
			self.pos = i;
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
	fn handle_alphabetic(&mut self) -> Token {
		//todo
		unimplemented!()
	}
	fn handle_number(&mut self) -> Token {
		let start = self.pos;
		self.advance_while(is_numeric);
		
		Token::Value(self.input[start..self.pos].parse().unwrap())

	}
	fn handle_other(&mut self) -> Token {
		//todo
		unimplemented!()
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		let token = match self.peek() {
			Some(c) if is_alphabetic(c) => self.handle_alphabetic(),
			Some(c) if is_numeric(c) => self.handle_number(),
			Some(c) if !is_numeric(c) && !is_alphabetic(c) => self.handle_other(),
			_ =>  unreachable!()
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
		'a' ... 'z' | 'A' ... 'Z' => true,
		_ => false
	}
}
