pub type Register = usize;

#[derive(Debug, PartialEq)]
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
	JMP, //todo
	JZ, //todo
	JNZ, //todo
	HLT, // Halts program
    NOP, // Does nothing
}