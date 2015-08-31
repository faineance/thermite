use std::fmt;
use std::collections::HashMap;
use instructions::Instruction;
const REG_SIZE: usize = 6; 


pub type VMResult<T> = Result<T, VMError>;

#[derive(Debug,PartialEq)]
enum VMError {
	ZeroDivision,
	MissingExitInstruction,
	MissingMainLabel,
	UndefinedLabel,
}
impl fmt::Debug for VM {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Registers: \n {:?} \n Jump Map:\n {:?}", self.registers,  self.jump_map)
	}
}

pub struct VM {
	registers: [i32; REG_SIZE],
	ip: usize,
	jump_map: HashMap<String, usize>,
	running: bool,
}

impl VM {
	pub fn new() -> VM {
		VM { registers: [0; REG_SIZE], ip: 0, jump_map: HashMap::new(), running: true}
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
			&Instruction::NOP => {
				Ok(())
			}
			&Instruction::OUT(register) => {
				println!("{:?}", self.registers[register as usize]);
				Ok(())
			}
			&Instruction::ADD(source, target, destination) => {
				self.registers[destination as usize] = self.registers[source as usize] + self.registers[target as usize];
				Ok(())
			}
			&Instruction::SUB(source, target, destination) => {
				self.registers[destination as usize] = self.registers[source as usize] - self.registers[target as usize];
				Ok(())
			}
			&Instruction::MUL(source, target, destination) => {
				self.registers[destination as usize] = self.registers[source as usize] * self.registers[target as usize];
				Ok(())
			}
			&Instruction::DIV(source, target, destination) => {
				match self.registers[target as usize] {
					0 => return Err(VMError::ZeroDivision),
					_ => {
						self.registers[destination as usize] = self.registers[source as usize] / self.registers[target as usize];
						Ok(())
					}
				}
				
			}

			&Instruction::STR(value, register) => {
				self.registers[register  as usize] = value;
				Ok(())
			}
			&Instruction::JMP(ref loc) => {
				match self.jump_map.get(loc) {
					Some(&ip) => self.ip = ip,
					_ => return Err(VMError::UndefinedLabel),
				}
				Ok(())
			}
			&Instruction::JZ(register, ref loc) => {
				
				if self.registers[register as usize] == 0 {
					match self.jump_map.get(loc) {
						Some(&ip) => self.ip = ip,
						_ => return Err(VMError::UndefinedLabel),
					}
				}

				Ok(())
			}
			&Instruction::JNZ(register, ref loc) => {
				if self.registers[register as usize] != 0 {
					match self.jump_map.get(loc) {
						Some(&ip) => self.ip = ip,
						_ => return Err(VMError::UndefinedLabel),
					}
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
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use instructions::Instruction;
	use registers::Register;

	// #[test]
	// fn add() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()), Instruction::Instruction::ADD, Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last().unwrap(), &15);
	// }
	// #[test]
	// fn sub() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()) ,Instruction::SUB, Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last().unwrap(), &5);
	// }
	// #[test]
	// fn mul() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::MUL, Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last().unwrap(), &50);
	// }
	// #[test]
	// fn div() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::DIV, Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last().unwrap(), &2);
	// }

	// #[test]
	// #[should_panic(expected = "VMError: StackError on ip 3")]
	// fn add_stackerror() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::ADD, Instruction::HLT];
	// 	vm.run(program, false);
	// }
	// #[test]
	// #[should_panic(expected = "VMError: StackError on ip 3")]
	// fn sub_stackerror() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::SUB, Instruction::HLT];
	// 	vm.run(program, false);
	// }
	// #[test]
	// #[should_panic(expected = "VMError: StackError on ip 3")]
	// fn mul_stackerror() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::MUL, Instruction::HLT];
	// 	vm.run(program, false);
	// }
	// #[test]
	// #[should_panic(expected = "VMError: StackError on ip 3")]
	// fn div_stackerror() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::DIV, Instruction::HLT];
	// 	vm.run(program, false);
	// }
	// #[test]
	// #[should_panic(expected = "VMError: ZeroDivision on ip 4")]
	// fn zerodivision() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()),Instruction::DIV, Instruction::HLT];
	// 	vm.run(program, false);
	// }
	// #[test]
	// fn jmp() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()), Instruction::JMP("test".to_string()), Instruction::LBL("test".to_string()), Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last().unwrap(), &10);
	// }
	// #[test]
	// fn jz() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()), Instruction::JZ("test".to_string()), Instruction::LBL("test".to_string()), Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last(), None);
	// }
	// #[test]
	// fn jnz() {
	// 	let mut vm = VM::new();
	// 	let program = vec![Instruction::LBL("main".to_string()), Instruction::JNZ("test".to_string()), Instruction::LBL("test".to_string()), Instruction::HLT];
	// 	vm.run(program, false);
	// 	assert_eq!( vm.stack.last(), None);
	// }

}

