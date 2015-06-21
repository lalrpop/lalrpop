//! Simple Rust AST. This is what the various code generators create,
//! which then gets serialized.

use std::io::{self, Write};
use std::fmt;

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
/// ```
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

enum State {
    FnHeader,
    CodeBlock,
}

const TAB: usize = 4;

impl<W:Write> RustWrite<W> {
    pub fn new(w: W) -> RustWrite<W> {
        RustWrite { write: w, indent: 0 }
    }

    fn write_indented(&mut self, out: &str, indent: usize) -> io::Result<()> {
        writeln!(self.write, "{0:1$}{2}", "", indent, out)
    }

    pub fn writeln(&mut self, out: &str) -> io::Result<()> {
        let buf = out.as_bytes();

        // pass empty lines through with no indentation
        if buf.is_empty() {
            return self.write.write_all("\n".as_bytes());
        }

        let n = buf.len() - 1;

        // Check for an opening brace all on its own. We only expect this to occur
        // as part of a fn header. As a special exception, print it at one TAB less
        // than normal but leave the indent otherwise unchanged.
        if buf[0] == ('{' as u8) && buf[n] == ('\n' as u8) {
            let indent = self.indent - TAB;
            return self.write_indented(out, indent);
        }

        // If the line begins with a `}`, `]`, or `)`, first decrement the indentation.
        if buf[0] == ('}' as u8) || buf[0] == (']' as u8) || buf[0] == (')' as u8) {
            self.indent -= TAB;
        }

        let indent = self.indent;
        try!(self.write_indented(out, indent));

        // Detect a line that ends in a `{` or `(` and increase indentation for future lines.
        if buf[n] == ('{' as u8) || buf[n] == ('[' as u8) || buf[n] == ('(' as u8) {
            self.indent += TAB;
        }

        Ok(())
    }
}
