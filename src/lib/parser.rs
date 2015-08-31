use std::vec;
use lexer::Token;
use instructions::Instruction;
use registers::Register;
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
	InvalidArgument,
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
					Token::Identifier(mut i) => {
						match i.as_ref() {
							"nop" => Instruction::NOP,
							"out" => {
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::OUT(register)
							}
							"add" => {
								let source = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let target = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let destination = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::ADD(source, target, destination)
							},
							"sub" => {
								let source = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let target = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let destination = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::SUB(source, target, destination)
							},
							"mul" => {
								let source = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let target = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let destination = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::MUL(source, target, destination)
							},
							"div" => {
								let source = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let target = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let destination = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::DIV(source, target, destination)
							},							
							"str" => {
								let value = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::STR(value, register)
							},
							"jmp" => {
								
								let loc = match self.advance().unwrap() {
									Token::Identifier(loc) => loc,
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::JMP(loc)
							},
							"jz" => {
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let loc = match self.advance().unwrap() {
									Token::Identifier(loc) => loc,
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::JZ(register, loc)
							},
							"jnz" => {
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								let loc = match self.advance().unwrap() {
									Token::Identifier(loc) => loc,
									_ => return Err(ParserErrorKind::InvalidArgument)
								};
								Instruction::JNZ(register, loc)
							},
							"hlt" => Instruction::HLT,
							_ if i.chars().last().unwrap() == ':' => {
								i.pop().unwrap();

								Instruction::LBL(i)
							},
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
			Err(e) => panic!("ParserError: {:?}", e)
		};

		instruction.unwrap()

	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use lexer::Token;
	use instructions::Instruction;
	use registers::Register;
	#[test]
	fn iter(){
		let tokens = vec![Token::Identifier("str".to_string()), Token::Value(6), Token::Identifier("ra".to_string())];
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
		assert_eq!(output, vec![Instruction::STR(6, Register::RA)]);
	}
}