use std::fmt;
use std::collections::HashMap;
use instructions::Instruction;
const STACK_SIZE: usize = 256;
const REG_SIZE: usize = 16; 


pub type VMResult<T> = Result<T, VMError>;
#[derive(Debug,PartialEq)]
enum VMError {
	StackError,
	ZeroDivision,
	MissingExitInstruction,
	MissingMainLabel,
	UndefinedLabel,
}
impl fmt::Debug for VM {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Registers: \n {:?} \nStack Contents:\n {:?} \n Jump Map:\n {:?}", self.registers, self.stack, self.jump_map)
	}
}

pub struct VM {
	stack: Vec<i32>,
	registers: [i32; REG_SIZE],
	ip: usize,
	jump_map: HashMap<String, usize>,
	running: bool,
}

impl VM {
	pub fn new() -> VM {
		VM { stack: Vec::with_capacity(STACK_SIZE), registers: [0; REG_SIZE], ip: 0, jump_map: HashMap::new(), running: true}
	}
	pub fn run(&mut self, program: Vec<Instruction>, repl: bool) {
		if !repl {
			self.jump_map = self.build_jump_map(program.clone()); 
			match self.jump_map.get(&"main".to_string()) {
				Some(&ip) => self.ip = ip,
				_ => panic!("VMError: {:?}", VMError::MissingMainLabel),
			}
			match program.iter().position(|hlt| *hlt == Instruction::HLT) {
				Some(_) => {},
				_ => panic!("VMError: {:?}", VMError::MissingExitInstruction),
			}
		}
		while self.running {
			let instruction = &program[self.ip];
			
			match self.eval(instruction) {
				Ok(_) => {},
				Err(e) => {
					println!("{:?}", self);
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
	fn build_jump_map(&mut self, program: Vec<Instruction>) -> HashMap< String, usize> {
		let mut jump_map: HashMap<String, usize> = HashMap::new();
		
		for (position, instruction) in program.iter().enumerate() {
			match instruction {
				&Instruction::LBL(ref s) => jump_map.insert(s.clone(),position),
				_ => Some(0),
			};
			
		}
		jump_map
	}
	fn eval(&mut self, instruction: &Instruction) -> VMResult<()> {
		match instruction {
			&Instruction::OUT => {
				println!("{:?}", self.stack.pop().unwrap() );
				Ok(())
			}
			&Instruction::PSH(value) => {
				self.stack.push(value);
				Ok(())
			}
			&Instruction::POP => {
				match self.stack.pop() {
					Some(_) => Ok(()),
					_ => Err(VMError::StackError)
				}
			}
			&Instruction::ADD => {
				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						self.stack.push(a + b); 
						Ok(())
					},
					_ => Err(VMError::StackError)
				}
			}
			&Instruction::SUB => {
				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						self.stack.push(a - b); 
						Ok(())
					},
					_ => Err(VMError::StackError)
				}

			}
			&Instruction::MUL => {
				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						self.stack.push(a * b); 
						Ok(())
					},
					_ => Err(VMError::StackError)
				}
			}
			&Instruction::DIV => {

				match (self.stack.pop(), self.stack.pop()) {
					(Some(a), Some(b)) => { 
						if b == 0 {
							return Err(VMError::ZeroDivision)
						} else { 
							self.stack.push(a / b);
						}
						Ok(())
						
					},
					_ => Err(VMError::StackError)
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
					_ => Err(VMError::StackError)
				}
			}
			&Instruction::JMP(ref loc) => {
				match self.jump_map.get(loc) {
					Some(&ip) => self.ip = ip,
					_ => return Err(VMError::UndefinedLabel),
				}
				Ok(())
			}
			&Instruction::JZ(ref loc) => {
				match self.stack.pop() {
					Some(value) if value == 0 => {
						match self.jump_map.get(loc) {
							Some(&ip) => self.ip = ip,
							_ => return Err(VMError::UndefinedLabel),
						}
					},
					_ => return Ok(())
				}
				Ok(())
			}
			&Instruction::JNZ(ref loc ) => {
				match self.stack.pop() {
					Some(value) if value != 0 => {
						match self.jump_map.get(loc) {
							Some(&ip) => self.ip = ip,
							_ => return Err(VMError::UndefinedLabel),
						}
					},
					_ => return Ok(())
				}
				Ok(())
			}
			&Instruction::LBL(ref loc) => {
				
				Ok(())
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
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &5);
	}
	#[test]
	fn pop() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5),Instruction::PSH(10), Instruction::POP, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &5);
	}
	#[test]
	fn add() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5), Instruction::PSH(10),Instruction::ADD, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &15);
	}
	#[test]
	fn sub() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5), Instruction::PSH(10),Instruction::SUB, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &5);
	}
	#[test]
	fn mul() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5), Instruction::PSH(10),Instruction::MUL, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &50);
	}
	#[test]
	fn div() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5), Instruction::PSH(10),Instruction::DIV, Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &2);
	}

	#[test]
	#[should_panic(expected = "VMError: StackError on ip 3")]
	fn add_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5),Instruction::ADD, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 3")]
	fn sub_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5),Instruction::SUB, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 3")]
	fn mul_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5),Instruction::MUL, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: StackError on ip 3")]
	fn div_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5),Instruction::DIV, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	#[should_panic(expected = "VMError: ZeroDivision on ip 4")]
	fn zerodivision() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(0),Instruction::PSH(10),Instruction::DIV, Instruction::HLT];
		vm.run(program, false);
	}
	#[test]
	fn set() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::SET(1, 15), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.registers[1], 15);
	}
	#[test]
	fn mov() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::SET(1, 15), Instruction::MOV(1, 2), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.registers[2], 15);
	}
	#[test]
	fn ldr() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::SET(1, 15), Instruction::LDR(1), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &15);
	}
	#[test]
	fn str() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(5), Instruction::STR(1), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.registers[1], 5);
	}
	#[test]
	fn jmp() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(10), Instruction::JMP("test".to_string()),Instruction::PSH(5), Instruction::LBL("test".to_string()), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last().unwrap(), &10);
	}
	#[test]
	fn jz() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(0), Instruction::JZ("test".to_string()),Instruction::PSH(5), Instruction::LBL("test".to_string()), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last(), None);
	}
	#[test]
	fn jnz() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::PSH(1), Instruction::JNZ("test".to_string()),Instruction::PSH(5), Instruction::LBL("test".to_string()), Instruction::HLT];
		vm.run(program, false);
		assert_eq!( vm.stack.last(), None);
	}

	#[test]
	#[should_panic(expected = "VMError: StackError on ip 2")]
	fn str_stackerror() {
		let mut vm = VM::new();
		let program = vec![Instruction::LBL("main".to_string()),Instruction::STR(1), Instruction::HLT];
		vm.run(program, false);

	}
}

