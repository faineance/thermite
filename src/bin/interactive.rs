use std::io::{stdin, stdout};
use std::io::{Stdin, Stdout};
use std::io::prelude::*;
use thermite::vm::VM;
use thermite::lexer;
use thermite::parser;

const WELCOME: &'static str = "Welcome to the \x1b[1mthermite\x1b[0m interactive mode.\nUse \x1b[1mctrl-c\x1b[0m to exit.\n";

const PROMPT: &'static str = "\x1B[36mvm> \x1B[37m";

pub struct Interactive {
	vm: VM,
	stdin: Stdin,
	stdout: Stdout,
}

impl Interactive {
	pub fn new() -> Interactive {
		Interactive {
			vm:  VM::new(),
			stdin: stdin(),
			stdout: stdout()
		}
	}
	pub fn run(&mut self) {
		self.stdout.write_all(WELCOME.as_bytes()).unwrap();
		self.stdout.flush().ok();

		loop {
			self.stdout.write_all(PROMPT.as_bytes()).unwrap();
			self.stdout.flush().ok();
			let mut input = String::new();
			self.stdin.read_line(&mut input).unwrap();
			if input.trim() == "" {continue}
			let tokens = lexer::tokenize(input.as_ref());

			let program = parser::parse(tokens);
			match program.len() {
				1 => self.vm.interactive(program[0].clone()),
				_ => println!("Interactive Mode: Only one instruction allowed by line")
			}
		}
	}
}