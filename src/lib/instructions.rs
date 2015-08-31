use registers::Register;


#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	NOP, 
	OUT, 
	PSH(i32), 
	POP, 
	ADD,
	SUB, 
	MUL,
	DIV,
	LDR(Register), 
	STR(Register),
	JMP(String), 
	JZ(String), 
	JNZ(String), 
	LBL(String),
	HLT, 
}


// impl ToString for Instruction {
// 	fn to_string(&self) -> String {
// 		match *self {
//             Instruction::NOP => format!("nop"),
//             Instruction::OUT => format!("out"),
//             Instruction::PSH(value) => format!("psh {}", value),
//             Instruction::POP => format!("pop"),
//             Instruction::ADD => format!("add"),
//             Instruction::SUB => format!("sub"),
//             Instruction::MUL => format!("mul"),
//             Instruction::DIV => format!("div"),
//             Instruction::LDR(register)  => format!("ldr {}", register),
//             Instruction::STR(register)  => format!("str {}", register),
//             Instruction::JMP(ref label) => format!("jmp {}", label),
//             Instruction::JZ(ref label)  => format!("jz {}", label),
//             Instruction::JNZ(ref label) => format!("jnz {}", label),
//             Instruction::LBL(ref label) => format!("{}:", label),
//             Instruction::HLT => format!("hlt"),
//         }.to_string()
// 	}
// }

// impl Into<Bytecode> for Instruction {
// 	fn into(self) -> Bytecode {
// 		match self {
// 			Instruction::NOP => 0x0, // TODO
//             Instruction::OUT => 0x0,
//             Instruction::PSH(value) => 0x1,
//             Instruction::POP => 0x2,
//             Instruction::ADD => 0x3,
//             Instruction::SUB => 0x4,
//             Instruction::MUL => 0x5,
//             Instruction::DIV => 0x6,
//             Instruction::LDR(register) => 0x9,
//             Instruction::STR(register) => 0xa,
//             Instruction::JMP(ref label) => 0xb,
//             Instruction::JZ(ref label)  => 0xc,
//             Instruction::JNZ(ref label) => 0xd,
//             Instruction::LBL(ref label) => 0xe,
//             Instruction::HLT => 0xf, 
// 		}
// 	}
// }


