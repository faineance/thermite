#![crate_type = "bin"]
extern crate thermite;

use std::env;

mod interactive;
mod runner;
use runner::Runner;
use interactive::Interactive;


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


fn main() {
	let mut arguments = env::args();
	arguments.next();
	match arguments.next() {
		Some(command) => {
			match command.as_ref() {
				"repl" => {
					let mut interactive = Interactive::new();
					interactive.run()
				},
				"run" => {
					match arguments.next() {
						Some(filename) => {
							let mut runner = Runner::new(filename);
							runner.run();
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
