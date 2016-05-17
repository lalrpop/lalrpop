/*!
 * Lower
 */

use intern::{self, intern, InternedString};
use normalize::NormResult;
use normalize::norm_util::{self, Symbols};
use grammar::consts::INPUT_LIFETIME;
use grammar::pattern::{Pattern, PatternKind};
use grammar::parse_tree as pt;
use grammar::parse_tree::{InternToken, NonterminalString, TerminalString};
use grammar::repr as r;
use collections::{map, Map};

pub fn lower(grammar: pt::Grammar, types: r::Types) -> NormResult<r::Grammar> {
    let state = LowerState::new(types, &grammar);
    state.lower(grammar)
}

struct LowerState {
    prefix: String,
    action_fn_defns: Vec<r::ActionFnDefn>,
    nonterminals: Map<NonterminalString, r::NonterminalData>,
    conversions: Vec<(TerminalString, Pattern<r::TypeRepr>)>,
    intern_token: Option<InternToken>,
    types: r::Types,
}

impl LowerState {
    fn new(types: r::Types, grammar: &pt::Grammar) -> LowerState {
        LowerState {
            prefix: grammar.prefix.clone(),
            action_fn_defns: vec![],
            nonterminals: map(),
            conversions: vec![],
            types: types,
            intern_token: None,
        }
    }

    fn lower(mut self, grammar: pt::Grammar) -> NormResult<r::Grammar> {
        let start_symbols = self.synthesize_start_symbols(&grammar);

        let mut uses = vec![];
        let mut token_span = None;

        for item in grammar.items {
            match item {
                pt::GrammarItem::Use(data) => {
                    uses.push(data);
                }

                pt::GrammarItem::InternToken(data) => {
                    token_span = Some(grammar.span);
                    let span = grammar.span;
                    let input_str = r::TypeRepr::Ref {
                        lifetime: Some(intern(INPUT_LIFETIME)),
                        mutable: false,
                        referent: Box::new(r::TypeRepr::Nominal(r::NominalTypeRepr {
                            path: r::Path::str(),
                            types: vec![]
                        }))
                    };
                    self.conversions.extend(
                        data.literals
                            .iter()
                            .enumerate()
                            .map(|(index, &literal)| {
                                let pattern = Pattern {
                                    span: span,
                                    kind: PatternKind::Tuple(vec![
                                        Pattern {
                                            span: span,
                                            kind: PatternKind::Usize(index),
                                        },
                                        Pattern {
                                            span: span,
                                            kind: PatternKind::Choose(input_str.clone())
                                        }
                                        ])
                                };
                                (TerminalString::Literal(literal), pattern)
                            }));
                    self.intern_token = Some(data);
                }

                pt::GrammarItem::ExternToken(data) => {
                    if let Some(enum_token) = data.enum_token {
                        token_span = Some(enum_token.type_span);
                        self.conversions.extend(
                            enum_token
                                .conversions
                                .iter()
                                .map(|conversion| (conversion.from,
                                                   conversion.to.map(&mut |t| t.type_repr()))));
                    }
                }

                pt::GrammarItem::Nonterminal(nt) => {
                    let nt_name = nt.name;
                    let productions: Vec<_> =
                        nt.alternatives
                          .into_iter()
                          .map(|alt| {
                              let nt_type = self.types.nonterminal_type(nt_name).clone();
                              let symbols = self.symbols(&alt.expr.symbols);
                              let action = self.action_kind(nt_type, &alt.expr,
                                                            &symbols, alt.action);
                              r::Production {
                                  nonterminal: nt_name,
                                  span: alt.span,
                                  symbols: symbols,
                                  action: action,
                              }
                          })
                          .collect();
                    self.nonterminals.insert(nt_name, r::NonterminalData {
                        name: nt_name,
                        annotations: nt.annotations,
                        span: nt.span,
                        productions: productions
                    });
                }
            }
        }

        let parameters =
            grammar.parameters.iter()
                              .map(|p| r::Parameter { name: p.name, ty: p.ty.type_repr() })
                              .collect();

        let algorithm =
            match grammar.algorithm {
                None => r::Algorithm::LR1,
                Some(ref a) => r::Algorithm::from_str(a.text).unwrap(),
            };

        let mut all_terminals: Vec<_> = self.conversions.iter()
                                                        .map(|c| c.0)
                                                        .collect();
        all_terminals.sort();

        let terminal_bits: Map<_,_> = all_terminals.iter()
                                                   .cloned()
                                                   .zip(0..)
                                                   .collect();

        Ok(r::Grammar {
            prefix: self.prefix,
            start_nonterminals: start_symbols,
            uses: uses,
            action_fn_defns: self.action_fn_defns,
            nonterminals: self.nonterminals,
            conversions: self.conversions.into_iter().collect(),
            types: self.types,
            token_span: token_span.unwrap(),
            type_parameters: grammar.type_parameters,
            parameters: parameters,
            where_clauses: grammar.where_clauses,
            algorithm: algorithm,
            intern_token: self.intern_token,
            all_terminals: all_terminals,
            terminal_bits: terminal_bits,
        })
    }

    fn synthesize_start_symbols(&mut self,
                                grammar: &pt::Grammar)
                                -> Map<NonterminalString, NonterminalString>
    {
        grammar.items
               .iter()
               .filter_map(|item| item.as_nonterminal())
               .filter(|nt| nt.public)
               .map(|nt| {
                   // create a synthetic symbol `__Foo` for each public symbol `Foo`
                   // with a rule like:
                   //
                   //     __Foo = Foo;
                   let fake_name =
                       pt::NonterminalString(intern(&format!("{}{}", self.prefix, nt.name)));
                   let nt_type = self.types.nonterminal_type(nt.name).clone();
                   self.types.add_type(fake_name, nt_type.clone());
                   let expr = pt::ExprSymbol {
                       symbols: vec![pt::Symbol::new(nt.span,
                                                     pt::SymbolKind::Nonterminal(fake_name))]
                   };
                   let symbols = vec![r::Symbol::Nonterminal(nt.name)];
                   let action_fn = self.action_fn(nt_type, false, &expr, &symbols, None);
                   let production = r::Production {
                       nonterminal: fake_name,
                       symbols: symbols,
                       action: action_fn,
                       span: nt.span
                   };
                   self.nonterminals.insert(
                       fake_name,
                       r::NonterminalData {
                           name: fake_name,
                           annotations: vec![],
                           span: nt.span,
                           productions: vec![production]
                       });
                   (nt.name, fake_name)
               })
               .collect()
    }

    fn action_kind(&mut self,
                   nt_type: r::TypeRepr,
                   expr: &pt::ExprSymbol,
                   symbols: &[r::Symbol],
                   action: Option<pt::ActionKind>)
                   -> r::ActionFn
    {
        match action {
            Some(pt::ActionKind::Lookahead) =>
                self.lookahead_action_fn(),
            Some(pt::ActionKind::Lookbehind) =>
                self.lookbehind_action_fn(),
            Some(pt::ActionKind::User(string)) =>
                self.action_fn(nt_type, false, &expr, &symbols, Some(string)),
            Some(pt::ActionKind::Fallible(string)) =>
                self.action_fn(nt_type, true, &expr, &symbols, Some(string)),
            None =>
                self.action_fn(nt_type, false, &expr, &symbols, None),
        }
    }

    fn lookahead_action_fn(&mut self) -> r::ActionFn {
        let action_fn_defn = r::ActionFnDefn {
            fallible: false,
            ret_type: self.types.terminal_loc_type(),
            kind: r::ActionFnDefnKind::Lookaround(r::LookaroundActionFnDefn::Lookahead),
        };

        self.add_action_fn(action_fn_defn)
    }

    fn lookbehind_action_fn(&mut self) -> r::ActionFn {
        let action_fn_defn = r::ActionFnDefn {
            fallible: false,
            ret_type: self.types.terminal_loc_type(),
            kind: r::ActionFnDefnKind::Lookaround(r::LookaroundActionFnDefn::Lookbehind),
        };

        self.add_action_fn(action_fn_defn)
    }

    fn action_fn(&mut self,
                 nt_type: r::TypeRepr,
                 fallible: bool,
                 expr: &pt::ExprSymbol,
                 symbols: &[r::Symbol],
                 action: Option<String>)
                 -> r::ActionFn
    {
        let action = match action {
            Some(s) => s,
            None => {
                // If the user declared a type `()`, or we inferred
                // it, then there is only one possible action that
                // will type-check (`()`), so supply that. Otherwise,
                // default is to include all selected items.
                if nt_type.is_unit() {
                    format!("()")
                } else {
                    format!("(<>)")
                }
            }
        };

        // Note that the action fn takes ALL of the symbols in `expr`
        // as arguments, and some of them are simply dropped based on
        // the user's selections.

        // The set of argument types is thus the type of all symbols:
        let arg_types: Vec<r::TypeRepr> =
            symbols.iter().map(|s| s.ty(&self.types)).cloned().collect();

        let action_fn_defn = match norm_util::analyze_expr(expr) {
            Symbols::Named(names) => {
                // if there are named symbols, we want to give the
                // arguments the names that the user gave them:
                let arg_patterns =
                    patterns(names.iter().map(|&(index, name, _)| (index, name)),
                             symbols.len());

                r::ActionFnDefn {
                    fallible: fallible,
                    ret_type: nt_type,
                    kind: r::ActionFnDefnKind::User(r::UserActionFnDefn {
                        arg_patterns: arg_patterns,
                        arg_types: arg_types,
                        code: action,
                    }),
                }
            }
            Symbols::Anon(indices) => {
                let names: Vec<_> =
                    (0..indices.len()).map(|i| self.fresh_name(i)).collect();
                let arg_patterns =
                    patterns(indices.iter().map(|&(index, _)| index)
                                           .zip(names.iter().cloned()),
                             symbols.len());
                let name_str = intern::read(|interner| {
                    let name_strs: Vec<_> = names.iter().map(|&n| interner.data(n)).collect();
                    name_strs.join(", ")
                });
                let action = action.replace("<>", &name_str);
                r::ActionFnDefn {
                    fallible: fallible,
                    ret_type: nt_type,
                    kind: r::ActionFnDefnKind::User(r::UserActionFnDefn {
                        arg_patterns: arg_patterns,
                        arg_types: arg_types,
                        code: action,
                    }),
                }
            }
        };

        self.add_action_fn(action_fn_defn)
    }

    fn add_action_fn(&mut self, action_fn_defn: r::ActionFnDefn) -> r::ActionFn {
        let index = r::ActionFn::new(self.action_fn_defns.len());
        self.action_fn_defns.push(action_fn_defn);
        index
    }

    fn symbols(&mut self, symbols: &[pt::Symbol]) -> Vec<r::Symbol> {
        symbols.iter().map(|sym| self.symbol(sym)).collect()
    }

    fn symbol(&mut self, symbol: &pt::Symbol) -> r::Symbol {
        match symbol.kind {
            pt::SymbolKind::Terminal(id) => r::Symbol::Terminal(id),
            pt::SymbolKind::Nonterminal(id) => r::Symbol::Nonterminal(id),
            pt::SymbolKind::Choose(ref s) | pt::SymbolKind::Name(_, ref s) => self.symbol(s),

            pt::SymbolKind::Macro(..) |
            pt::SymbolKind::Repeat(..) |
            pt::SymbolKind::Expr(..) |
            pt::SymbolKind::AmbiguousId(_) |
            pt::SymbolKind::Lookahead |
            pt::SymbolKind::Lookbehind => {
                unreachable!("symbol `{}` should have been normalized away by now", symbol)
            }
        }
    }

    fn fresh_name(&self, i: usize) -> InternedString {
        intern(&format!("{}{}", self.prefix, i))
    }
}

fn patterns<I>(mut chosen: I, num_args: usize) -> Vec<InternedString>
    where I: Iterator<Item=(usize, InternedString)>
{
    let blank = intern("_");

    let mut next_chosen = chosen.next();

    let result =
        (0..num_args)
        .map(|index| {
            match next_chosen {
                Some((chosen_index, chosen_name)) if chosen_index == index => {
                    next_chosen = chosen.next();
                    chosen_name
                }
                _ => blank,
            }
        })
        .collect();

    debug_assert!(next_chosen.is_none());

    result
}
