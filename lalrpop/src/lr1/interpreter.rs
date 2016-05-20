use rust::RustWrite;

use lr1::core::*;
use lr1::lookahead::Lookahead;
use collections::Map;

use intern;

use grammar::repr::{ActionFnDefnKind, Grammar, NonterminalString, Types, TypeRepr};

use std::io;
use std::iter;
use std::collections::HashSet;

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

const TAB: usize = 4;

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

        rust!(self.out, "const PRODUCTIONS: [ReducedProduction; {}] = [", productions.len());

        for p in &productions {
            rust!(self.out,
                  "    ReducedProduction {{ nonterminal: {}, symbol_count: {}, action_fn_id: {} }},",
                  self.nonterminal_bits[&p.nonterminal], p.symbols.len(), p.action.index());
        }
        rust!(self.out, "];");

        let rows = try!(self.states.iter().enumerate().map(|(i, s)| {
            let mut row = iter::repeat("0".to_owned()).take(row_len).collect::<Vec<_>>();
            for (lookahead, action) in &s.tokens {
                let target = match *action {
                    Action::Shift(index) => format!("{}", (index.0 as i32) + 1),
                    Action::Reduce(prod) => format!("-{}", production_bits[prod]),
                };

                let column_index = match *lookahead {
                    Lookahead::Terminal(ref s) => self.grammar.terminal_bits[s],
                    Lookahead::EOF => self.grammar.all_terminals.len()
                };

                row[column_index] = target;
            }

            rust!(self.out, "const ACTION_ROW_{}: &'static [i32] = &[{}];", i, row.join(", "));
            Ok(format!("ACTION_ROW_{}", i))
        }).collect::<io::Result<Vec<_>>>());

        rust!(self.out, "const ACTIONS: [&'static [i32]; {}] = [{}];", self.states.len(), rows.join(", "));
        Ok(())
    }

    fn write_goto_table(&mut self) -> io::Result<()> {
        let row_len = self.grammar.nonterminals.len();

        let rows = try!(self.states.iter().enumerate().map(|(i, s)| {
            let mut row = iter::repeat("0".to_owned()).take(row_len).collect::<Vec<_>>();
            for (nt, i) in &s.gotos {
                let column_index = self.nonterminal_bits[nt];
                row[column_index] = format!("{}", i.0);
            }
            rust!(self.out, "const GOTO_ROW_{}: &'static [u32] = &[{}];", i, row.join(", "));
            Ok(format!("GOTO_ROW_{}", i))
        }).collect::<io::Result<Vec<_>>>());

        rust!(self.out, "const GOTOS: [&'static [u32]; {}] = [\n{}];", self.states.len(), rows.join(", "));
        Ok(())   
    }

    fn write_types(&mut self) -> io::Result<()> {
        rust!(self.out, "struct ReducedProduction {{");
        rust!(self.out, "nonterminal: u32,");
        rust!(self.out, "symbol_count: u32,");
        rust!(self.out, "action_fn_id: u32,");
        rust!(self.out, "}}");
        Ok(())   
    }

    fn write_uses(&mut self) -> io::Result<()> {
        try!(self.out.write_uses("super::", &self.grammar));
        rust!(self.out, "use super::*;");
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

        rust!(self.out, "let mut {}machine = Machine::new();", self.prefix);
        rust!(self.out,
              "{}machine.execute_partial(&mut {}tokens);",
              //self.grammar.parameters.iter().map(|p| format!("{}", p)).collect::<Vec<_>>().join(", "),
              self.prefix, self.prefix);


        rust!(self.out, "Err({}ParseError::ExtraToken {{ token: {}tokens.next().expect(\"no more tokens\").unwrap() }})", self.prefix, self.prefix);
        //rust!(self.out, "    let machine = Machine::new(&ACTIONS, &GOTOS, &PRODUCTIONS);");
        rust!(self.out, "}}");
        Ok(())
    }

    fn write_terminal_to_index_fn(&mut self) -> io::Result<()> {
        try!(self.out.write_fn_header_helper2(
            "",
            "terminal_to_index".to_owned(),
            self.grammar.type_parameters.iter().map(|p| format!("{}", p)).collect(),
            vec![format!("token: &{}", self.grammar.types.terminal_token_type())],
            "usize".to_owned(),
            self.grammar.where_clauses.iter().map(|p| format!("{}", p)).collect()));
        rust!(self.out, "{{");
        rust!(self.out, "match *token {{");

        for (&k, v) in &self.grammar.terminal_bits {
            // TODO same as in ascent.rs
            let mut pattern_names = vec![];
            let pattern = self.grammar.pattern(k).map(&mut |_| {
                let index = pattern_names.len();
                pattern_names.push(format!("{}tok{}", self.prefix, index));
                pattern_names.last().cloned().unwrap()
            });

            rust!(self.out, "{} => {},", pattern, v);
        }

        rust!(self.out, "_ => panic!(\"unuspported token\"),");

        rust!(self.out, "}}");
        rust!(self.out, "}}");

        Ok(())
    }

    fn collect_lifetimes(&self, t: &TypeRepr, lifetimes: &mut Vec<String>) {
        match t {
            &TypeRepr::Tuple(ref args) => {
                for a in args {
                    self.collect_lifetimes(a, lifetimes);
                }
            }
            &TypeRepr::Nominal(ref nt) => {
                for a in &nt.types {
                    self.collect_lifetimes(a, lifetimes);
                }
            }
            &TypeRepr::Ref { lifetime: Some(lt), referent: ref r, ..} => {
                lifetimes.push(format!("{}", lt));
                self.collect_lifetimes(r, lifetimes);
            }
            _ => {}
        }
    }

    fn write_machine(&mut self) -> io::Result<()> {
        let mut lts = Vec::new();
        self.collect_lifetimes(self.types.terminal_token_type(), &mut lts);
        let type_parameters = if lts.is_empty() {
            "".to_owned()
        } else {
            format!("<{}>", lts.join(","))
        };
        rust!(self.out, "struct StackData{} {{", type_parameters);
        rust!(self.out, "start: {},", self.types.terminal_loc_type());
        rust!(self.out, "end: {},", self.types.terminal_loc_type());
        rust!(self.out, "data: StackData_{}", type_parameters);
        rust!(self.out, "}}");
        rust!(self.out, "impl StackData{} {{", type_parameters);
        rust!(self.out, "fn from_{}(l: {}, r: {}, d: {}) -> StackData{} {{", self.types.terminal_token_type(), self.types.terminal_loc_type(), self.types.terminal_loc_type(), self.types.terminal_token_type(), type_parameters);
        rust!(self.out, "StackData{}::new(l, r, StackData_{}::__{}(d))", type_parameters, type_parameters, self.types.terminal_token_type());
        rust!(self.out, "}}");


        rust!(self.out, "fn {}_triple(self) -> ({}, {}, {}) {{", self.types.terminal_token_type(), self.types.terminal_loc_type(), self.types.terminal_token_type(), self.types.terminal_loc_type());
        rust!(self.out, "if let StackData_::__{}(d) = self.data {{", self.types.terminal_token_type());
        rust!(self.out, "(self.start, d, self.end)");
        rust!(self.out, "}} else {{");
        rust!(self.out, "panic!()"); 
        rust!(self.out, "}}");
        rust!(self.out, "}}");

        for t in self.grammar.action_fn_defns.iter().map(|a| format!("{}", a.ret_type)).collect::<HashSet<_>>().iter() {
            rust!(self.out, "fn from_{}(l: {}, r: {}, d: {}) -> StackData{} {{", t, self.types.terminal_loc_type(), self.types.terminal_loc_type(), t, type_parameters);
            rust!(self.out, "StackData{}::new(l, r, StackData_{}::__{}(d))", type_parameters, type_parameters, t);
            rust!(self.out, "}}");

            rust!(self.out, "fn {}_triple(&self) -> ({}, {}, {}) {{", t, self.types.terminal_loc_type(), t, self.types.terminal_loc_type());
            rust!(self.out, "if let StackData_::__{}(d) = self.data {{", t);
            rust!(self.out, "(self.start, d, self.end)");
            rust!(self.out, "}} else {{");
            rust!(self.out, "panic!()"); 
            rust!(self.out, "}}");
            rust!(self.out, "}}");
        }
        rust!(self.out, "fn new(start: {}, end: {}, data: StackData_{}) -> StackData{} {{", self.types.terminal_loc_type(), self.types.terminal_loc_type(), type_parameters, type_parameters);
        rust!(self.out, "StackData{} {{ start: start, end:end, data: data }}", type_parameters);
        rust!(self.out, "}}");
        rust!(self.out, "}}");

        rust!(self.out, "enum StackData_{} {{", type_parameters);
        rust!(self.out, "__{}({}),", self.types.terminal_token_type(), self.types.terminal_token_type());

        for t in self.grammar.action_fn_defns.iter().map(|a| format!("{}", a.ret_type)).collect::<HashSet<_>>().iter() {
            rust!(self.out, "__{}({}),", t, t);
        }
        rust!(self.out, "}}");
        rust!(self.out, "impl StackData_{} {{", type_parameters);
        rust!(self.out, "}}");
        rust!(self.out, "");

        rust!(self.out, "struct Machine{} {{", type_parameters);
        rust!(self.out, "state_stack: Vec<u32>,");
        rust!(self.out, "data_stack: Vec<StackData{}>", type_parameters);
        rust!(self.out, "}}");

        rust!(self.out, "impl{} Machine{} {{", type_parameters, type_parameters);

        rust!(self.out, "fn new() -> Machine{} {{", type_parameters);
        rust!(self.out, "Machine {{ state_stack: Vec::new(), data_stack: Vec::new() }}");
        rust!(self.out, "}}");

        rust!(self.out, "fn top_state(&self) -> usize {{");
        rust!(self.out, "*self.state_stack.last().expect(\"state stack is empty!\") as usize");
        rust!(self.out, "}}");

        rust!(self.out, "fn dispatch_action(&mut self, l: {}, r: {}, action_fn_id: u32) -> StackData{} {{", self.types.terminal_loc_type(), self.types.terminal_loc_type(), type_parameters);

        rust!(self.out, "match action_fn_id {{");
        for (i, d) in self.grammar.action_fn_defns.iter().enumerate() {
            rust!(self.out, "{} => {{", i);
            if let ActionFnDefnKind::User(ref def) = d.kind {
                let mut args = Vec::new();
                for (i, t) in def.arg_types.iter().enumerate() {
                    rust!(self.out, "let a{} = self.data_stack.pop().expect(\"element must be there\").{}_triple();", i, t);
                    args.push(format!("a{}", i));
                }
                args.reverse();
                rust!(self.out, "StackData{}::from_{}(l, r,__action{}(0, {}))", type_parameters, d.ret_type, i, args.join(", "));
            } else {
                rust!(self.out, "{}", i);
            }
            rust!(self.out, "}},");
        }
            rust!(self.out, "_ => panic!(\"invalid action\"),");
        rust!(self.out, "}}");
        rust!(self.out, "}}");

        rust!(self.out, "fn reduce(&mut self, l: {}, r: {}, production: &ReducedProduction) {{",  self.types.terminal_loc_type(), self.types.terminal_loc_type());

        //rust!(self.out, "let mut args = Vec::new();");
        //rust!(self.out, "for _ in 0 .. production.symbol_count {{");
        //rust!(self.out, "args.push(self.data_stack.pop().expect(\"popped data stack\"));");
        //rust!(self.out, "self.state_stack.pop();");
        //rust!(self.out, "}}");

        rust!(self.out, "let top_state = self.top_state();");
        rust!(self.out, "self.state_stack.push(GOTOS[top_state][production.nonterminal as usize]);");
        rust!(self.out, "let res = self.dispatch_action(l, r, production.nonterminal);");
        rust!(self.out, "self.data_stack.push(res);");

        rust!(self.out, "}}");


        let triple_type = self.types.triple_type();
        //
        // If we are generated the tokenizer, it generates ParseError
        // errors, otherwise they are user errors.
        let iter_error_type = if self.grammar.intern_token.is_some() {
            self.parse_error_type()
        } else {
            format!("{}", self.types.error_type())
        };

        let mut parameters = vec!["&mut self".to_owned()];
        //parameters.extend(self.grammar.parameters.iter().map(|p| format!("{}", p)));
        parameters.push(format!("{}tokens: &mut {}TOKENS", self.prefix, self.prefix));

        let mut tps: Vec<String> = Vec::new(); //self.grammar.type_parameters.iter().map(|p| format!("{}", p)).collect();
        tps.push(format!("{}TOKENS: Iterator<Item=Result<{},{}>>", self.prefix, triple_type, iter_error_type));

        try!(self.out.write_fn_header_helper2(
            "",
            "execute_partial".to_owned(),
            tps,
            parameters,
            "usize".to_owned(),
            self.grammar.where_clauses.iter().map(|p| format!("{}", p)).collect()));

        rust!(self.out, "{{");
        rust!(self.out, "self.state_stack.push(0);");
        rust!(self.out, "let mut {}token = {}tokens.next();", self.prefix, self.prefix);
        rust!(self.out, "while let Some(Ok((l, terminal, r))) = {}token {{", self.prefix);

        rust!(self.out, "let terminal_index = terminal_to_index(&terminal);");
        rust!(self.out, "let state = self.top_state();");
        rust!(self.out, "let action = ACTIONS[state][terminal_index];");

        rust!(self.out, "if action > 0 {{");
        rust!(self.out, "self.state_stack.push((action-1) as u32);");
        rust!(self.out, "self.data_stack.push(StackData::new(l, r, StackData_::__{}(terminal)));", self.types.terminal_token_type());
        rust!(self.out, "{}token = {}tokens.next();", self.prefix, self.prefix);
        rust!(self.out, "}} else if action < 0 {{");
        rust!(self.out, "self.reduce(l, r, &PRODUCTIONS[(action*-1) as usize]);");
        rust!(self.out, "{}token = Some(Ok((l, terminal, r)));", self.prefix);
        rust!(self.out, "}} else {{");
        rust!(self.out, "{}token = None;", self.prefix);
        rust!(self.out, "// error");
        rust!(self.out, "}}");

        rust!(self.out, "}}");
        /*

            // check whether we can shift this token
            match self.get_action(state, &terminal) {
                None => { panic!("Error"); }
                Some(Action::Shift(next_index)) => {
                    self.state_stack.push(next_index);
                    token = tokens.next();
                }

                Some(Action::Reduce(index)) => {
                    token = Some(terminal);
                }
            }
        }
        */
        rust!(self.out, "0");
        rust!(self.out, "}}");

        rust!(self.out, "}}");

        Ok(())
    }

    fn write(&mut self) -> io::Result<()> {
        rust!(self.out, "mod {}parse{} {{",
              self.prefix, self.start_symbol);
        try!(self.write_uses());
        rust!(self.out, "");

        try!(self.write_types());
        rust!(self.out, "");

        try!(self.write_action_table());
        rust!(self.out, "");

        try!(self.write_goto_table());
        rust!(self.out, "");

        try!(self.write_terminal_to_index_fn());
        try!(self.write_start_fn());

        try!(self.write_machine());

        rust!(self.out, "}}");
        Ok(())
    }
}
