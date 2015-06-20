//! A compiler from an LR(1) table to a [recursive ascent] parser.
//!
//! [recursive ascent]: https://en.wikipedia.org/wiki/Recursive_ascent_parser

use intern::intern;
use rust;
use util::Set;

pub fn compile<'grammar>(grammar: &'grammar Grammar,
                         action_path: &rust::Path,
                         states: &[State<'grammar>],
                         out: &mut Writer)
                         -> rust::Module
{
    let mut ascent = RecursiveAscent::new(grammar, action_path, states);
    ascent.push_items();
}

struct RecursiveAscent<'ascent,'grammar:'ascent> {
    grammar: &'grammar Grammar,
    action_path: &'ascent rust::Path,
    states: &'ascent [State<'grammar>],
    out: &mut Writer,
}

impl RecursiveAscent {
    fn new(grammar: &'grammar Grammar,
           action_path: &'ascent rust::Path,
           states: &'ascent [State<'grammar>])
    {
        let num_states = states.len();
        RecursiveAscent {
            grammar: grammar,
            states: states,
            action_path: action_path,
            items: vec![],
            option_name: intern("Option"),
            iterator_name: intern("Iterator"),
            tokens_name: intern("TOKENS"),
            item_name: intern("Item"),
            state_names: (0..num_states).map(|i| intern(&format!("state{}", i))),
            nt_type_name: intern("Nonterminal"),
            term_type_name: intern("Terminal"),
            lookahead_name: intern("lookahead"),
            tokens_name: intern("tokens"),
        }
    }

    fn push_items(&mut self) {
        self.push_terminal_use();
        self.push_return_type_defn();
    }

    fn state_name(&self, index: StateIndex) -> InternedString {
        intern(&format!("state{}", index.0))
    }

    fn terminal_type(&self) -> TypeRepr {
        self.grammar.types.terminal_type().clone()
    }

    fn lookahead_type(&self) -> TypeRepr {
        TypeRepr::Nominal {
            path: self.option_path.clone(),
            types: vec![self.grammar.types.terminal_type().clone()]
        }
    }

    fn push_terminal_use(&mut self) {
        match *self.grammar.types.terminal_type() {
            Nominal { ref path, types: _ } => { // FIXME refactor types
                self.items.push(Use(Use {
                    path: path.clone(),
                    as_name: Some(self.term_type_name)
                }));
            }
            _ => { unreachable!() }
        }
    }

    fn push_return_type_defn(&mut self) {
        // find all nonterminals that get produced by any state
        let nonterminals: Set<NonterminalString> =
            self.states.gotos.keys().cloned().collect();

        // make an enum with one variant per nonterminal; I considered
        // making different enums per state, but this would mean we
        // have to unwrap and rewrap as we pass up the stack, which
        // seems silly
        let variants: Vec<VariantDefn> =
            nonterminals.iter()
                        .map(|n| (n, self.grammar.nonterminal_type(n).clone()))
                        .map(|(n, t)| VariantDefn { name: n, arguments: vec![t] })
                        .collect();

        let enum_defn = rust::EnumDefn {
            name: self.state_names[state_index],
            variants: variants,
        };

        self.items.push(rust::ItemDefn::Enum(enum_defn));
    }

    fn push_state_fn(&mut self, state_index: usize) {
        let state = &self.states[state_index];

        // Each state fn takes as argument the longest prefix of any
        // item. Note that all items must have compatible prefixes.
        let prefix: &[Symbol] =
            state.items.iter()
                       .map(|item| &item.production.symbols[..item.index])
                       .max_by(|s| s.len())
                       .unwrap();

        debug_assert!(
            state.items.iter()
                       .all(|item| item.production.symbols.starts_with(prefix)));

        // Each state fn is parameterized by the input iterator type.
        //
        // <TOKENS:Iterator<Item=Token>>
        let tokens_defn =
            rust::TypeParameterDefn {
                name: self.tokens_name,
                bounds: vec![rust::Bound {
                    path: vec![self.iterator_name],
                    assoc_bindings: vec![rust::AssocBinding {
                        name: self.item_name,
                        ty: self.terminal_type()
                    }]
                }]
            };

        // Each state fn takes 2 base arguments: the lookahead
        // token (of type Option<Token>) and the iterator (of type
        // TOKENS).
        write!(out, "    lookahead: Option<{}>,");
        write!(out, "    tokens: TOKENS,");

        // Each of the symbols in the prefix will be an argument. So
        // collect the types of the argument.
        arg_types.extend(
            prefix.iter().map(|p| p.ty(&self.grammar.types)).cloned());

        // Give them names like sym0, sym1, etc.
        let sym_patterns =
            (0..prefix.len()).map(|i| intern(&format!("sym{}", i)));

        // The return type will be the master enum we created above.
        let ret_type =
            TypeRepr::Nominal { path: vec![self.nt_type_name],
                                types: vec![] };
    }
}
