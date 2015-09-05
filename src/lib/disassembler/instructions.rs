use instructions::Instruction;
use instructions::{IOType, ArithmaticType, BitwiseType, ShiftType, BranchType, ControlType, AssignmentType};
use std::fmt;



impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Instruction::IO(ref kind, register) => {
				let string = match kind {
					&IOType::OUT => "out",
					&IOType::IN => "in",
				};
				write!(f, "{} {}", string, register)
			},
			&Instruction::Arithmatic(ref kind, source, target, destination) => {
				let string = match kind {
					&ArithmaticType::ADD => "add",
					&ArithmaticType::SUB => "sub",
					&ArithmaticType::MUL => "mul",
					&ArithmaticType::DIV => "div",
					&ArithmaticType::MAX => "max",
					&ArithmaticType::MIN => "min",
				};
				write!(f, "{} {} {} {}", string, source, target, destination)
			},
			&Instruction::Bitwise(ref kind, source, target, destination) => {
				let string = match kind {
					&BitwiseType::AND => "and",
					&BitwiseType::OR  => "or",
					&BitwiseType::XOR => "xor",
					&BitwiseType::SHIFT(ref kind) => {
						match kind {
							&ShiftType::LEFT => "shl",
							&ShiftType::RIGHT => "shr",
						}
					}
				};
				write!(f, "{} {} {} {}", string, source, target, destination)
			},
			&Instruction::Branch(ref kind, ref label) => {
				let string: String = match kind {
					&BranchType::UNCONDITIONAL => "jmp".to_string(),
					&BranchType::NOTZERO(register) => format!("jnz {}", register),
					&BranchType::ZERO(register) => format!("jz {}", register),
				};
				write!(f, "{} {}", string, label)
			},
			&Instruction::Control(ref kind) => {
				let string = match kind {
					&ControlType::LBL(ref label) => format!("{}:", label),
				};
				write!(f, "{}", string)
			}
			&Instruction::Assignment(ref kind, register) => {
				let string = match kind {
					&AssignmentType::STR(value) => format!("str {}", value),
					&AssignmentType::CPY(value) => format!("cpy {}", value),
				};
				 write!(f, "{} {}", string, register)
			}
			&Instruction::NOP => write!(f, "nop"), 
			&Instruction::HLT => write!(f, "hlt"), 
		}
	}
}
