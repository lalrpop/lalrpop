//! A compiler from an LR(1) table to a traditional table driven parser.

use collections::{Entry, Map, Set};
use grammar::repr::*;
use lr1::core::*;
use lr1::lookahead::Token;
use rust::RustWrite;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;
use string_cache::DefaultAtom as Atom;
use tls::Tls;
use util::Sep;

use super::base::CodeGenerator;

const DEBUG_PRINT: bool = false;

pub fn compile<'grammar, W: Write>(
    grammar: &'grammar Grammar,
    user_start_symbol: NonterminalString,
    start_symbol: NonterminalString,
    states: &[LR1State<'grammar>],
    action_module: &str,
    out: &mut RustWrite<W>,
) -> io::Result<()> {
    let mut table_driven = CodeGenerator::new_table_driven(
        grammar,
        user_start_symbol,
        start_symbol,
        states,
        action_module,
        out,
    );
    table_driven.write()
}

enum Comment<'a, T> {
    Goto(T, usize),
    Error(T),
    Reduce(T, &'a Production),
}

impl<'a, T: fmt::Display> fmt::Display for Comment<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Comment::Goto(ref token, new_state) => {
                write!(f, " // on {}, goto {}", token, new_state)
            }
            Comment::Error(ref token) => write!(f, " // on {}, error", token),
            Comment::Reduce(ref token, production) => {
                write!(f, " // on {}, reduce `{:?}`", token, production)
            }
        }
    }
}

struct TableDriven<'grammar> {
    /// type parameters for the `Nonterminal` type
    symbol_type_params: Vec<TypeParameter>,

    symbol_where_clauses: Vec<WhereClause>,

    machine: Rc<MachineParameters>,

    /// a list of each nonterminal in some specific order
    all_nonterminals: Vec<NonterminalString>,

    reduce_indices: Map<&'grammar Production, usize>,

    state_type: &'static str,

    variant_names: Map<Symbol, String>,
    variants: Map<TypeRepr, String>,
    reduce_functions: Set<usize>,
}

impl<'ascent, 'grammar, W: Write> CodeGenerator<'ascent, 'grammar, W, TableDriven<'grammar>> {
    fn new_table_driven(
        grammar: &'grammar Grammar,
        user_start_symbol: NonterminalString,
        start_symbol: NonterminalString,
        states: &'ascent [LR1State<'grammar>],
        action_module: &str,
        out: &'ascent mut RustWrite<W>,
    ) -> Self {
        let (symbol_type_params, symbol_where_clauses) =
            Self::filter_type_parameters_and_where_clauses(
                grammar,
                grammar
                    .types
                    .nonterminal_types()
                    .into_iter()
                    .chain(grammar.types.terminal_types()),
            );

        let machine = Rc::new(MachineParameters::new(grammar));

        // Assign each production a unique index to use as the values for reduce
        // actions in the ACTION and EOF_ACTION tables.
        let reduce_indices: Map<&'grammar Production, usize> = grammar
            .nonterminals
            .values()
            .flat_map(|nt| &nt.productions)
            .zip(0..)
            .collect();

        let state_type = {
            // `reduce_indices` are allowed to be +1 since the negative maximum of any integer type
            // is one larger than the positive maximum
            let max_value = ::std::cmp::max(states.len(), reduce_indices.len());
            if max_value <= ::std::i8::MAX as usize {
                "i8"
            } else if max_value <= ::std::i16::MAX as usize {
                "i16"
            } else {
                "i32"
            }
        };

        CodeGenerator::new(
            grammar,
            user_start_symbol,
            start_symbol,
            states,
            out,
            false,
            action_module,
            TableDriven {
                symbol_type_params,
                symbol_where_clauses,
                machine,
                all_nonterminals: grammar.nonterminals.keys().cloned().collect(),
                reduce_indices,
                state_type,
                variant_names: Map::new(),
                variants: Map::new(),
                reduce_functions: Set::new(),
            },
        )
    }

    fn write(&mut self) -> io::Result<()> {
        self.write_parse_mod(|this| {
            this.write_value_type_defn()?;
            this.write_parse_table()?;
            this.write_machine_definition()?;
            this.write_token_to_integer_fn()?;
            this.write_token_to_symbol_fn()?;
            this.write_simulate_reduce_fn()?;
            this.write_parser_fn()?;
            this.write_accepts_fn()?;
            this.emit_reduce_actions()?;
            this.emit_downcast_fns()?;
            this.emit_reduce_action_functions()?;
            Ok(())
        })
    }

    fn write_machine_definition(&mut self) -> io::Result<()> {
        let error_type = self.types.error_type();
        let token_type = self.types.terminal_token_type();
        let loc_type = self.types.terminal_loc_type();
        let start_type = self.types.nonterminal_type(&self.start_symbol);
        let state_type = self.custom.state_type;
        let symbol_type = self.symbol_type();
        let phantom_data_type = self.phantom_data_type();
        let phantom_data_expr = self.phantom_data_expr();
        let machine = self.custom.machine.clone();
        let machine_type_parameters = Sep(", ", &machine.type_parameters);
        let machine_where_clauses = Sep(", ", &machine.where_clauses);

        rust!(
            self.out,
            "pub struct {p}StateMachine<{mtp}>",
            p = self.prefix,
            mtp = machine_type_parameters,
        );
        rust!(self.out, "where {mwc}", mwc = machine_where_clauses);
        rust!(self.out, "{{");
        for param in &machine.fields {
            rust!(self.out, "{name}: {ty},", name = param.name, ty = param.ty,);
        }
        rust!(
            self.out,
            "{p}phantom: {phantom},",
            p = self.prefix,
            phantom = phantom_data_type,
        );
        rust!(self.out, "}}");

        rust!(
            self.out,
            "impl<{mtp}> {p}state_machine::ParserDefinition for {p}StateMachine<{mtp}>",
            p = self.prefix,
            mtp = machine_type_parameters,
        );
        rust!(self.out, "where {mwc}", mwc = machine_where_clauses);
        rust!(self.out, "{{");
        rust!(self.out, "type Location = {t};", t = loc_type);
        rust!(self.out, "type Error = {t};", t = error_type);
        rust!(self.out, "type Token = {t};", t = token_type);
        rust!(self.out, "type TokenIndex = usize;");
        rust!(
            self.out,
            "type Symbol = {symbol_type};",
            symbol_type = symbol_type,
        );
        rust!(self.out, "type Success = {t};", t = start_type);
        rust!(self.out, "type StateIndex = {t};", t = state_type);
        rust!(self.out, "type Action = {t};", t = state_type);
        rust!(self.out, "type ReduceIndex = {t};", t = state_type);
        rust!(self.out, "type NonterminalIndex = usize;");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(self.out, "fn start_location(&self) -> Self::Location {{");
        rust!(self.out, "  Default::default()");
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(self.out, "fn start_state(&self) -> Self::StateIndex {{");
        rust!(self.out, "  0");
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(
            self.out,
            "fn token_to_index(&self, token: &Self::Token) -> Option<usize> {{"
        );
        rust!(
            self.out,
            "{p}token_to_integer(token, {phantom})",
            p = self.prefix,
            phantom = phantom_data_expr,
        );
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(
            self.out,
            "fn action(&self, state: {state_type}, integer: usize) -> {state_type} {{",
            state_type = state_type
        );
        rust!(
            self.out,
            "{p}ACTION[(state as usize) * {num_term} + integer]",
            p = self.prefix,
            num_term = self.grammar.terminals.all.len(),
        );
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(
            self.out,
            "fn error_action(&self, state: {state_type}) -> {state_type} {{",
            state_type = state_type,
        );
        rust!(
            self.out,
            "{p}ACTION[(state as usize) * {num_term} + ({num_term} - 1)]",
            p = self.prefix,
            num_term = self.grammar.terminals.all.len(),
        );
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(
            self.out,
            "fn eof_action(&self, state: {state_type}) -> {state_type} {{",
            state_type = state_type,
        );
        rust!(self.out, "{p}EOF_ACTION[state as usize]", p = self.prefix,);
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(
            self.out,
            "fn goto(&self, state: {state_type}, nt: usize) -> {state_type} {{",
            state_type = state_type,
        );
        rust!(
            self.out,
            "{p}GOTO[(state as usize) * {num_non_term} + nt] - 1",
            p = self.prefix,
            num_non_term = self.grammar.nonterminals.len(),
        );
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(
            self.out,
            "fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {{"
        );
        rust!(
            self.out,
            "{p}token_to_symbol(token_index, token, {phantom})",
            p = self.prefix,
            phantom = phantom_data_expr,
        );
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(
            self.out,
            "fn expected_tokens(&self, state: {state_type}) -> Vec<String> {{",
            state_type = state_type,
        );
        rust!(
            self.out,
            "{p}expected_tokens(state as usize)",
            p = self.prefix
        );
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(self.out, "fn uses_error_recovery(&self) -> bool {{");
        rust!(self.out, "{}", self.grammar.uses_error_recovery);
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "#[inline]");
        rust!(self.out, "fn error_recovery_symbol(");
        rust!(self.out, "&self,");
        rust!(
            self.out,
            "recovery: {p}state_machine::ErrorRecovery<Self>,",
            p = self.prefix
        );
        rust!(self.out, ") -> Self::Symbol {{");
        if self.grammar.uses_error_recovery {
            let error_variant =
                self.variant_name_for_symbol(&Symbol::Terminal(TerminalString::Error));
            rust!(
                self.out,
                "{p}Symbol::{e}(recovery)",
                p = self.prefix,
                e = error_variant
            );
        } else {
            rust!(
                self.out,
                "panic!(\"error recovery not enabled for this grammar\")"
            )
        }
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(self.out, "fn reduce(");
        rust!(self.out, "&mut self,");
        rust!(self.out, "action: {state_type},", state_type = state_type);
        rust!(self.out, "start_location: Option<&Self::Location>,");
        rust!(
            self.out,
            "states: &mut Vec<{state_type}>,",
            state_type = state_type
        );
        rust!(
            self.out,
            "symbols: &mut Vec<{p}state_machine::SymbolTriple<Self>>,",
            p = self.prefix,
        );
        rust!(
            self.out,
            ") -> Option<{p}state_machine::ParseResult<Self>> {{",
            p = self.prefix,
        );
        rust!(self.out, "{p}reduce(", p = self.prefix);
        for Parameter { name, .. } in self.grammar.parameters.iter() {
            rust!(self.out, "self.{},", name);
        }
        rust!(self.out, "action,");
        rust!(self.out, "start_location,");
        rust!(self.out, "states,");
        rust!(self.out, "symbols,");
        rust!(self.out, "{},", phantom_data_expr);
        rust!(self.out, ")");
        rust!(self.out, "}}");

        rust!(self.out, "");
        rust!(
            self.out,
            "fn simulate_reduce(&self, action: {state_type}) -> {p}state_machine::SimulatedReduce<Self> {{",
            p = self.prefix,
            state_type = state_type,
        );
        rust!(
            self.out,
            "{p}simulate_reduce(action, {phantom})",
            p = self.prefix,
            phantom = phantom_data_expr,
        );
        rust!(self.out, "}}");

        rust!(self.out, "}}");

        Ok(())
    }

    fn write_value_type_defn(&mut self) -> io::Result<()> {
        // sometimes some of the variants are not used, particularly
        // if we are generating multiple parsers from the same file:
        rust!(self.out, "#[allow(dead_code)]");
        rust!(
            self.out,
            "pub enum {}Symbol<{}>",
            self.prefix,
            Sep(", ", &self.custom.symbol_type_params),
        );

        if !self.custom.symbol_where_clauses.is_empty() {
            rust!(
                self.out,
                " where {}",
                Sep(", ", &self.custom.symbol_where_clauses),
            );
        }

        rust!(self.out, " {{");

        // make one variant per terminal
        for term in &self.grammar.terminals.all {
            let ty = self.types.terminal_type(term).clone();
            let len = self.custom.variants.len();
            let name = match self.custom.variants.entry(ty.clone()) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    let name = format!("Variant{}", len);

                    rust!(self.out, "{}({}),", name, ty);
                    entry.insert(name)
                }
            };

            self.custom
                .variant_names
                .insert(Symbol::Terminal(term.clone()), name.clone());
        }

        // make one variant per nonterminal
        for nt in self.grammar.nonterminals.keys() {
            let ty = self.types.nonterminal_type(nt).clone();
            let len = self.custom.variants.len();
            let name = match self.custom.variants.entry(ty.clone()) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    let name = format!("Variant{}", len);

                    rust!(self.out, "{}({}),", name, ty);
                    entry.insert(name)
                }
            };

            self.custom
                .variant_names
                .insert(Symbol::Nonterminal(nt.clone()), name.clone());
        }

        rust!(self.out, "}}");
        Ok(())
    }

    fn write_parse_table(&mut self) -> io::Result<()> {
        // The table is a two-dimensional matrix indexed first by state
        // and then by the terminal index. The value is described above.
        rust!(
            self.out,
            "const {}ACTION: &'static [{}] = &[",
            self.prefix,
            self.custom.state_type
        );

        for (index, state) in self.states.iter().enumerate() {
            rust!(self.out, "// State {}", index);

            if Tls::session().emit_comments {
                for item in state.items.vec.iter() {
                    rust!(self.out, "//     {:?}", item);
                }
            }

            // Write an action for each terminal (either shift, reduce, or error).
            let custom = &self.custom;
            let iterator = self.grammar.terminals.all.iter().map(|terminal| {
                if let Some(new_state) = state.shifts.get(&terminal) {
                    (
                        new_state.0 as i32 + 1,
                        Comment::Goto(Token::Terminal(terminal.clone()), new_state.0),
                    )
                } else {
                    Self::write_reduction(custom, state, &Token::Terminal(terminal.clone()))
                }
            });
            self.out.write_table_row(iterator)?
        }

        rust!(self.out, "];");

        // Actions on EOF. Indexed just by state.
        rust!(
            self.out,
            "const {}EOF_ACTION: &'static [{}] = &[",
            self.prefix,
            self.custom.state_type
        );
        for (index, state) in self.states.iter().enumerate() {
            rust!(self.out, "// State {}", index);
            let reduction = Self::write_reduction(&self.custom, state, &Token::EOF);
            self.out.write_table_row(Some(reduction))?;
        }
        rust!(self.out, "];");

        // The goto table is indexed by state and *nonterminal*.
        rust!(
            self.out,
            "const {}GOTO: &'static [{}] = &[",
            self.prefix,
            self.custom.state_type
        );
        for (index, state) in self.states.iter().enumerate() {
            rust!(self.out, "// State {}", index);
            let iterator = self.grammar.nonterminals.keys().map(|nonterminal| {
                if let Some(&new_state) = state.gotos.get(&nonterminal) {
                    (
                        new_state.0 as i32 + 1,
                        Comment::Goto(nonterminal, new_state.0),
                    )
                } else {
                    (0, Comment::Error(nonterminal))
                }
            });
            self.out.write_table_row(iterator)?;
        }
        rust!(self.out, "];");

        self.emit_expected_tokens_fn()?;

        Ok(())
    }

    fn write_reduction<'s>(
        custom: &TableDriven<'grammar>,
        state: &'s LR1State,
        token: &Token,
    ) -> (i32, Comment<'s, Token>) {
        let reduction = state
            .reductions
            .iter()
            .filter(|&&(ref t, _)| t.contains(token))
            .map(|&(_, p)| p)
            .next();
        if let Some(production) = reduction {
            let action = custom.reduce_indices[production];
            (
                -(action as i32 + 1),
                Comment::Reduce(token.clone(), production),
            )
        } else {
            // Otherwise, this is an error. Store 0.
            (0, Comment::Error(token.clone()))
        }
    }

    fn write_parser_fn(&mut self) -> io::Result<()> {
        let phantom_data_expr = self.phantom_data_expr();

        self.start_parser_fn()?;

        self.define_tokens()?;

        rust!(
            self.out,
            "let {p}r = {p}state_machine::Parser::drive(",
            p = self.prefix,
        );
        rust!(self.out, "{p}StateMachine {{", p = self.prefix);
        for Parameter { name, .. } in &self.grammar.parameters {
            rust!(self.out, "{},", name);
        }
        rust!(
            self.out,
            "{p}phantom: {phantom},",
            p = self.prefix,
            phantom = phantom_data_expr,
        );
        rust!(self.out, "}},");
        rust!(self.out, "{p}tokens,", p = self.prefix);
        rust!(self.out, ");");
        rust!(self.out, "{p}r", p = self.prefix);

        self.end_parser_fn()
    }

    fn write_token_to_integer_fn(&mut self) -> io::Result<()> {
        let token_type = self.types.terminal_token_type();

        let parameters = vec![
            format!(
                "{p}token: &{token_type}",
                p = self.prefix,
                token_type = token_type,
            ),
            format!("_: {}", self.phantom_data_type()),
        ];

        self.out
            .fn_header(
                &Visibility::Priv,
                format!("{p}token_to_integer", p = self.prefix),
            )
            .with_type_parameters(&self.grammar.type_parameters)
            .with_where_clauses(&self.grammar.where_clauses)
            .with_parameters(parameters)
            .with_return_type("Option<usize>")
            .emit()?;
        rust!(self.out, "{{");

        rust!(self.out, "match *{p}token {{", p = self.prefix);

        for (terminal, index) in self.grammar.terminals.all.iter().zip(0..) {
            if *terminal == TerminalString::Error {
                continue;
            }
            let pattern = self.grammar.pattern(terminal).map(&mut |_| "_");
            rust!(
                self.out,
                "{pattern} if true => Some({index}),",
                pattern = pattern,
                index = index
            );
        }

        rust!(self.out, "_ => None,");

        rust!(self.out, "}}");
        rust!(self.out, "}}");

        Ok(())
    }

    fn write_token_to_symbol_fn(&mut self) -> io::Result<()> {
        let symbol_type = self.symbol_type();
        let token_type = self.types.terminal_token_type();

        let parameters = vec![
            format!("{p}token_index: usize", p = self.prefix,),
            format!(
                "{p}token: {token_type}",
                p = self.prefix,
                token_type = token_type,
            ),
            format!("_: {}", self.phantom_data_type()),
        ];

        self.out
            .fn_header(
                &Visibility::Priv,
                format!("{p}token_to_symbol", p = self.prefix),
            )
            .with_type_parameters(&self.grammar.type_parameters)
            .with_where_clauses(&self.grammar.where_clauses)
            .with_parameters(parameters)
            .with_return_type(symbol_type)
            .emit()?;
        rust!(self.out, "{{");

        rust!(self.out, "match {p}token_index {{", p = self.prefix,);
        for (terminal, index) in self.grammar.terminals.all.iter().zip(0..) {
            if *terminal == TerminalString::Error {
                continue;
            }
            rust!(self.out, "{} => match {}token {{", index, self.prefix);

            let mut pattern_names = vec![];
            let pattern = self.grammar.pattern(terminal).map(&mut |_| {
                let index = pattern_names.len();
                pattern_names.push(format!("{}tok{}", self.prefix, index));
                pattern_names.last().cloned().unwrap()
            });

            let mut pattern = format!("{}", pattern);
            if pattern_names.is_empty() {
                pattern_names.push(format!("{}tok", self.prefix));
                pattern = format!("{}tok @ {}", self.prefix, pattern);
            }

            let variant_name = self.variant_name_for_symbol(&Symbol::Terminal(terminal.clone()));
            rust!(
                self.out,
                "{pattern} => {p}Symbol::{variant_name}(({pattern_names})),",
                pattern = pattern,
                p = self.prefix,
                variant_name = variant_name,
                pattern_names = pattern_names.join(", "),
            );
            rust!(self.out, "_ => unreachable!(),");
            rust!(self.out, "}},");
        }

        rust!(self.out, "_ => unreachable!(),");

        rust!(self.out, "}}");
        rust!(self.out, "}}");
        Ok(())
    }

    fn emit_reduce_actions(&mut self) -> io::Result<()> {
        let success_type = self.types.nonterminal_type(&self.start_symbol);
        let parse_error_type = self.types.parse_error_type();
        let loc_type = self.types.terminal_loc_type();
        let spanned_symbol_type = self.spanned_symbol_type();

        let parameters = vec![
            format!("{}action: {}", self.prefix, self.custom.state_type),
            format!("{}lookahead_start: Option<&{}>", self.prefix, loc_type),
            format!(
                "{}states: &mut ::std::vec::Vec<{}>",
                self.prefix, self.custom.state_type
            ),
            format!(
                "{}symbols: &mut ::std::vec::Vec<{}>",
                self.prefix, spanned_symbol_type
            ),
            format!("_: {}", self.phantom_data_type()),
        ];

        self.out
            .fn_header(
                &Visibility::Pub(Some(Path::from_id(Atom::from("crate")))),
                format!("{}reduce", self.prefix),
            )
            .with_grammar(self.grammar)
            .with_parameters(parameters)
            .with_return_type(format!(
                "Option<Result<{},{}>>",
                success_type, parse_error_type
            ))
            .emit()?;
        rust!(self.out, "{{");

        rust!(
            self.out,
            "let ({p}pop_states, {p}nonterminal) = match {}action {{",
            p = self.prefix
        );
        for (production, index) in self
            .grammar
            .nonterminals
            .values()
            .flat_map(|nt| &nt.productions)
            .zip(0..)
        {
            rust!(self.out, "{} => {{", index);
            // In debug builds LLVM is not very good at reusing stack space which makes this
            // reduce function take up O(number of states) space. By wrapping each reduce action in
            // an immediately called function each reduction takes place in their own function
            // context which ends up reducing the stack space used.

            // Fallible actions and the start symbol may do early returns so we avoid wrapping
            // those
            let is_fallible = self.grammar.action_is_fallible(production.action);
            let reduce_stack_space = !is_fallible && production.nonterminal != self.start_symbol;

            if reduce_stack_space {
                self.custom.reduce_functions.insert(index);
                let phantom_data_expr = self.phantom_data_expr();
                rust!(
                    self.out,
                    "{p}reduce{}({}{p}action, {p}lookahead_start, {p}states, {p}symbols, {})",
                    index,
                    self.grammar.user_parameter_refs(),
                    phantom_data_expr,
                    p = self.prefix
                );
            } else {
                self.emit_reduce_action(production)?;
            }

            rust!(self.out, "}}");
        }
        rust!(
            self.out,
            "_ => panic!(\"invalid action code {{}}\", {}action)",
            self.prefix
        );
        rust!(self.out, "}};");

        // pop the consumed states from the stack
        rust!(
            self.out,
            "let {p}states_len = {p}states.len();",
            p = self.prefix
        );
        rust!(
            self.out,
            "{p}states.truncate({p}states_len - {p}pop_states);",
            p = self.prefix
        );

        rust!(
            self.out,
            "let {}state = *{}states.last().unwrap() as usize;",
            self.prefix,
            self.prefix
        );
        rust!(
            self.out,
            "let {}next_state = {}GOTO[{}state * {} + {}nonterminal] - 1;",
            self.prefix,
            self.prefix,
            self.prefix,
            self.grammar.nonterminals.len(),
            self.prefix
        );
        if DEBUG_PRINT {
            rust!(
                self.out,
                "println!(\"goto state {{}} from {{}} due to nonterminal {{}}\", {}next_state, \
                 {}state, {}nonterminal);",
                self.prefix,
                self.prefix,
                self.prefix
            );
        }
        rust!(
            self.out,
            "{}states.push({}next_state);",
            self.prefix,
            self.prefix
        );
        rust!(self.out, "None");
        rust!(self.out, "}}");
        Ok(())
    }

    fn emit_reduce_action_functions(&mut self) -> io::Result<()> {
        for (production, index) in self
            .grammar
            .nonterminals
            .values()
            .flat_map(|nt| &nt.productions)
            .zip(0..)
        {
            if self.custom.reduce_functions.contains(&index) {
                self.emit_reduce_alternative_fn_header(index)?;
                self.emit_reduce_action(production)?;
                rust!(self.out, "}}");
            }
        }
        Ok(())
    }

    fn emit_reduce_alternative_fn_header(&mut self, index: usize) -> io::Result<()> {
        let loc_type = self.types.terminal_loc_type();
        let spanned_symbol_type = self.spanned_symbol_type();

        let parameters = vec![
            format!("{}action: {}", self.prefix, self.custom.state_type),
            format!("{}lookahead_start: Option<&{}>", self.prefix, loc_type),
            format!(
                "{}states: &mut ::std::vec::Vec<{}>",
                self.prefix, self.custom.state_type
            ),
            format!(
                "{}symbols: &mut ::std::vec::Vec<{}>",
                self.prefix, spanned_symbol_type
            ),
            format!("_: {}", self.phantom_data_type()),
        ];

        self.out
            .fn_header(
                &Visibility::Pub(Some(Path::from_id(Atom::from("crate")))),
                format!("{}reduce{}", self.prefix, index),
            )
            .with_grammar(self.grammar)
            .with_parameters(parameters)
            .with_return_type("(usize, usize)")
            .emit()?;
        rust!(self.out, "{{");
        Ok(())
    }

    fn emit_reduce_action(&mut self, production: &Production) -> io::Result<()> {
        rust!(self.out, "// {:?}", production);

        // Pop each of the symbols and their associated states.
        for (index, symbol) in production.symbols.iter().enumerate().rev() {
            let name = self.variant_name_for_symbol(symbol);
            rust!(
                self.out,
                "let {}sym{} = {}pop_{}({}symbols);",
                self.prefix,
                index,
                self.prefix,
                name,
                self.prefix
            );
        }
        let transfer_syms: Vec<_> = (0..production.symbols.len())
            .map(|i| format!("{}sym{}", self.prefix, i))
            .collect();

        // Execute the action fn
        // identify the "start" location for this production; this
        // is typically the start of the first symbol we are
        // reducing; but in the case of an empty production, it
        // will be the last symbol pushed, or at worst `default`.
        if let Some(first_sym) = transfer_syms.first() {
            rust!(
                self.out,
                "let {}start = {}.0.clone();",
                self.prefix,
                first_sym
            );
        } else {
            // we pop no symbols, so grab from the top of the stack
            // (unless we are in the start state, in which case the
            // stack will be empty)
            rust!(
                self.out,
                "let {}start = {}symbols.last().map(|s| s.2.clone()).unwrap_or_default();",
                self.prefix,
                self.prefix
            );
        }

        // identify the "end" location for this production;
        // this is typically the end of the last symbol we are reducing,
        // but in the case of an empty production it will come from the
        // lookahead
        if let Some(last_sym) = transfer_syms.last() {
            rust!(self.out, "let {}end = {}.2.clone();", self.prefix, last_sym);
        } else {
            rust!(
                self.out,
                "let {}end = {}lookahead_start.cloned().unwrap_or_else(|| \
                 {}start.clone());",
                self.prefix,
                self.prefix,
                self.prefix
            );
        }

        let transfered_syms = transfer_syms.len();

        let mut args = transfer_syms;
        if transfered_syms == 0 {
            args.push(format!("&{}start", self.prefix));
            args.push(format!("&{}end", self.prefix));
        }

        // invoke the action code
        let is_fallible = self.grammar.action_is_fallible(production.action);
        if is_fallible {
            rust!(
                self.out,
                "let {}nt = match {}::{}action{}::<{}>({}{}) {{",
                self.prefix,
                self.action_module,
                self.prefix,
                production.action.index(),
                Sep(", ", &self.grammar.non_lifetime_type_parameters()),
                self.grammar.user_parameter_refs(),
                Sep(", ", &args)
            );
            rust!(self.out, "Ok(v) => v,");
            rust!(self.out, "Err(e) => return Some(Err(e)),");
            rust!(self.out, "}};");
        } else {
            rust!(
                self.out,
                "let {}nt = {}::{}action{}::<{}>({}{});",
                self.prefix,
                self.action_module,
                self.prefix,
                production.action.index(),
                Sep(", ", &self.grammar.non_lifetime_type_parameters()),
                self.grammar.user_parameter_refs(),
                Sep(", ", &args)
            );
        }

        // if this is the final state, return it
        if production.nonterminal == self.start_symbol {
            rust!(self.out, "return Some(Ok({}nt));", self.prefix);
            return Ok(());
        }

        // push the produced value on the stack
        let name =
            self.variant_name_for_symbol(&Symbol::Nonterminal(production.nonterminal.clone()));
        rust!(
            self.out,
            "{}symbols.push(({}start, {}Symbol::{}({}nt), {}end));",
            self.prefix,
            self.prefix,
            self.prefix,
            name,
            self.prefix,
            self.prefix
        );

        // produce the index that we will use to extract the next state
        // from GOTO array
        let index = self
            .custom
            .all_nonterminals
            .iter()
            .position(|x| *x == production.nonterminal)
            .unwrap();
        rust!(
            self.out,
            "({len}, {index})",
            index = index,
            len = production.symbols.len()
        );

        Ok(())
    }

    fn variant_name_for_symbol(&self, s: &Symbol) -> String {
        self.custom.variant_names[s].clone()
    }

    fn emit_downcast_fns(&mut self) -> io::Result<()> {
        for (ty, name) in self.custom.variants.clone() {
            self.emit_downcast_fn(&name, ty)?;
        }

        Ok(())
    }

    fn emit_downcast_fn(&mut self, variant_name: &str, variant_ty: TypeRepr) -> io::Result<()> {
        let spanned_symbol_type = self.spanned_symbol_type();

        rust!(self.out, "fn {}pop_{}<", self.prefix, variant_name);
        for type_parameter in &self.custom.symbol_type_params {
            rust!(self.out, "  {},", type_parameter);
        }
        rust!(self.out, ">(");
        rust!(
            self.out,
            "{}symbols: &mut ::std::vec::Vec<{}>",
            self.prefix,
            spanned_symbol_type
        );
        rust!(self.out, ") -> {}", self.types.spanned_type(variant_ty));

        if !self.custom.symbol_where_clauses.is_empty() {
            rust!(
                self.out,
                " where {}",
                Sep(", ", &self.custom.symbol_where_clauses)
            );
        }

        rust!(self.out, " {{");

        if DEBUG_PRINT {
            rust!(self.out, "println!(\"pop_{}\");", variant_name);
        }
        rust!(self.out, "match {}symbols.pop().unwrap() {{", self.prefix);
        rust!(
            self.out,
            "({}l, {}Symbol::{}({}v), {}r) => ({}l, {}v, {}r),",
            self.prefix,
            self.prefix,
            variant_name,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix
        );
        rust!(self.out, "_ => panic!(\"symbol type mismatch\")");
        rust!(self.out, "}}");

        rust!(self.out, "}}");

        Ok(())
    }

    fn write_simulate_reduce_fn(&mut self) -> io::Result<()> {
        let state_type = self.custom.state_type;

        let parameters = vec![
            format!(
                "{p}reduce_index: {state_type}",
                p = self.prefix,
                state_type = state_type,
            ),
            format!("_: {}", self.phantom_data_type()),
        ];

        self.out
            .fn_header(
                &Visibility::Priv,
                format!("{p}simulate_reduce", p = self.prefix),
            )
            .with_type_parameters(&self.custom.machine.type_parameters)
            .with_where_clauses(&self.custom.machine.where_clauses)
            .with_parameters(parameters)
            .with_return_type(format!(
                "{p}state_machine::SimulatedReduce<{p}StateMachine<{mtp}>>",
                p = self.prefix,
                mtp = Sep(", ", &self.custom.machine.type_parameters),
            ))
            .emit()?;
        rust!(self.out, "{{");

        rust!(self.out, "match {p}reduce_index {{", p = self.prefix,);
        for (production, index) in self
            .grammar
            .nonterminals
            .values()
            .flat_map(|nt| &nt.productions)
            .zip(0..)
        {
            if Tls::session().emit_comments {
                rust!(self.out, "// simulate {:?}", production);
            }

            // if we just reduced the start symbol, that is also an accept criteria
            if production.nonterminal == self.start_symbol {
                rust!(
                    self.out,
                    "{index} => {p}state_machine::SimulatedReduce::Accept,",
                    index = index,
                    p = self.prefix,
                );
            } else {
                let num_symbols = production.symbols.len();
                let nt = self
                    .custom
                    .all_nonterminals
                    .iter()
                    .position(|x| *x == production.nonterminal)
                    .unwrap();
                rust!(self.out, "{} => {{", index);
                if DEBUG_PRINT {
                    rust!(
                        self.out,
                        "println!(r##\"accepts: simulating {:?}\"##);",
                        production
                    );
                }
                rust!(
                    self.out,
                    "{p}state_machine::SimulatedReduce::Reduce {{",
                    p = self.prefix,
                );
                rust!(
                    self.out,
                    "states_to_pop: {num_symbols},",
                    num_symbols = num_symbols,
                );
                rust!(self.out, "nonterminal_produced: {nt},", nt = nt);
                rust!(self.out, "}}");
                rust!(self.out, "}}");
            }
        }
        rust!(
            self.out,
            "_ => panic!(\"invalid reduction index {{}}\", {}reduce_index)",
            self.prefix,
        );
        rust!(self.out, "}}"); // end match

        rust!(self.out, "}}");
        Ok(())
    }

    /// The `accepts` function
    ///
    /// ```ignore
    /// fn __accepts() {
    ///     error_state: i32,
    ///     states: &Vec<i32>,
    ///     opt_integer: Option<usize>,
    /// ) -> bool {
    ///     ...
    /// }
    /// ```
    ///
    /// has the job of figuring out whether the given error state would
    /// "accept" the given lookahead. We basically trace through the LR
    /// automaton looking for one of two outcomes:
    ///
    /// - the lookahead is eventually shifted
    /// - we reduce to the end state successfully (in the case of EOF).
    ///
    /// If we used the pure LR(1) algorithm, we wouldn't need this
    /// function, because we would be guaranteed to error immediately
    /// (and not after some number of reductions). But with an LALR
    /// (or Lane Table) generated automaton, it is possible to reduce
    /// some number of times before encountering an error. Failing to
    /// take this into account can lead error recovery into an
    /// infinite loop (see the `error_recovery_lalr_loop` test) or
    /// produce crappy results (see `error_recovery_lock_in`).
    fn write_accepts_fn(&mut self) -> io::Result<()> {
        if !self.grammar.uses_error_recovery {
            return Ok(());
        }

        let phantom_data_expr = self.phantom_data_expr();
        let actions_per_state = self.grammar.terminals.all.len();
        let parameters = vec![
            format!(
                "{p}error_state: {typ}",
                p = self.prefix,
                typ = self.custom.state_type
            ),
            format!(
                "{p}states: & [{typ}]",
                p = self.prefix,
                typ = self.custom.state_type
            ),
            format!("{p}opt_integer: Option<usize>", p = self.prefix),
            format!("_: {}", self.phantom_data_type()),
        ];

        self.out
            .fn_header(&Visibility::Priv, format!("{}accepts", self.prefix))
            .with_grammar(self.grammar)
            .with_parameters(parameters)
            .with_return_type("bool")
            .emit()?;
        rust!(self.out, "{{");

        if DEBUG_PRINT {
            rust!(
                self.out,
                "println!(\"Testing whether state {{}} accepts token {{:?}}\", \
                 {p}error_state, {p}opt_integer);",
                p = self.prefix
            );
        }

        // Create our own copy of the state stack to play with.
        rust!(
            self.out,
            "let mut {p}states = {p}states.to_vec();",
            p = self.prefix
        );
        rust!(self.out, "{p}states.push({p}error_state);", p = self.prefix);

        rust!(self.out, "loop {{",);

        rust!(
            self.out,
            "let mut {}states_len = {}states.len();",
            self.prefix,
            self.prefix
        );

        rust!(
            self.out,
            "let {p}top = {p}states[{p}states_len - 1] as usize;",
            p = self.prefix
        );

        if DEBUG_PRINT {
            rust!(
                self.out,
                "println!(\"accepts: top-state={{}} num-states={{}}\", {p}top, {p}states_len);",
                p = self.prefix
            );
        }

        rust!(
            self.out,
            "let {p}action = match {p}opt_integer {{",
            p = self.prefix
        );
        rust!(
            self.out,
            "None => {p}EOF_ACTION[{p}top as usize],",
            p = self.prefix
        );
        rust!(
            self.out,
            "Some({p}integer) => {p}ACTION[{p}top * {actions_per_state} + {p}integer],",
            p = self.prefix,
            actions_per_state = actions_per_state,
        );
        rust!(self.out, "}};"); // end `match`

        // If we encounter an error action, we do **not** accept.
        rust!(
            self.out,
            "if {p}action == 0 {{ return false; }}",
            p = self.prefix
        );

        // If we encounter a shift action, we DO accept.
        rust!(
            self.out,
            "if {p}action > 0 {{ return true; }}",
            p = self.prefix
        );

        // If we encounter a reduce action, we need to simulate its
        // effect on the state stack.
        rust!(
            self.out,
            "let ({p}to_pop, {p}nt) = match {p}simulate_reduce(-({p}action + 1), {pde}) {{",
            p = self.prefix,
            pde = phantom_data_expr,
        );
        rust!(
            self.out,
            "{p}state_machine::SimulatedReduce::Reduce {{",
            p = self.prefix,
        );
        rust!(self.out, "states_to_pop, nonterminal_produced",);
        rust!(self.out, "}} => (states_to_pop, nonterminal_produced),",);
        rust!(
            self.out,
            "{p}state_machine::SimulatedReduce::Accept => return true,",
            p = self.prefix,
        );
        rust!(self.out, "}};");

        rust!(self.out, "{p}states_len -= {p}to_pop;", p = self.prefix);
        rust!(
            self.out,
            "{p}states.truncate({p}states_len);",
            p = self.prefix
        );
        rust!(
            self.out,
            "let {p}top = {p}states[{p}states_len - 1] as usize;",
            p = self.prefix
        );

        if DEBUG_PRINT {
            rust!(
                self.out,
                "println!(\"accepts: popped {{}} symbols, new top is {{}}, nt is {{}}\", \
                 {p}to_pop, \
                 {p}top, \
                 {p}nt, \
                 );",
                p = self.prefix
            );
        }

        rust!(
            self.out,
            "let {p}next_state = {p}GOTO[{p}top * {num_non_terminals} + {p}nt] - 1;",
            p = self.prefix,
            num_non_terminals = self.grammar.nonterminals.len(),
        );

        rust!(self.out, "{p}states.push({p}next_state);", p = self.prefix);

        rust!(self.out, "}}"); // end loop
        rust!(self.out, "}}"); // end fn

        Ok(())
    }

    fn symbol_type(&self) -> String {
        format!(
            "{p}Symbol<{stp}>",
            p = self.prefix,
            stp = Sep(", ", &self.custom.symbol_type_params),
        )
    }

    fn spanned_symbol_type(&self) -> String {
        let loc_type = self.types.terminal_loc_type();
        format!("({},{},{})", loc_type, self.symbol_type(), loc_type)
    }

    fn emit_expected_tokens_fn(&mut self) -> io::Result<()> {
        rust!(
            self.out,
            "fn {}expected_tokens({}state: usize) -> Vec<::std::string::String> {{",
            self.prefix,
            self.prefix
        );

        rust!(
            self.out,
            "const {}TERMINAL: &'static [&'static str] = &[",
            self.prefix
        );
        let all_terminals = if self.grammar.uses_error_recovery {
            // Subtract one to exlude the error terminal
            &self.grammar.terminals.all[..self.grammar.terminals.all.len() - 1]
        } else {
            &self.grammar.terminals.all
        };
        for terminal in all_terminals {
            // Three # should hopefully be enough to prevent any
            // reasonable terminal from escaping the literal
            rust!(self.out, "r###\"{}\"###,", terminal);
        }
        rust!(self.out, "];");

        // Grab any terminals in the current state which would have resulted in a successful parse
        rust!(
            self.out,
            "{}ACTION[({}state * {})..].iter().zip({}TERMINAL).filter_map(|(&state, terminal)| {{",
            self.prefix,
            self.prefix,
            self.grammar.terminals.all.len(),
            self.prefix
        );
        rust!(self.out, "if state == 0 {{");
        rust!(self.out, "None");
        rust!(self.out, "}} else {{");
        rust!(self.out, "Some(terminal.to_string())");
        rust!(self.out, "}}");
        rust!(self.out, "}}).collect()");
        rust!(self.out, "}}");
        Ok(())
    }
}

struct MachineParameters {
    type_parameters: Vec<TypeParameter>,
    fields: Vec<Parameter>,
    where_clauses: Vec<WhereClause>,
}

impl MachineParameters {
    fn new(grammar: &Grammar) -> Self {
        let mut type_parameters = grammar.type_parameters.clone();
        let mut where_clauses = grammar.where_clauses.clone();

        let fields: Vec<_> = grammar
            .parameters
            .iter()
            .map(|Parameter { name, ty }| {
                let named_ty = ty.name_anonymous_lifetimes_and_compute_implied_outlives(
                    &grammar.prefix,
                    &mut type_parameters,
                    &mut where_clauses,
                );
                Parameter {
                    name: name.clone(),
                    ty: named_ty,
                }
            })
            .collect();

        // Put lifetimes first (this is stable, mind, so order remains
        // largely unperturbed):
        type_parameters.sort_by_key(|tp| match tp {
            TypeParameter::Lifetime(_) => 0,
            TypeParameter::Id(_) => 1,
        });

        Self {
            type_parameters,
            fields,
            where_clauses,
        }
    }
}
