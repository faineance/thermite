pub mod instructions;
pub mod registers;


#[cfg(test)]
mod tests {
	use super::*;
	use std::fmt::Write;
	use instructions::Instruction;
	use registers::Register;
	#[test]
	fn nop() {
		let instruction = Instruction::NOP;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "nop")
	}
	#[test]
	fn out() {
		let instruction = Instruction::OUT;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "out")
	}
	#[test]
	fn psh() {
		let instruction = Instruction::PSH(5);
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "psh 5")
	}
	#[test]
	fn pop() {
		let instruction = Instruction::POP;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "pop")
	}
	#[test]
	fn add() {
		let instruction = Instruction::ADD;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "add")
	}
	#[test]
	fn sub() {
		let instruction = Instruction::SUB;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "sub")
	}
	#[test]
	fn mul() {
		let instruction = Instruction::MUL;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "mul")
	}
	#[test]
	fn div() {
		let instruction = Instruction::DIV;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "div")
	}
	#[test]
	fn ldr() {
		let instruction = Instruction::LDR(Register::RA);
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "ldr ra")
	}
	#[test]
	fn str() {
		let instruction = Instruction::STR(Register::RA);
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "str ra")
	}
	#[test]
	fn jmp() {
		let instruction = Instruction::JMP("end".to_string());
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "jmp end")
	}
	#[test]
	fn jz() {
		let instruction = Instruction::JZ("end".to_string());
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "jz end")
	}
	#[test]
	fn jnz() {
		let instruction = Instruction::JNZ("end".to_string());
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "jnz end")
	}
	#[test]
	fn lbl() {
		let instruction = Instruction::LBL("end".to_string());
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "end:")
	}
	#[test]
	fn hlt() {
		let instruction = Instruction::HLT;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "hlt")
	}
}