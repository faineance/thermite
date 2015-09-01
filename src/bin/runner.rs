use std::fs::File;
use std::io::prelude::*;
use thermite::vm::VM;
use thermite::lexer;
use thermite::parser;

pub struct Runner {
	filename: String
}

impl Runner {
	pub fn new(filename: String) -> Runner {
		Runner {
			filename: filename
		}
	}

	pub fn run(&mut self) {
		match File::open(&self.filename) {
			Ok(mut input) => {
				let mut vm = VM::new();
				let mut contents = String::new(); 

				input.read_to_string(&mut contents).unwrap();

				let tokens = lexer::tokenize(contents.as_ref());

				let program = parser::parse(tokens);

				vm.run(program);
			},
			Err(error) => panic!("{}", error),
		}
	}
}