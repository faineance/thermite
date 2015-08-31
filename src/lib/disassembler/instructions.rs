use instructions::Instruction;
use std::fmt;



impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Instruction::NOP => write!(f, "nop"), 
			&Instruction::OUT(source) => write!(f, "out {}", source),
			&Instruction::ADD(source, target, destination) => write!(f, "add {} {} {}", source, target, destination),
			&Instruction::SUB(source, target, destination) => write!(f, "sub {} {} {}", source, target, destination),
			&Instruction::MUL(source, target, destination) => write!(f, "mul {} {} {}", source, target, destination),
			&Instruction::DIV(source, target, destination) => write!(f, "div {} {} {}", source, target, destination),
			&Instruction::STR(value, register)  => write!(f, "str {} {}", value, register),
			&Instruction::JMP(ref label) => write!(f, "jmp {}", label),
			&Instruction::JZ(register, ref label)  => write!(f, "jz {} {}", register, label),
			&Instruction::JNZ(register, ref label) => write!(f, "jnz {} {}", register, label),
			&Instruction::LBL(ref label) => write!(f, "{}:", label),
			&Instruction::HLT => write!(f, "hlt"), 
		}
	}
}
