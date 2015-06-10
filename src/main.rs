#![feature(convert)] 
mod vm;
mod instructions;
use instructions::Instruction;
use vm::VM;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{stdin, stdout};
const PROMPT: &'static str = "\x1B[36mvm> \x1B[37m";
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
					print!("Welcome to the vmachine repl.\nUse ctrl-c to exit.\n");
					let mut stdin = stdin();
					let mut stdout = stdout();
					loop {
						stdout.write_all(PROMPT.as_bytes());
						stdout.flush().ok();
						let mut input = String::new();
						stdin.read_line(&mut input);

					}	
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
