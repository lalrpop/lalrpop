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
    for (index, state) in dfa.states.iter().enumerate() {
        try!(matcher.state(DFAStateIndex(index), state));
    }
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
    fn state(&mut self, index: DFAStateIndex, state: &State) -> io::Result<()> {
        rust!(self.out, "fn {}state{}<'input,{}CHARS>(", self.prefix, index, self.prefix);
        rust!(self.out, "mut {}chars: {}CHARS,", self.prefix, self.prefix);
        rust!(self.out, "{}current_match: Option<(usize, usize)>,", self.prefix);
        rust!(self.out, ") -> Option<(usize, usize)>");
        rust!(self.out, "where {}CHARS: Iterator<Item=(usize, char)>", self.prefix);
        rust!(self.out, "{{");
        rust!(self.out, "let ({}index, {}ch) = \
                         match {}chars.next() {{ Some(p) => p, None => return {}current_match }};",
              self.prefix, self.prefix, self.prefix, self.prefix);
        rust!(self.out, "match {}ch {{", self.prefix);
        for &(test, target_state) in &state.test_edges {
            match test {
                Test::Char(ch) => {
                    rust!(self.out, "{:?} => {{", ch);
                    let index = format!("{}index + {}", self.prefix, ch.len_utf8());
                    try!(self.transition(target_state, "chars", &index, "current_match"));
                    rust!(self.out, "}}");
                }
            }
        }
        rust!(self.out, "_ => {{");
        let index = format!("{}index + {}ch.len_utf8()", self.prefix, self.prefix);
        try!(self.transition(state.other_edge, "chars", &index, "current_match"));
        rust!(self.out, "}}");
        rust!(self.out, "}}");
        rust!(self.out, "}}");
        Ok(())
    }

    fn transition(&mut self,
                  target_state: DFAStateIndex,
                  chars: &str,
                  index: &str,
                  current_match: &str)
                  -> io::Result<()> {
        let next_match = match self.dfa.state(target_state).kind {
            Kind::Accepts(nfa) => {
                format!("Some(({}, {}))", nfa.index(), index)
            }
            Kind::Neither => {
                format!("{}{}", self.prefix, current_match)
            }
            Kind::Reject => {
                rust!(self.out, "{}{}", self.prefix, current_match);
                return Ok(());
            }
        };

        rust!(self.out, "{}state{}(", self.prefix, target_state);
        rust!(self.out, "{}{},", self.prefix, chars);
        rust!(self.out, "{},", next_match);
        rust!(self.out, ")");
        Ok(())
    }
}
