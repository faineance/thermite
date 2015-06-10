use std::fmt;
use instructions::Instruction;
const STACK_SIZE: usize = 256;
const REG_SIZE: usize = 16; 


#[derive(Debug,PartialEq)]
enum VMErrorKind {
	StackError,
	UndefinedLabel,
	ZeroDivision,
}
impl fmt::Debug for VM {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Registers: \n {:?} \n Stack Contents:\n {:?}", self.registers, self.stack)
	}
}

pub struct VM {
	stack: Vec<i32>,
	registers: [u8; REG_SIZE],
	ip: usize,
	running: bool,
	debug: bool,
}

impl VM {
	pub fn new() -> VM {
		VM { stack: Vec::with_capacity(STACK_SIZE), registers: [0; REG_SIZE], ip: 0, running: true, debug: true}
	}
	pub fn run(&mut self, program: Vec<Instruction>) {

		while self.running {
			let instruction = &program[self.ip];

			match self.eval(instruction) {
				Ok(_) => {},
				Err(e) => {
					
					println!("Registers: \n {:?}",self.registers);
					println!("Stack Contents:\n {:?}",self.stack);
					panic!("VMError: {:?} on ip {:?}",e, self.ip + 1);	
				}
			}
			self.ip += 1;
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
					Some(val) => Ok(()),
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
				self.registers[register as usize] = value as u8;
				Ok(())
			}
			&Instruction::MOV(register1, register2) => {
				self.registers[register1 as usize] = self.registers[register2 as usize];
				Ok(())
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