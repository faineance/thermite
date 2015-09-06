use std::vec;
use lexer::Token;
use instructions::Instruction;
use instructions::{IOType, ArithmaticType, BitwiseType, ShiftType, BranchType, ControlType, AssignmentType};
use registers::Register;
pub type ParserResult<T> = Result<Option<T>, ParserError>;

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
pub enum ParserError {
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
	fn handle_io(&mut self, kind: IOType) -> ParserResult<Instruction> {
		let register = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		Ok(Some(Instruction::IO(kind, register)))
	}
	fn handle_arithmatic(&mut self, kind: ArithmaticType) -> ParserResult<Instruction> {
		let source = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		let target = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		let destination = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		Ok(Some(Instruction::Arithmatic(kind, source, target, destination)))
	}
	fn handle_bitwise(&mut self, kind: BitwiseType) -> ParserResult<Instruction> {
		let source = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		let target = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		let destination = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		Ok(Some(Instruction::Bitwise(kind, source, target, destination)))
	}
	fn handle_branch(&mut self, kind: BranchType) -> ParserResult<Instruction> {
		let label = match self.advance().unwrap() {
			Token::Identifier(r) => r,
			_ => return Err(ParserError::InvalidArgument)
		};
		Ok(Some(Instruction::Branch(kind, label)))
	}
	fn handle_control(&mut self, kind: ControlType) -> ParserResult<Instruction> {
		Ok(Some(Instruction::Control(kind)))
	}
	fn handle_assignment(&mut self, kind: AssignmentType) -> ParserResult<Instruction> {
		let register = match self.advance().unwrap() {
			Token::Identifier(r) => Register::from(r),
			_ => return Err(ParserError::InvalidArgument)
		};
		Ok(Some(Instruction::Assignment(kind, register)))
	}

	fn next_instruction(&mut self) -> ParserResult<Instruction> {
		match self.advance() {
			Some(t) => {
				match t {
					Token::Identifier(mut i) => {
						match i.as_ref() {

							"nop" => Ok(Some(Instruction::NOP)),
							"hlt" => Ok(Some(Instruction::HLT)),

							"out" => self.handle_io(IOType::OUT),
							"in" => self.handle_io(IOType::IN),

							"add" => self.handle_arithmatic(ArithmaticType::ADD),
							"sub" => self.handle_arithmatic(ArithmaticType::SUB),
							"mul" => self.handle_arithmatic(ArithmaticType::MUL),
							"div" => self.handle_arithmatic(ArithmaticType::DIV),
							"max" => self.handle_arithmatic(ArithmaticType::MAX),
							"min" => self.handle_arithmatic(ArithmaticType::MIN),

							"and" => self.handle_bitwise(BitwiseType::AND),
							"or"  => self.handle_bitwise(BitwiseType::OR),
							"xor" => self.handle_bitwise(BitwiseType::XOR),
							"shr" => self.handle_bitwise(BitwiseType::SHIFT(ShiftType::RIGHT)),
							"shl" => self.handle_bitwise(BitwiseType::SHIFT(ShiftType::LEFT)),

							"jmp" => self.handle_branch(BranchType::UNCONDITIONAL),
							"jz" => {
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserError::InvalidArgument)
								};
								self.handle_branch(BranchType::ZERO(register))
							}
							"jnz" => {
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserError::InvalidArgument)
								};
								self.handle_branch(BranchType::NOTZERO(register))
							},

							"str" => {
								let value = match self.advance().unwrap() {
									Token::Value(i) => i,
									_ => return Err(ParserError::InvalidArgument)
								};
								self.handle_assignment(AssignmentType::STR(value))
							},
							"cpy" => {
								let register = match self.advance().unwrap() {
									Token::Identifier(r) => Register::from(r),
									_ => return Err(ParserError::InvalidArgument)
								};
								self.handle_assignment(AssignmentType::CPY(register))
							},
							_ if i.chars().last().unwrap() == ':' => {
								i.pop().unwrap();
								self.handle_control(ControlType::LBL(i))
							},

							_ => return Err(ParserError::InvalidInstruction)
						}

					},
					_ => return Err(ParserError::InvalidInstruction),
				}

			}
			None => return Ok(None)
		}

	}
}
impl Iterator for Parser {
	type Item = Instruction;
	fn next(&mut self) -> Option<Instruction> {

		match self.next_instruction() {
			Ok(i) => i,
			Err(e) => panic!("ParserError: {:?}", e)
		}

	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use lexer::Token;
	use instructions::Instruction;
	use instructions::{IOType, ArithmaticType, BitwiseType, ShiftType, BranchType, ControlType, AssignmentType};

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
		assert_eq!(output, vec![Instruction::Assignment(AssignmentType::STR(6), Register::RA)]);
	}
}