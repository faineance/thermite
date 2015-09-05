use std::fmt;
use std::collections::HashMap;
use std::cmp;
use instructions::{IOType, ArithmaticType, BitwiseType, ShiftType, BranchType, ControlType, AssignmentType};
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
    program: Vec<Instruction>,
    registers: [i32; REG_SIZE],
    ip: usize,
    jump_map: HashMap<String, usize>,
    running: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {  program: Vec::new(), registers: [0; REG_SIZE], ip: 0, jump_map: HashMap::new(), running: true}
    }
    pub fn run(&mut self, program: Vec<Instruction>) {
        
        self.jump_map = self.build_jump_map(program.clone()); 
        match self.jump_map.get(&"main".to_string()) {
            Some(&ip) => self.ip = ip,
            _ => panic!("VMError: {:?}", VMError::MissingMainLabel),
        }
        match program.iter().position(|hlt| *hlt == Instruction::HLT) {
            Some(_) => {},
            _ => panic!("VMError: {:?}", VMError::MissingExitInstruction),
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
    }
    pub fn interactive(&mut self, instruction: Instruction) {
        self.program.push(instruction);
    
        let ref i = self.program[self.ip].clone();
        match i {
                &Instruction::Control(ref kind) => {
                    match kind {
                        &ControlType::LBL(ref s) => {
                            self.jump_map.insert(s.clone(),self.ip)
                        },
                    }
                },
                _ => Some(0)
        };
        match self.eval(&i) {
            Ok(_) => {},
            Err(e) => {
                println!("{:?}", self);
                println!("VMError: {:?} on ip {:?}", e, self.ip + 1); 
            }
        }
        self.ip += 1;
        
    }
    fn build_jump_map(&mut self, program: Vec<Instruction>) -> HashMap<String, usize> {
        let mut jump_map: HashMap<String, usize> = HashMap::new();

        for (position, instruction) in program.iter().enumerate() {
            match instruction {
                &Instruction::Control(ref kind) => {
                        match kind {
                            &ControlType::LBL(ref s) => {
                                jump_map.insert(s.clone(),position)
                            },
                        }
                },
                _ => Some(0),
            };

        }
        jump_map
    }
    fn eval(&mut self, instruction: &Instruction) -> VMResult<()> {
        match instruction {
            &Instruction::NOP => {
                Ok(())
            },
            &Instruction::HLT => {
                self.running = false;
                Ok(())
            }
            &Instruction::IO(ref kind, register) => {
                match kind {
                    &IOType::OUT => println!("{:?}", self.registers[register as usize]),
                    &IOType::IN => unimplemented!(),
                }
                Ok(())
            },
            &Instruction::Arithmatic(ref kind, source, target, destination) => {
                match kind {
                    &ArithmaticType::ADD => {
                        self.registers[destination as usize] = self.registers[source as usize] + self.registers[target as usize]
                    },
                    &ArithmaticType::SUB => {
                        self.registers[destination as usize] = self.registers[source as usize] - self.registers[target as usize]
                    },
                    &ArithmaticType::MUL => {
                        self.registers[destination as usize] = self.registers[source as usize] * self.registers[target as usize]
                    },
                    &ArithmaticType::DIV => {
                        match self.registers[target as usize] {
                            0 => return Err(VMError::ZeroDivision),
                            _ => {
                                self.registers[destination as usize] = self.registers[source as usize] / self.registers[target as usize];
                            }
                        }
                    },
                    &ArithmaticType::MAX => {
                        self.registers[destination as usize] = cmp::max(self.registers[source as usize], self.registers[target as usize])
                    },
                    &ArithmaticType::MIN => {
                        self.registers[destination as usize] = cmp::min(self.registers[source as usize],self.registers[target as usize])
                    },
                }
                Ok(())
            },
            &Instruction::Bitwise(ref kind, source, target, destination) => {
                match kind {
                    &BitwiseType::AND => {
                        self.registers[destination as usize] = self.registers[source as usize] & self.registers[target as usize]
                    },
                    &BitwiseType::OR => {
                        self.registers[destination as usize] = self.registers[source as usize] | self.registers[target as usize]
                    },
                    &BitwiseType::XOR => {
                        self.registers[destination as usize] = self.registers[source as usize] ^ self.registers[target as usize]
                    },
                    &BitwiseType::SHIFT(ref kind) => {
                        match kind {
                            &ShiftType::LEFT => {
                                 self.registers[destination as usize] = self.registers[source as usize] << self.registers[target as usize]
                            },
                            &ShiftType::RIGHT => {
                                 self.registers[destination as usize] = self.registers[source as usize] >> self.registers[target as usize]
                            },
                        }
                    }
                }
                Ok(())
            },
            &Instruction::Branch(ref kind, ref label) => {
                match kind {
                    &BranchType::UNCONDITIONAL => {
                        match self.jump_map.get(label) {
                            Some(&ip) => self.ip = ip,
                            _ => return Err(VMError::UndefinedLabel),
                        }
                    },
                    &BranchType::NOTZERO(register) => {
                        if self.registers[register as usize] != 0 {
                            match self.jump_map.get(label) {
                                Some(&ip) => self.ip = ip,
                                _ => return Err(VMError::UndefinedLabel),
                            }
                        }
                    },
                    &BranchType::ZERO(register) => {
                        if self.registers[register as usize] == 0 {
                            match self.jump_map.get(label) {
                                Some(&ip) => self.ip = ip,
                                _ => return Err(VMError::UndefinedLabel),
                            }
                        }

                    }
                }
                Ok(())
            },
            &Instruction::Control(ref kind) => {
                match kind {
                    &ControlType::LBL(ref label) => {
                    }
                }
                Ok(())
            }
            &Instruction::Assignment(ref kind, register) => {
                match kind {
                    &AssignmentType::STR(value) => self.registers[register  as usize] = value,
                    &AssignmentType::CPY(register2) => self.registers[register as usize] = self.registers[register2 as usize],
                }
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
    use instructions::{IOType, ArithmaticType, BitwiseType, ShiftType, BranchType, ControlType, AssignmentType};
    #[test]
    fn add() {
        let mut vm = VM::new();
        let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(10), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Arithmatic(ArithmaticType::ADD, Register::RA, Register::RB, Register::RC),
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RC as usize], 15);
    }
    #[test]
    fn sub() {
        let mut vm = VM::new();
        let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(10), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Arithmatic(ArithmaticType::SUB, Register::RA, Register::RB, Register::RC),
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RC as usize], 5);
    }
    #[test]
    fn mul() {
        let mut vm = VM::new();
        let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(10), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Arithmatic(ArithmaticType::MUL, Register::RA, Register::RB, Register::RC),
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RC as usize], 50);
    }
    #[test]
    fn div() {
        let mut vm = VM::new();
         let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(10), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Arithmatic(ArithmaticType::DIV, Register::RA, Register::RB, Register::RC),
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RC as usize], 2);
    }
    #[test]
    #[should_panic(expected = "VMError: ZeroDivision on ip 4")]
    fn zerodivision() {
        let mut vm = VM::new();
                    let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(10), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(0), Register::RB),
                    Instruction::Arithmatic(ArithmaticType::DIV, Register::RA, Register::RB, Register::RC),
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RC as usize], 2);
    }

    #[test]
    fn jmp() {
        let mut vm = VM::new();
        let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(10), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Branch(BranchType::UNCONDITIONAL, "test".to_string()),
                    Instruction::Arithmatic(ArithmaticType::ADD, Register::RA, Register::RB, Register::RB),
                    Instruction::Control(ControlType::LBL("test".to_string())), 
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RB as usize], 5);
    }
    #[test]
    fn jz() {
        let mut vm = VM::new();
        let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(0), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RC),
                    Instruction::Branch(BranchType::ZERO(Register::RA), "test".to_string()),
                    Instruction::Arithmatic(ArithmaticType::ADD, Register::RA, Register::RB, Register::RB),
                    Instruction::Control(ControlType::LBL("test".to_string())), 
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RB as usize], 5);
    }
    #[test]
    fn jnz() {
        let mut vm = VM::new();
        let program = vec![Instruction::Control(ControlType::LBL("main".to_string())), 
                    Instruction::Assignment(AssignmentType::STR(0), Register::RA),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RB),
                    Instruction::Assignment(AssignmentType::STR(5), Register::RC),
                    Instruction::Branch(BranchType::NOTZERO(Register::RB), "test".to_string()),
                    Instruction::Arithmatic(ArithmaticType::ADD, Register::RA, Register::RB, Register::RB),
                    Instruction::Control(ControlType::LBL("test".to_string())), 
                    Instruction::HLT];
        vm.run(program);
        assert_eq!( vm.registers[Register::RB as usize], 5);
    }
}

