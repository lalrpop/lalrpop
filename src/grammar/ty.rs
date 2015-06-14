/*!
 * Simple representation of Rust types. Really only understands two
 * things: named types, and tuples.
 */

use intern::{self, InternedString};

#[derive(Clone, Debug)]
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

