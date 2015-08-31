#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    RA,
    RB,
    RC,
    RD,
    RE,
    RF,
}

impl From<String> for Register {
    fn from(string: String) -> Register {
        match string.as_ref() {

            "ra" => Register::RA,
            "rb" => Register::RB,
            "rc" => Register::RC,
            "rd" => Register::RD,
            "re" => Register::RE,
            "rf" => Register::RF,
            _ => unreachable!(),

        }
    }
}

