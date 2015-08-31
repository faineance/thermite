use registers::Register;
use std::fmt;


impl fmt::Display for Register {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
            &Register::RA => write!(f, "ra"),
            &Register::RB => write!(f, "rb"),
            &Register::RC => write!(f, "rc"),
            &Register::RD => write!(f, "rd"),
            &Register::RE => write!(f, "re"),
            &Register::RF => write!(f, "rf"),
        }
    }
}