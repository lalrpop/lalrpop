//! The grammar definition.

mod token;

pub use self::token::TokenDefinition;

#[derive(Clone)]
pub struct TypeName {
    pub module: Vec<String>,
    pub type_name: String,
    pub parameters: Vec<String>,
}

impl TypeName {
    pub fn new(module: Vec<String>,
               type_name: String,
               parameters: Vec<String>)
               -> TypeName
    {
        TypeName { module: module, type_name: type_name, parameters: parameters }
    }

    pub fn path(&self) -> String {
        if self.module.is_empty() {
            format!("::{}",
                    self.type_name)
        } else {
            format!("::{}::{}",
                    self.module.connect("::"),
                    self.type_name)
        }
    }

    pub fn reference(&self) -> String {
        format!("{}<{}>", self.path(), self.parameters.connect(", "))
    }
}

///////////////////////////////////////////////////////////////////////////
// A grammar looks like:
//
// - A series of NONTERMINAL definitions
