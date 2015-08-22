mod vm;
mod lexer;
mod parser;
mod instructions;
mod registers;

use instructions::Instruction;
use vm::VM;
use lexer::tokenize;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{stdin, stdout};

const PROMPT: &'static str = "\x1B[36mvm> \x1B[37m";
static USAGE: &'static str = "
Usage:
thermite repl
thermite run
thermite -h | --help
thermite --version
Options:
-h, --help      Show this message.
--version   Display the version.
";
fn repl() {
	print!("Welcome to the thermite repl.\nUse ctrl-c to exit.\n");
	let stdin = stdin();
	let mut stdout = stdout();
	let mut vm = VM::new();
	loop {

		stdout.write_all(PROMPT.as_bytes()).unwrap();
		stdout.flush().ok();
		let mut input = String::new();
		stdin.read_line(&mut input).unwrap();

		let tokens = lexer::tokenize(input.as_ref());

		let mut program = parser::parse(tokens);
		program.push(Instruction::HLT);

		vm.run(program, true);
	}	
}
fn run(filename: String) {
	match File::open(filename) {
		Ok(mut input) => {
			let mut vm = VM::new();
			let mut contents = String::new(); 

			input.read_to_string(&mut contents).unwrap();

			let tokens = lexer::tokenize(contents.as_ref());

			let program = parser::parse(tokens);

			vm.run(program, false);
		},
		Err(error) => panic!("{}", error),
	}
}
fn main() {
	let mut arguments = env::args();
	arguments.next();
	match arguments.next() {
		Some(command) => {
			match command.as_ref() {
				"repl" => {
					repl();
				},
				"run" => {
					match arguments.next() {
						Some(filename) => {
							run(filename);
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
