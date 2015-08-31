use instructions::Instruction;
use std::fmt;



impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Instruction::NOP => write!(f, "nop"), 
			&Instruction::OUT => write!(f, "out"),
			&Instruction::PSH(value) => write!(f, "psh {}", value),
			&Instruction::POP => write!(f, "pop"),
			&Instruction::ADD => write!(f, "add"),
			&Instruction::SUB => write!(f, "sub"),
			&Instruction::MUL => write!(f, "mul"),
			&Instruction::DIV => write!(f, "div"),
			&Instruction::LDR(register)  => write!(f, "ldr {}", register),
			&Instruction::STR(register)  => write!(f, "str {}", register),
			&Instruction::JMP(ref label) => write!(f, "jmp {}", label),
			&Instruction::JZ(ref label)  => write!(f, "jz {}", label),
			&Instruction::JNZ(ref label) => write!(f, "jnz {}", label),
			&Instruction::LBL(ref label) => write!(f, "{}:", label),
			&Instruction::HLT => write!(f, "hlt"), 
		}
	}
}
