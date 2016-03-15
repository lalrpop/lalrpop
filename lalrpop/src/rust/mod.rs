//! Simple Rust AST. This is what the various code generators create,
//! which then gets serialized.

use grammar::repr::Grammar;
use std::io::{self, Write};

macro_rules! rust {
    ($w:expr, $($args:tt)*) => {
        try!(($w).writeln(&::std::fmt::format(format_args!($($args)*))))
    }
}

/// A wrapper around a Write instance that handles indentation for
/// Rust code. It expects Rust code to be written in a stylized way,
/// with lots of braces and newlines (example shown here with no
/// indentation). Over time maybe we can extend this to make things
/// look prettier, but seems like...meh, just run it through some
/// rustfmt tool.
///
/// ```ignore
/// fn foo(
/// arg1: Type1,
/// arg2: Type2,
/// arg3: Type3)
/// -> ReturnType
/// {
/// match foo {
/// Variant => {
/// }
/// }
/// }
/// ```
pub struct RustWrite<W: Write> {
    write: W,
    indent: usize,
}

const TAB: usize = 4;

impl<W:Write> RustWrite<W> {
    pub fn new(w: W) -> RustWrite<W> {
        RustWrite { write: w, indent: 0 }
    }

    pub fn into_inner(self) -> W {
        self.write
    }

    fn write_indented(&mut self, out: &str) -> io::Result<()> {
        writeln!(self.write, "{0:1$}{2}", "", self.indent, out)
    }

    pub fn writeln(&mut self, out: &str) -> io::Result<()> {
        let buf = out.as_bytes();

        // pass empty lines through with no indentation
        if buf.is_empty() {
            return self.write.write_all("\n".as_bytes());
        }

        let n = buf.len() - 1;

        // If the line begins with a `}`, `]`, or `)`, first decrement the indentation.
        if buf[0] == ('}' as u8) || buf[0] == (']' as u8) || buf[0] == (')' as u8) {
            self.indent -= TAB;
        }

        try!(self.write_indented(out));

        // Detect a line that ends in a `{` or `(` and increase indentation for future lines.
        if buf[n] == ('{' as u8) || buf[n] == ('[' as u8) || buf[n] == ('(' as u8) {
            self.indent += TAB;
        }

        Ok(())
    }

    pub fn write_pub_fn_header(&mut self,
                               grammar: &Grammar,
                               name: String,
                               type_parameters: Vec<String>,
                               parameters: Vec<String>,
                               return_type: String,
                               where_clauses: Vec<String>)
                               -> io::Result<()>
    {
        self.write_fn_header_helper(grammar, "pub ", name, type_parameters,
                                    parameters, return_type, where_clauses)
    }

    pub fn write_fn_header_helper(&mut self,
                              grammar: &Grammar,
                              qualifiers: &str,
                              name: String,
                              type_parameters: Vec<String>,
                              parameters: Vec<String>,
                              return_type: String,
                              where_clauses: Vec<String>)
                              -> io::Result<()>
    {

        self.write_fn_header_helper2(qualifiers,
                                    name,
                                    grammar.type_parameters.iter().map(|p| format!("{}", p)).chain(type_parameters.iter().cloned()).collect(),
                                    grammar.parameters.iter().map(|p| format!("{}", p)).chain(parameters.iter().cloned()).collect(),
                                    return_type,
                                    grammar.where_clauses.iter().map(|p| format!("{}", p)).chain(where_clauses.iter().cloned()).collect())
    }

    pub fn write_fn_header_helper2(&mut self,
                              qualifiers: &str,
                              name: String,
                              type_parameters: Vec<String>,
                              parameters: Vec<String>,
                              return_type: String,
                              where_clauses: Vec<String>)
                              -> io::Result<()>
    {
        rust!(self, "{}fn {}<", qualifiers, name);

        for type_parameter in &type_parameters {
            rust!(self, "{0:1$}{2},", "", TAB, type_parameter);
        }

        rust!(self, ">(");

        for parameter in &parameters {
            rust!(self, "{},", parameter);
        }

        if !where_clauses.is_empty() {
            rust!(self, ") -> {} where", return_type);

            for where_clause in &where_clauses {
                rust!(self, "  {},", where_clause);
            }
        } else {
            rust!(self,") -> {}", return_type);
        }

        Ok(())
    }

    pub fn write_uses(&mut self,
                      super_prefix: &str,
                      grammar: &Grammar)
                      -> io::Result<()>
    {
        // things the user wrote
        for u in &grammar.uses {
            if u.starts_with("super::") {
                rust!(self, "use {}{};", super_prefix, u);
            } else {
                rust!(self, "use {};", u);
            }
        }

        self.write_standard_uses(&grammar.prefix)
    }

    pub fn write_standard_uses(&mut self, prefix: &str) -> io::Result<()> {
        // stuff that we plan to use
        rust!(self, "extern crate lalrpop_util as {}lalrpop_util;",
              prefix);
        rust!(self, "use self::{}lalrpop_util::ParseError as {}ParseError;",
              prefix, prefix);

        Ok(())
    }
}
