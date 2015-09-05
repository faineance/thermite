pub mod instructions;
pub mod registers;


#[cfg(test)]
mod tests {
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
	// #[test]
	// fn out() {

	// 	let instruction = Instruction::OUT(Register::RA);
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "out ra")
	// }
	// #[test]
	// fn add() {
	// 	let instruction = Instruction::ADD(Register::RA, Register::RB, Register::RC);
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "add ra rb rc")
	// }
	// #[test]
	// fn sub() {
	// 	let instruction = Instruction::SUB(Register::RA, Register::RB, Register::RC);
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "sub ra rb rc")
	// }
	// #[test]
	// fn mul() {
	// 	let instruction = Instruction::MUL(Register::RA, Register::RB, Register::RC);
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "mul ra rb rc")
	// }
	// #[test]
	// fn div() {
	// 	let instruction = Instruction::DIV(Register::RA, Register::RB, Register::RC);
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "div ra rb rc")
	// }
	// #[test]
	// fn str() {
	// 	let instruction = Instruction::STR(6, Register::RA);
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "str 6 ra")
	// }
	// #[test]
	// fn jmp() {
	// 	let instruction = Instruction::JMP("end".to_string());
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "jmp end")
	// }
	// #[test]
	// fn jz() {
	// 	let instruction = Instruction::JZ(Register::RA, "end".to_string());
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "jz ra end")
	// }
	// #[test]
	// fn jnz() {
	// 	let instruction = Instruction::JNZ(Register::RA, "end".to_string());
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "jnz ra end")
	// }
	// #[test]
	// fn lbl() {
	// 	let instruction = Instruction::LBL("end".to_string());
	// 	let mut string = String::new(); 
	// 	write!(&mut string, "{}", instruction).unwrap();
	// 	assert_eq!(string, "end:")
	// }
	#[test]
	fn hlt() {
		let instruction = Instruction::HLT;
		let mut string = String::new(); 
		write!(&mut string, "{}", instruction).unwrap();
		assert_eq!(string, "hlt")
	}
}