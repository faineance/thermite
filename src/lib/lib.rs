#![crate_name = "thermite"]
#![crate_type = "lib"]

pub mod vm;
pub mod lexer;
pub mod parser;
pub mod instructions;
pub mod registers;

pub mod disassembler;

// pub trait To<T> {
//     type Err;
//     fn to(T) -> Result<Self, Self::Err>;
// }

// pub trait From<T> {
//     type Err;
//     fn from(T) -> Result<Self, Self::Err>;
// }