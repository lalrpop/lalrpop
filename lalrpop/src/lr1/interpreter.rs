use rust::RustWrite;

use lr1::core::*;
use lr1::lookahead::Lookahead;
use collections::Map;

use intern;

use grammar::repr::{Grammar, NonterminalString, Types};

use std::io;
use std::iter;

pub struct Interpreter<'emitter,'grammar:'emitter> {
    /// the complete grammar
    grammar: &'grammar Grammar,

    /// some suitable prefix to separate our identifiers from the user's
    prefix: &'grammar str,

    /// types from the grammar
    types: &'grammar Types,

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
            prefix: &grammar.prefix,
            types: &grammar.types,
            user_start_symbol: user_start_symbol,
            start_symbol: start_symbol,
            states: states,
            out: out,
            nonterminal_bits: grammar.nonterminals.keys().cloned().zip(0..).collect(),
        }
    }

    fn parse_error_type(&mut self) -> String {
        format!("{}ParseError<{},{},{}>",
                self.prefix,
                self.types.terminal_loc_type(),
                self.types.terminal_token_type(),
                self.types.error_type())
    }

    fn write_action_table(&mut self) -> io::Result<()> {
        let row_len = self.grammar.all_terminals.len()+1;

        let productions = self.grammar.nonterminals.values().flat_map(|nt| nt.productions.iter()).cloned().collect::<Vec<_>>();
        let production_bits: Map<_, _> = productions.iter().cloned().zip(0..).collect();

        rust!(self.out, "const productions: [ReducedProduction; {}] = [", productions.len());

        for p in &productions {
            rust!(self.out, "    ReducedProduction {{ nonterminal: {}, symbol_count: {} }},", *self.nonterminal_bits.get(&p.nonterminal).expect("got nonexisting nonterminal"), p.symbols.len());
        }
        rust!(self.out, "];");

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
        try!(self.out.write_uses("super::", &self.grammar));
        if self.grammar.intern_token.is_none() {
            rust!(self.out, "use super::{}ToTriple;", self.prefix);


        }
        rust!(self.out ,"__lalrpop_util::{{Machine, ReducedProduction}};");
        Ok(())
    }

    fn write_start_fn(&mut self) -> io::Result<()> {
        let error_type = self.types.error_type();
        let parse_error_type = self.parse_error_type();

        let (type_parameters, parameters);

        if self.grammar.intern_token.is_some() {
            // if we are generating the tokenizer, we just need the
            // input, and that has already been added as one of the
            // user parameters
            type_parameters = vec![];
            parameters = vec![];
        } else {
            // otherwise, we need an iterator of type `TOKENS`
            let mut user_type_parameters = String::new();
            for type_parameter in &self.grammar.type_parameters {
                user_type_parameters.push_str(&format!("{}, ", type_parameter));
            }
            type_parameters = vec![
                format!("{}TOKEN: {}ToTriple<{}Error={}>",
                        self.prefix, self.prefix, user_type_parameters, error_type),
                format!("{}TOKENS: IntoIterator<Item={}TOKEN>", self.prefix, self.prefix)];
            parameters = vec![format!("{}tokens: {}TOKENS", self.prefix, self.prefix)];
        }

        try!(self.out.write_pub_fn_header(
            self.grammar,
            format!("parse_{}", self.user_start_symbol),
            type_parameters,
            parameters,
            format!("Result<{}, {}>",
                    self.types.nonterminal_type(self.start_symbol),
                    parse_error_type),
            vec![]));
        rust!(self.out, "{{");

        if self.grammar.intern_token.is_some() {
            // if we are generating the tokenizer, create a matcher as our input iterator
            rust!(self.out, "let mut {}tokens = super::{}intern_token::{}Matcher::new(input);",
                  self.prefix, self.prefix, self.prefix);
        } else {
            // otherwise, convert one from the `IntoIterator`
            // supplied, using the `ToTriple` trait which inserts
            // errors/locations etc if none are given
            rust!(self.out, "let {}tokens = {}tokens.into_iter();", self.prefix, self.prefix);
            rust!(self.out, "let mut {}tokens = {}tokens.map(|t| {}ToTriple::to_triple(t));",
                  self.prefix, self.prefix, self.prefix);
        }


        //rust!(self.out, "    let machine = Machine::new(&actions, &gotos, &productions);");
        rust!(self.out, "}}");
        Ok(())
    }

    fn write_terminal_to_index_fn(&mut self) -> io::Result<()> {

        if self.grammar.intern_token.is_some() {
            rust!(self.out, "fn terminal_to_index<'input>(token: &{}) -> usize {{", self.grammar.types.terminal_token_type());
        } else {
            rust!(self.out, "fn terminal_to_index(token: &{}) -> usize {{", self.grammar.types.terminal_token_type());
        }

        rust!(self.out, "    match *token {{");

        for (&k, v) in &self.grammar.terminal_bits {
            // TODO same as in ascent.rs
            let mut pattern_names = vec![];
            let pattern = self.grammar.pattern(k).map(&mut |_| {
                let index = pattern_names.len();
                pattern_names.push(format!("{}tok{}", self.prefix, index));
                pattern_names.last().cloned().unwrap()
            });

            let mut pattern = format!("{}", pattern);
            if pattern_names.is_empty() {
                pattern_names.push(format!("{}tok", self.prefix));
                pattern = format!("{}tok @ {}", self.prefix, pattern);
            }
            rust!(self.out, "        {} => {},", pattern, v);
        }
        rust!(self.out, "    }}");
        rust!(self.out, "}}");

        Ok(())
    }

    fn write(&mut self) -> io::Result<()> {
       rust!(self.out, "mod {}parse{} {{",
              self.prefix, self.start_symbol);
        try!(self.write_uses());
        rust!(self.out, "");

        try!(self.write_action_table());
        rust!(self.out, "");

        try!(self.write_goto_table());
        rust!(self.out, "");

        try!(self.write_terminal_to_index_fn());
        try!(self.write_start_fn());

        rust!(self.out, "}}");
        Ok(())
    }
}
