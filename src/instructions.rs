pub type Register = usize;


#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	OUT, // Print top of stack
    PSH(i32), // Pushes value to stack
	POP, // Pops from stack
	ADD, // Adds two top values on stack
	SUB, 
	MUL,
	DIV,
	SET(Register,i32), 
	MOV(Register,Register),
	LDR(Register), //Pushes value in register to stack
	STR(Register), //Store TOS to register
	JMP(String), //todo
	JZ(String), //todo
	JNZ, //todo
	LBL(String),
	HLT, // Halts program
    NOP, // Does nothing
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