use lexer::dfa::*;
use lexer::re::Test;
use rust::RustWrite;
use std::io::{self, Write};

#[cfg(test)]
mod test;

// We generate matcher code that uses recursion. This is in a sense a
// bit odd because we'll recurse O(n) deep where n is the length of
// the longest token, but it's also quite convenient, because as we
// unwind stack we can find which was the longest applicable regular
// expression.

pub fn compile<W: Write>(
    prefix: &str,
    dfa: &DFA,
    out: &mut RustWrite<W>)
    -> io::Result<()>
{
    let mut matcher = Matcher { prefix: prefix, dfa: dfa, out: out };
    matcher.tokenize();
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
        rust!(self.out, "fn {}tokenize<'input,{}CHARS>(", self.prefix, self.prefix);
        rust!(self.out, "mut {}chars: {}CHARS,", self.prefix, self.prefix);
        rust!(self.out, ") -> Option<(usize, usize)>");
        rust!(self.out, "where {}CHARS: Iterator<Item=(usize, char)>", self.prefix);
        rust!(self.out, "{{");
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
                         match {}chars.next() {{ Some(p) => p, None => return {}current_match }};",
              self.prefix, self.prefix, self.prefix, self.prefix);
        rust!(self.out, "match {}ch {{", self.prefix);
        for &(test, target_state) in &state.test_edges {
            match test {
                Test::Char(ch) => {
                    rust!(self.out, "{:?} => {{", ch);
                    let index = format!("{}index + {}", self.prefix, ch.len_utf8());
                    try!(self.transition(target_state, &index));
                    rust!(self.out, "}}");
                }
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
