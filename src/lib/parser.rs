use std::vec;
use lexer::Token;
use instructions::Instruction;
use instructions::{IOType, ArithmaticType, BitwiseType, ShiftType, BranchType, ControlType, AssignmentType};
use registers::Register;
pub type ParserResult<T> = Result<Option<T>, ParserError>;

pub fn parse(input: Vec<Token>) -> Vec<Instruction> {
	let mut parser = Parser::new(input);
	let mut program = vec![];
	while let Some(i) = parser.next() {
		program.push(i)
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
	fn take_register(&mut self) -> Option<Register> {
		match self.advance().unwrap() {
			Token::Identifier(r) => Some(Register::from(r)),
			_ => return None
		}
	}
	fn take_value(&mut self) -> Option<i32> {
		match self.advance().unwrap() {
			Token::Value(i) => Some(i),
			_ => return None
		}
	}

	fn handle_io(&mut self, kind: IOType) -> ParserResult<Instruction> {
		match self.take_register() {
			Some(register) => Ok(Some(Instruction::IO(kind, register))),
			None => return Err(ParserError::InvalidArgument)
		}
	}
	fn handle_arithmatic(&mut self, kind: ArithmaticType) -> ParserResult<Instruction> {
		match (self.take_register(), self.take_register(), self.take_register()) {
			(Some(source), Some(target), Some(destination)) => {
				Ok(Some(Instruction::Arithmatic(kind, source, target, destination)))
			},
			_ => return Err(ParserError::InvalidArgument)
		}
	}
	fn handle_bitwise(&mut self, kind: BitwiseType) -> ParserResult<Instruction> {
		match (self.take_register(), self.take_register(), self.take_register()) {
			(Some(source), Some(target), Some(destination)) => {
				Ok(Some(Instruction::Bitwise(kind, source, target, destination)))
			},
			_ => return Err(ParserError::InvalidArgument)
		}
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
		match self.take_register() {
			Some(register) => Ok(Some(Instruction::Assignment(kind, register))),
			None => return Err(ParserError::InvalidArgument)
		}
	}
	fn next_instruction(&mut self) -> ParserResult<Instruction> {
		let mut result = Ok(None);
		if let Some(t) = self.advance() {
			if let Token::Identifier(mut i) = t {
				match i.as_ref() {
					"nop" => result = Ok(Some(Instruction::NOP)),
					"hlt" => result = Ok(Some(Instruction::HLT)),

					"out" => result = self.handle_io(IOType::OUT),
					"in" => result =  self.handle_io(IOType::IN),

					"add" => result = self.handle_arithmatic(ArithmaticType::ADD),
					"sub" => result = self.handle_arithmatic(ArithmaticType::SUB),
					"mul" => result = self.handle_arithmatic(ArithmaticType::MUL),
					"div" => result = self.handle_arithmatic(ArithmaticType::DIV),
					"max" => result = self.handle_arithmatic(ArithmaticType::MAX),
					"min" => result = self.handle_arithmatic(ArithmaticType::MIN),

					"and" => result = self.handle_bitwise(BitwiseType::AND),
					"or"  => result = self.handle_bitwise(BitwiseType::OR),
					"xor" => result = self.handle_bitwise(BitwiseType::XOR),
					"shr" => result = self.handle_bitwise(BitwiseType::SHIFT(ShiftType::RIGHT)),
					"shl" => result = self.handle_bitwise(BitwiseType::SHIFT(ShiftType::LEFT)),

					"jmp" => result = self.handle_branch(BranchType::UNCONDITIONAL),
					"jz" => {
						match self.take_register() {
							Some(register) => result = self.handle_branch(BranchType::ZERO(register)),
							None => return Err(ParserError::InvalidArgument)
						}
					},
					"jnz" => {
						match self.take_register() {
							Some(register) => result = self.handle_branch(BranchType::NOTZERO(register)),
							None => return Err(ParserError::InvalidArgument)
						}
					},
					"str" => {
						match self.take_value() {
							Some(value) => result = self.handle_assignment(AssignmentType::STR(value)),
							None => return Err(ParserError::InvalidArgument)
						}
					},
					"cpy" => {
						match self.take_register() {
							Some(register) => result = self.handle_assignment(AssignmentType::CPY(register)),
							None => return Err(ParserError::InvalidArgument)
						}
					},
					_ if i.chars().last().unwrap() == ':' => {
						i.pop().unwrap();
						result = self.handle_control(ControlType::LBL(i))
					},
					_ => return Err(ParserError::InvalidInstruction)
				}
			}
		}
		result
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