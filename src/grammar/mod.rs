//! The grammar definition.

mod token;
use intern::{self, InternedString};

pub use self::token::TokenDefinition;

#[derive(Clone)]
pub struct TypeName {
    pub module: Vec<InternedString>,
    pub type_name: InternedString,
    pub parameters: Vec<InternedString>,
}

impl TypeName {
    pub fn new(module: Vec<InternedString>,
               type_name: InternedString,
               parameters: Vec<InternedString>)
               -> TypeName
    {
        TypeName { module: module, type_name: type_name, parameters: parameters }
    }

    pub fn path(&self) -> String {
        if self.module.is_empty() {
            format!("::{}", self.type_name)
        } else {
            format!("::{}::{}", connect(&self.module, "::"), self.type_name)
        }
    }

    pub fn reference(&self) -> String {
        format!("{}<{}>", self.path(), connect(&self.parameters, ", "))
    }
}

fn connect(strs: &[InternedString], sep: &str) -> String {
    let mut buf = String::new();
    intern::read(|interner| {
        let mut iter = strs.iter();
        if let Some(&v) = iter.next() {
            buf.push_str(interner.data(v));
            while let Some(&v) = iter.next() {
                buf.push_str(sep);
                buf.push_str(interner.data(v));
            }
        }
    });
    buf
}

///////////////////////////////////////////////////////////////////////////
// A grammar looks like:
//
// - A series of NONTERMINAL definitions
