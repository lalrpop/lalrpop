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

        let productions = self.grammar.nonterminals.values().flat_map(|nt| nt.productions.iter()).cloned().collect::<Vec<_>>();
        let production_bits: Map<_, _> = productions.iter().cloned().zip(0..).collect();
        let rows = try!(self.states.iter().enumerate().map(|(i, s)| {
            let mut row = iter::repeat("0".to_owned()).take(row_len).collect::<Vec<_>>();
            for (lookahead, action) in &s.tokens {
                let target = match *action {
                    Action::Shift(index) => format!("{}", (index.0 as i32) + 1),
                    Action::Reduce(ref prod) => format!("-{}", production_bits.get(prod).expect("got nonexisting production")),
                };

                let column_index = match *lookahead {
                    Lookahead::Terminal(ref s) =>
                        *self.grammar.terminal_bits.get(s)
                                                   .expect("got nonexisting terminal string"),
                    Lookahead::EOF => self.grammar.all_terminals.len()
                };

                row[column_index] = target;
            }

            rust!(self.out, "const action_row_{}: &'static [i32] = &[{}];", i, row.join(", "));
            Ok(format!("action_row_{}", i))
        }).collect::<io::Result<Vec<_>>>());

        rust!(self.out, "const actions: [&'static [i32]; {}] = [{}];", self.states.len(), rows.join(", "));
        Ok(())
    }

    fn write_goto_table(&mut self) -> io::Result<()> {
        let row_len = self.grammar.nonterminals.len();

        let rows = try!(self.states.iter().enumerate().map(|(i, s)| {
            let mut row = iter::repeat("0".to_owned()).take(row_len).collect::<Vec<_>>();
            for (nt, i) in &s.gotos {
                let column_index = *self.nonterminal_bits.get(nt)
                                                         .expect("got nonexisting nonterminal");
                row[column_index] = format!("{}", i.0);
            }
            rust!(self.out, "const goto_row_{}: &'static [u32] = &[{}];", i, row.join(", "));
            Ok(format!("goto_row_{}", i))
        }).collect::<io::Result<Vec<_>>>());

        rust!(self.out, "const gotos: [&'static [u32]; {}] = [\n{}];", self.states.len(), rows.join(", "));
        Ok(())   
    }

    fn write_uses(&mut self) -> io::Result<()> {
        rust!(self.out, "extern crate lalrpop_util;");
        rust!(self.out, "use lalrpop_util::Machine;");
        Ok(())
    }

    fn write_parse_fn(&mut self) -> io::Result<()> {
        rust!(self.out, "fn parse() {{");
        rust!(self.out, "    let machine = Machine::new(&actions, &gotos);");
        rust!(self.out, "}}");
        Ok(())
    }

    fn write(&mut self) -> io::Result<()> {
        try!(self.write_uses());
        rust!(self.out, "");

        try!(self.write_action_table());
        rust!(self.out, "");

        try!(self.write_goto_table());
        rust!(self.out, "");

        try!(self.write_parse_fn());
        Ok(())
    }
}
