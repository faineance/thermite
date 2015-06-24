use std::fmt;
use std::collections::HashMap;
use instructions::Instruction;
const STACK_SIZE: usize = 256;
const REG_SIZE: usize = 16; 


#[derive(Debug,PartialEq)]
enum VMErrorKind {
	StackError,
	ZeroDivision,
}
impl fmt::Debug for VM {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Registers: \n {:?} \nStack Contents:\n {:?}", self.registers, self.stack)
	}
}

pub struct VM {
	stack: Vec<i32>,
	registers: [i32; REG_SIZE],
	ip: usize,
	running: bool,
	debug: bool,
}

impl VM {
	pub fn new() -> VM {
		VM { stack: Vec::with_capacity(STACK_SIZE), registers: [0; REG_SIZE], ip: 0, running: true, debug: true}
	}
	pub fn run(&mut self, program: Vec<Instruction>, repl: bool) {

		while self.running {
			let instruction = &program[self.ip];
			
			match self.eval(instruction) {
				Ok(_) => {},
				Err(e) => {
					println!("Registers: \n {:?}",self.registers);
					println!("Stack Contents:\n {:?}",self.stack);
					panic!("VMError: {:?} on ip {:?}", e, self.ip + 1);	
				}
			}
			self.ip += 1;
		}
		if repl {
			self.ip = 0;
			self.running = true;
		}
	}

	fn eval(&mut self, instruction: &Instruction) -> Result<(), VMErrorKind> {
		match instruction {
			&Instruction::OUT => {
				println!("{:?}", self.stack.last().unwrap() );
				Ok(())
			}
			&Instruction::PSH(value) => {
				self.stack.push(value);
				Ok(())
			}
			&Instruction::POP => {
				match self.stack.pop() {
					Some(v) => Ok(()),
					_ => Err(VMErrorKind::StackError)
				}
			}
			&Instruction::ADD => {
				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						self.stack.push(a + b); 
						Ok(())
					},
					_ => Err(VMErrorKind::StackError)
				}
			}
			&Instruction::SUB => {
				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						self.stack.push(a - b); 
						Ok(())
					},
					_ => Err(VMErrorKind::StackError)
				}

			}
			&Instruction::MUL => {
				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						self.stack.push(a * b); 
						Ok(())
					},
					_ => Err(VMErrorKind::StackError)
				}
			}
			&Instruction::DIV => {

				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						if b == 0 {
							return Err(VMErrorKind::ZeroDivision)
						} else { 
							self.stack.push(a / b);
						}
						Ok(())
						
					},
					_ => Err(VMErrorKind::StackError)
				}
			}
			&Instruction::SET(register, value) => {
				self.registers[register as usize] = value;
				Ok(())
			}
			&Instruction::MOV(register1, register2) => {
				self.registers[register2 as usize] = self.registers[register1 as usize];
				Ok(())
			}
			&Instruction::LDR(register) => {
				self.stack.push(self.registers[register as usize] as i32 );
				Ok(())
			}
			&Instruction::STR(register) => {
				match self.stack.pop() {
					Some(value) => {
						self.registers[register as usize] = value;
						Ok(())
					},
					_ => Err(VMErrorKind::StackError)
				}
			}
			&Instruction::JMP => {
				unimplemented!()
				// Ok(())
			}
			&Instruction::JZ => {
				unimplemented!()
				// Ok(())
			}
			&Instruction::JNZ => {
				unimplemented!()
				// Ok(())
			}
			&Instruction::HLT => {
				self.running = false;
				Ok(())
			}
			&Instruction::NOP => {
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use instructions::Instruction;

	#[test]
	fn psh() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &5);
	}
	#[test]
	fn pop() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5),Instruction::PSH(10), Instruction::POP, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &5);
	}
	#[test]
	fn add() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5), Instruction::PSH(10),Instruction::ADD, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &15);
	}
	#[test]
	fn sub() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5), Instruction::PSH(10),Instruction::SUB, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &5);
	}
	#[test]
	fn mul() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5), Instruction::PSH(10),Instruction::MUL, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &50);
	}
	#[test]
	fn div() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5), Instruction::PSH(10),Instruction::DIV, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &2);
	}

	#[test]
	#[should_panic(expected = "VMError: StackError on ip 2")]
	fn add_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5),Instruction::ADD, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 2")]
	fn sub_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5),Instruction::SUB, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 2")]
	fn mul_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5),Instruction::MUL, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 2")]
	fn div_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5),Instruction::DIV, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: ZeroDivision on ip 3")]
	fn zerodivision() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(0),Instruction::PSH(10),Instruction::DIV, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	fn set() {
		let mut vm = VM::new();
		let program = vec![Instruction::SET(1, 15), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.registers[1], 15);
	}
	#[test]
	fn mov() {
		let mut vm = VM::new();
		let program = vec![Instruction::SET(1, 15), Instruction::MOV(1, 2), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.registers[2], 15);
	}
	#[test]
	fn ldr() {
		let mut vm = VM::new();
		let program = vec![Instruction::SET(1, 15), Instruction::LDR(1), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &15);
	}
	#[test]
	fn str() {
		let mut vm = VM::new();
		let program = vec![Instruction::PSH(5), Instruction::STR(1), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.registers[1], 5);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 1")]
	fn str_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::STR(1), Instruction::HLT];
		vm.run(program, false);

	}
}

