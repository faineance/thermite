#![feature(convert)] 
mod vm;
use vm::{VM, Instruction};
use std::env;
use std::io::prelude::*;
use std::fs::File;
static USAGE: &'static str = "
Usage:
vmach repl
vmach run
vmach -h | --help
vmach --version
Options:
-h, --help      Show this message.
--version   Display the version.
";



fn main() {
	let mut arguments = env::args();
	arguments.next();
	match arguments.next() {
		Some(command) => {
			match command.as_ref() {
				"repl" => {

				},
				"run" => {
					match arguments.next() {
						Some(filename) => {
							let mut s = String::new(); 
							match File::open(filename) {
								Ok(mut input) => {
									// let mut program = String::new(); 

									// input.read_to_string(&mut program);

									let mut vm = VM::new();
									let program = vec![Instruction::PSH(6),Instruction::PSH(7),Instruction::ADD, Instruction::OUT, Instruction::HLT];	
									vm.run(program);
								},
								Err(error) => panic!("{}", error),
							}
						}
						_ => println!("No file specified\n{}", USAGE),
					}
				}
				_ => println!("Unrecognised command \n{}", USAGE),
			}
		},
		_ => println!("No command specified \n{}", USAGE),
	}
}
