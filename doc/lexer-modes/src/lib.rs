pub mod lexer;

#[cfg(test)]
mod test;

// A value.
#[derive(Debug, PartialEq, Eq)]
pub struct Value {
    pub value: Vec<u8>,
}

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);
pub use grammar::ListParser;
