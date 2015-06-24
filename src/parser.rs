use std::vec;
use lexer::Token;
use instructions::Instruction;

pub fn parse(input: Vec<Token>) -> Vec<Instruction> {
	let mut parser = Parser::new(input);
	let mut program = vec![];

	loop {
		match parser.next() {
			Some(t) => {

				program.push(t);

			}
			None => break
		}
	}
	program
}
pub struct Parser {
	iter: vec::IntoIter<Token>
}

#[derive(Debug,PartialEq)]
pub enum ParserErrorKind {
	InvalidInstruction,
}

impl Parser {
	fn new(input: Vec<Token>) -> Parser {
		Parser {
			iter: input.into_iter()
		}
	}
	fn advance(&mut self) -> Option<Token> {
		if let Some(t) = self.iter.next() {
			return Some(t);
		}
		None
	}
	fn next_instruction(&mut self) -> Result<Option<Instruction>, ParserErrorKind> {
		let instruction = match self.advance() {
			Some(t) => {
				match t {
					Token::Identifier(i) => {
						match i.as_ref() {
							"out" => Instruction::OUT,
							"psh" => {
								
								let value = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								Instruction::PSH(value)
							},
							"pop" => Instruction::POP,
							"add" => Instruction::ADD,
							"sub" => Instruction::SUB,
							"mul" => Instruction::MUL,
							"div" => Instruction::DIV,
							"set" => {
								let register = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								let value = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								Instruction::SET(register as usize, value)
							},
							"mov" => {
								let register1 = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								let register2 = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								Instruction::MOV(register1 as usize , register2 as usize)
							},
							"ldr" => {
								
								let register = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								Instruction::LDR(register as usize)
							},
							"str" => {
								
								let register = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => unreachable!()
								};
								Instruction::STR(register as usize)
							},
							"hlt" => Instruction::HLT,
							"nop" => Instruction::NOP,
							_ => return Err(ParserErrorKind::InvalidInstruction)
						}

					},
					_ => return Err(ParserErrorKind::InvalidInstruction),
				}

			}
			None => return Ok(None)
		};

		Ok(Some(instruction))
	}
}
impl Iterator for Parser {
	type Item = Instruction;
	fn next(&mut self) -> Option<Instruction> {

		let instruction = match self.next_instruction() {
			Ok(i) => Some(i),
			Err(e) => return None
		};
			
		instruction.unwrap()

	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use lexer::Token;
	use instructions::Instruction;
	#[test]
	fn advance(){
		let tokens = vec![Token::Identifier("psh".to_string()), Token::Value(6)];
		let mut parser = Parser::new(tokens);
		let mut output = vec![];

		loop {
			match parser.advance() {
				Some(c) => {
					output.push(c);

				},
				None => break
			}
		}
		assert_eq!(output, vec![Token::Identifier("psh".to_string()), Token::Value(6)]);
	}
	#[test]
	fn iter(){
		let tokens = vec![Token::Identifier("psh".to_string()), Token::Value(6)];
		let mut parser = Parser::new(tokens);
		let mut output = vec![];

		loop {
			match parser.next() {
				Some(c) => {
					output.push(c);
				},
				None => break
			}
		}
		assert_eq!(output, vec![Instruction::PSH(6)]);
	}
}