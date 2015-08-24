/*!

Generates an iterator type `__Matcher` that looks roughly like

```ignore
mod __intern_token {
    pub struct __Matcher<'input> { .. }

    fn __tokenize(text: &str) -> Option<(usize, usize)> { ... }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), ParseError>;
        //                  ~~~~~   ~~~~~  ~~~~~~~~~~~   ~~~~~
        //                  start   token  token         end
        //                          index  text
    }
}
```

 */

use lexer::dfa::codegen;
use grammar::parse_tree::InternToken;
use rust::RustWrite;
use std::io::{self, Write};

pub fn compile<W: Write>(
    prefix: &str,
    intern_token: &InternToken,
    out: &mut RustWrite<W>)
    -> io::Result<()>
{
    rust!(out, "mod {}intern_token {{", prefix);
    rust!(out, "pub struct {}Matcher<'input> {{", prefix);
    rust!(out, "text: &'input str,"); // remaining input
    rust!(out, "consumed: usize,"); // number of chars consumed thus far
    rust!(out, "}}");
    rust!(out, "");
    try!(codegen::compile_tokenize_fn(prefix, &intern_token.dfa, out));
    rust!(out, "");
    rust!(out, "impl<'input> {}Matcher<'input> {{", prefix);
    rust!(out, "    pub fn new(s: &'input str) -> {}Matcher<'input> {{", prefix);
    rust!(out, "        {}Matcher {{ text: s, consumed: 0 }}", prefix);
    rust!(out, "    }}");
    rust!(out, "}}");
    rust!(out, "");
    rust!(out, "impl<'input> Iterator for {}Matcher<'input> {{", prefix);
    rust!(out, "type Item = Result<(usize, (usize, &'input str), usize), ParseError>;");
    rust!(out, "");
    rust!(out, "fn next(&mut self) -> Self::Item {{");

    // start by trimming whitespace from left
    rust!(out, "let text = self.text.trim_left();");
    rust!(out, "let whitespace = self.text.len() - text.len();");
    rust!(out, "let start_offset = self.consumed + whitespace;");

    // if nothing left, return None
    rust!(out, "if text.is_empty() {{");
    rust!(out, "self.text = text;");
    rust!(out, "self.consumed = start_offset;");
    rust!(out, "return None;");
    rust!(out, "}}");

    // otherwise, tokenize
    rust!(out, "match {}tokenize(text) {{", prefix);
    rust!(out, "Some(({}index, {}length)) => {{", prefix, prefix);
    rust!(out, "let result = &text[..{}length];", prefix);
    rust!(out, "let remaining = &text[{}length..];", prefix);
    rust!(out, "let end_offset = start_offset + length;");
    rust!(out, "self.text = remaining");
    rust!(out, "self.consumed = end_offset;");
    rust!(out, "Some(Ok((start_offset, ({}index, result), end_offset)))", prefix);
    rust!(out, "}}"); // some
    rust!(out, "None => {{");
    rust!(out, "Some(Err(InvalidToken {{ location: start_offset }}))");
    rust!(out, "}}"); // none
    rust!(out, "}}"); // match

    rust!(out, "}}"); // fn
    rust!(out, "}}"); // impl
    rust!(out, "}}"); // mod

    Ok(())
}

