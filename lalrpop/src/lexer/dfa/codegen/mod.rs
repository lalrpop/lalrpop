use lexer::dfa::*;
use rust::RustWrite;
use std::char;
use std::io::{self, Write};

#[cfg(test)]
mod test;

/// Generates a fn `__tokenize` based on the given DFA with the following signature:
///
/// ```ignore
/// fn tokenize(text: &str) -> Option<(usize, usize)>
/// ```
///
/// This function returns `None` if there is no matching
/// token. Otherwise, it returns the pair of (NFA index, length) for
/// the next token.
pub fn compile_tokenize_fn<W: Write>(
    prefix: &str,
    dfa: &DFA,
    out: &mut RustWrite<W>)
    -> io::Result<()>
{
    let mut matcher = Matcher { prefix: prefix, dfa: dfa, out: out };
    try!(matcher.tokenize());
    Ok(())
}

struct Matcher<'m, W: Write+'m> {
    prefix: &'m str,
    dfa: &'m DFA,
    out: &'m mut RustWrite<W>,
}

impl<'m,W> Matcher<'m,W>
    where W: Write
{
    fn tokenize(&mut self) -> io::Result<()> {
        rust!(self.out, "fn {}tokenize(text: &str) -> Option<(usize, usize)> {{",
              self.prefix);
        rust!(self.out, "let mut {}chars = text.char_indices();", self.prefix);
        rust!(self.out, "let mut {}current_match: Option<(usize, usize)> = None;", self.prefix);
        rust!(self.out, "let mut {}current_state: usize = 0;", self.prefix);
        rust!(self.out, "loop {{");
        rust!(self.out, "match {}current_state {{", self.prefix);

        for (index, state) in self.dfa.states.iter().enumerate() {
            rust!(self.out, "{} => {{", index);
            try!(self.state(state));
            rust!(self.out, "}}");
        }

        rust!(self.out, "_ => {{ panic!(\"invalid state {{}}\", {}current_state); }}",
              self.prefix);
        rust!(self.out, "}}");
        rust!(self.out, "}}");
        rust!(self.out, "}}");
        Ok(())
    }

    fn state(&mut self, state: &State) -> io::Result<()> {
        // this could be pulled to the top of the loop, but we want to
        // encourage LLVM to convert the loop+switch pair into actual
        // gotos.
        rust!(self.out, "let ({}index, {}ch) = \
                         match {}chars.next() {{ Some(p) => p, \
                         None => return {}current_match }};",
              self.prefix, self.prefix, self.prefix, self.prefix);
        rust!(self.out, "match {}ch as u32 {{", self.prefix);
        for &(test, target_state) in &state.test_edges {
            if test.len() == 1 {
                match char::from_u32(test.start) {
                    Some(ch) => {
                        rust!(self.out, "{} => /* {:?} */ {{", ch as u32, ch);
                        let index = format!("{}index + {}", self.prefix, ch.len_utf8());
                        try!(self.transition(target_state, &index));
                        rust!(self.out, "}}");
                    }
                    None => {
                        // if somehow we have a singleton range that does not
                        // contain valid unicode, can just ignore it;
                        // it will never happen in practice
                    }
                }
            } else {
                rust!(self.out, "{:?} ... {:?} => {{", test.start, test.end - 1);
                let index = format!("{}index + {}ch.len_utf8()",
                                    self.prefix, self.prefix);
                try!(self.transition(target_state, &index));
                rust!(self.out, "}}");
            }
        }
        rust!(self.out, "_ => {{");
        let index = format!("{}index + {}ch.len_utf8()", self.prefix, self.prefix);
        try!(self.transition(state.other_edge, &index));
        rust!(self.out, "}}");
        rust!(self.out, "}}");
        Ok(())
    }

    fn transition(&mut self,
                  target_state: DFAStateIndex,
                  index: &str)
                  -> io::Result<()> {
        match self.dfa.state(target_state).kind {
            Kind::Accepts(nfa) => {
                rust!(self.out, "{}current_match = Some(({}, {}));",
                      self.prefix, nfa.index(), index)
            }
            Kind::Neither => { }
            Kind::Reject => {
                rust!(self.out, "return {}current_match;", self.prefix);
                return Ok(());
            }
        }

        rust!(self.out, "{}current_state = {};", self.prefix, target_state.index());
        rust!(self.out, "continue;");
        Ok(())
    }
}
