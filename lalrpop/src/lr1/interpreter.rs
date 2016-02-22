use rust::RustWrite;

use lr1::core::*;
use lr1::lookahead::Lookahead;
use util::Map;

use grammar::repr::{Grammar, NonterminalString};

use std::io;
use std::iter;

pub struct Interpreter<'emitter,'grammar:'emitter> {
    /// the complete grammar
    grammar: &'grammar Grammar,

    /// the start symbol S the user specified
    user_start_symbol: NonterminalString,

    /// the synthetic start symbol S' that we specified
    start_symbol: NonterminalString,

    /// the vector of states
    states: &'emitter [State<'grammar>],

    /// where we write output
    out: &'emitter mut RustWrite<Vec<u8>>,

    nonterminal_bits: Map<NonterminalString, usize>,
}

pub fn compile<'emitter, 'grammar>(grammar: &'grammar Grammar,
                                   user_start_symbol: NonterminalString,
                                   start_symbol: NonterminalString,
                                   states: &'emitter [State<'grammar>],
                                   out: &'emitter mut RustWrite<Vec<u8>>)
                                   -> io::Result<()> {
    let mut interpreter = Interpreter::new(grammar, user_start_symbol, start_symbol, states, out);

    interpreter.write()
}

impl<'emitter, 'grammar> Interpreter<'emitter, 'grammar> {
    fn new(grammar: &'grammar Grammar,
               user_start_symbol: NonterminalString,
               start_symbol: NonterminalString,
               states: &'emitter [State<'grammar>],
               out: &'emitter mut RustWrite<Vec<u8>>)
               -> Interpreter<'emitter, 'grammar>
    {
        Interpreter {
            grammar: grammar,
            user_start_symbol: user_start_symbol,
            start_symbol: start_symbol,
            states: states,
            out: out,
            nonterminal_bits: grammar.nonterminals.keys().cloned().zip(0..).collect(),
        }
    }

    fn write_action_table(&mut self) -> io::Result<()> {
        let row_len = self.grammar.all_terminals.len()+1;

        let rows = self.states.iter().map(|s| {
            let mut row = iter::repeat("0".to_owned()).take(row_len).collect::<Vec<_>>();
            for (lookahead, action) in &s.tokens {
                let target = match *action {
                    Action::Shift(index) => format!("{}", (index.0 as i32) + 1),
                    Action::Reduce(..) => format!("-1") // TODO
                };

                let column_index = match *lookahead {
                    Lookahead::Terminal(ref s) =>
                        *self.grammar.terminal_bits.get(s)
                                                   .expect("got nonexisting terminal string"),
                    Lookahead::EOF => self.grammar.all_terminals.len()
                };

                row[column_index] = target;
            }

            format!("    [{}]", row.join(", "))
        }).collect::<Vec<_>>().join(",\n");

        rust!(self.out, "const actions: [[i32; {}]; {}] = [\n{}];", row_len, self.states.len(), rows);
        Ok(())
    }

    fn write_goto_table(&mut self) -> io::Result<()> {
        let row_len = self.grammar.nonterminals.len();

        let rows = self.states.iter().map(|s| {
            let mut row = iter::repeat("0".to_owned()).take(row_len).collect::<Vec<_>>();
            for (nt, i) in &s.gotos {
                let column_index = *self.nonterminal_bits.get(nt)
                                                         .expect("got nonexisting nonterminal");
                row[column_index] = format!("{}", i.0);
            }
            format!("    [{}]", row.join(", "))
        }).collect::<Vec<_>>().join(",\n");

        rust!(self.out, "const gotos: [[u32; {}]; {}] = [\n{}];", row_len, self.states.len(), rows);
        Ok(())   
    }

    fn write(&mut self) -> io::Result<()> {
        try!(self.write_action_table());
        try!(self.write_goto_table());
        Ok(())
    }
}
