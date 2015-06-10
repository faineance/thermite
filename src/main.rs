#![feature(convert)] 
mod vm;
use vm::{VM, Instruction};

static USAGE: &'static str = "
Usage:
vmach assemble 
vmach run
vmach -h | --help
vmach --version
Options:
-h, --help      Show this message.
--version   Display the version.
";



fn main() {
	let mut vm = VM::new();

	let program = vec![Instruction::PSH(6),Instruction::PSH(7),Instruction::ADD, Instruction::OUT, Instruction::HLT];	
	vm.run(program);
	// println!("{:?}", vm);
}
