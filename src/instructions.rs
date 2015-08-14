use std::string::ToString;
pub type Register = usize;


#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	OUT, /
    PSH(i32), 
	POP, 
	ADD,
	SUB, 
	MUL,
	DIV,
	SET(Register,i32), 
	MOV(Register,Register),
	LDR(Register), 
	STR(Register),
	JMP(String), 
	JZ(String), 
	JNZ(String), 
	LBL(String),
	HLT, 
    NOP, 
} 


impl ToString for Instruction {
	fn to_string(&self) -> String {
		match *self {
            Instruction::OUT    => format!("out"),
            Instruction::PSH(value) => format!("out {}", value),
            Instruction::POP => format!("pop"),
            Instruction::ADD => format!("add"),
            Instruction::SUB => format!("sub"),
            Instruction::MUL => format!("mul"),
            Instruction::DIV => format!("div"),
            Instruction::SET(register, value) => format!("set {} {}", register, value),
            Instruction::MOV(register1, register2) => format!("mov {} {}", register1, register2),
            Instruction::LDR(register) => format!("ldr {}", register),
            Instruction::STR(register) => format!("str {}", register),
            Instruction::JMP(ref label) => format!("jmp {}", label),
            Instruction::JZ(ref label)  => format!("jz {}", label),
            Instruction::JNZ(ref label) => format!("jnz {}", label),
            Instruction::LBL(ref label) => format!("{}:", label),
            Instruction::HLT => format!("hlt"),
            Instruction::NOP => format!("nop")
        }.to_string()
	}
}

// pub trait ToInstruction {
//     fn to_instruction(&self) -> Option<Instruction>;
// }

// impl ToInstruction for Token {
//     fn to_instruction(&self) -> Option<Instruction> {
//         let instruction = match self {
//         	&Token::Identifier(i) => {
//         		match i.as_ref() {
//         			"out" => Some(Instruction::OUT),
//         			"psh" => Some(Instruction::PSH(None)),
//         			"pop" => Some(Instruction::POP),
//         			"add" => Some(Instruction::ADD),
//         			"sub" => Some(Instruction::SUB),
//         			"mul" => Some(Instruction::MUL),
//         			"div" => Some(Instruction::DIV),
//         			"set" => Some(Instruction::SET(None, None)),
//         			"mov" => Some(Instruction::MOV(None, None)),
//         			"ldr" => Some(Instruction::LDR(None)),
//         			"str" => Some(Instruction::STR(None)),
//         			"jmp" => Some(Instruction::JMP),
//         			"jz"  => Some(Instruction::JZ ),
//         			"jnz" => Some(Instruction::JNZ),
//         			"hlt" => Some(Instruction::HLT),
//         			"nop" => Some(Instruction::NOP),
//         			_ => None,
//         		}
//         	}
//             _ => unreachable!(),
//         };
//         instruction
//     }
// }