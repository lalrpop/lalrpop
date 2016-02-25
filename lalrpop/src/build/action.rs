//! Code for generating action code.

use grammar::repr as r;
use rust::RustWrite;
use std::io::{self, Write};

pub fn emit_action_code<W: Write>(grammar: &r::Grammar, rust: &mut RustWrite<W>) -> io::Result<()> {
    for (i, defn) in grammar.action_fn_defns.iter().enumerate() {
        rust!(rust, "");

        match defn.kind {
            r::ActionFnDefnKind::User(ref data) => {
                try!(emit_user_action_code(grammar, rust, i, defn, data))
            }
            r::ActionFnDefnKind::Lookaround(ref variant) => {
                try!(emit_lookaround_action_code(grammar, rust, i, defn, variant))
            }
            r::ActionFnDefnKind::Inline(ref data) => {
                try!(emit_inline_action_code(grammar, rust, i, defn, data))
            }
        }
    }

    Ok(())
}

fn ret_type_string(grammar: &r::Grammar, defn: &r::ActionFnDefn) -> String {
    if defn.fallible {
        format!("Result<{},{}ParseError<{},{},{}>>",
                defn.ret_type,
                grammar.prefix,
                grammar.types.terminal_loc_type(),
                grammar.types.terminal_token_type(),
                grammar.types.error_type())
    } else {
        format!("{}", defn.ret_type)
    }
}

fn emit_user_action_code<W: Write>(grammar: &r::Grammar,
                                   rust: &mut RustWrite<W>,
                                   index: usize,
                                   defn: &r::ActionFnDefn,
                                   data: &r::UserActionFnDefn)
                                   -> io::Result<()> {
    let ret_type = ret_type_string(grammar, defn);

    let lookarounds = vec![format!("{}lookbehind: &{}",
                                   grammar.prefix,
                                   grammar.types.terminal_loc_type()),
                           format!("{}lookahead: &{}",
                                   grammar.prefix,
                                   grammar.types.terminal_loc_type())];

    try!(rust.write_pub_fn_header(grammar,
                                  format!("{}action{}", grammar.prefix, index),
                                  vec![],
                                  data.arg_patterns
                                      .iter()
                                      .zip(data.arg_types
                                               .iter()
                                               .cloned()
                                               .map(|t| grammar.types.spanned_type(t)))
                                      .map(|(p, t)| format!("(_, {}, _): {}", p, t))
                                      .chain(lookarounds)
                                      .collect(),
                                  ret_type,
                                  vec![]));
    rust!(rust, "{{");
    rust!(rust, "{}", data.code);
    rust!(rust, "}}");
    Ok(())
}

fn emit_lookaround_action_code<W: Write>(grammar: &r::Grammar,
                                         rust: &mut RustWrite<W>,
                                         index: usize,
                                         _defn: &r::ActionFnDefn,
                                         data: &r::LookaroundActionFnDefn)
                                         -> io::Result<()> {
    try!(rust.write_pub_fn_header(grammar,
                                  format!("{}action{}", grammar.prefix, index),
                                  vec![],
                                  vec![format!("{}lookbehind: &{}",
                                               grammar.prefix,
                                               grammar.types.terminal_loc_type()),
                                       format!("{}lookahead: &{}",
                                               grammar.prefix,
                                               grammar.types.terminal_loc_type())],
                                  format!("{}", grammar.types.terminal_loc_type()),
                                  vec![]));

    rust!(rust, "{{");
    match *data {
        r::LookaroundActionFnDefn::Lookahead => {
            // take the lookahead, if any; otherwise, we are
            // at EOF, so taker the lookbehind (end of last
            // pushed token); if that is missing too, then
            // supply default.
            rust!(rust,
                  "{}lookahead.clone()",
                  grammar.prefix);
        }
        r::LookaroundActionFnDefn::Lookbehind => {
            // take lookbehind or supply default
            rust!(rust,
                  "{}lookbehind.clone()",
                  grammar.prefix);
        }
    }
    rust!(rust, "}}");
    Ok(())
}

fn emit_inline_action_code<W: Write>(grammar: &r::Grammar,
                                     rust: &mut RustWrite<W>,
                                     index: usize,
                                     defn: &r::ActionFnDefn,
                                     data: &r::InlineActionFnDefn)
                                     -> io::Result<()> {
    let ret_type = ret_type_string(grammar, defn);

    let arg_types: Vec<_> = data.symbols
                                .iter()
                                .flat_map(|sym| {
                                    match *sym {
                                        r::InlinedSymbol::Original(s) => vec![s],
                                        r::InlinedSymbol::Inlined(_, ref syms) => syms.clone(),
                                    }
                                })
                                .map(|s| s.ty(&grammar.types))
                                .collect();

    // this is the number of symbols we expect to be passed in; it is
    // distinct from data.symbols.len(), because sometimes we have
    // inlined actions with no input symbols
    let num_flat_args = arg_types.len();

    let arguments: Vec<_> = arg_types.iter()
                                     .map(|&t| grammar.types.spanned_type(t.clone()))
                                     .enumerate()
                                     .map(|(i, t)| format!("{}{}: {}", grammar.prefix, i, t))
                                     .chain(vec![format!("{}lookbehind: &{}",
                                                         grammar.prefix,
                                                         grammar.types.terminal_loc_type()),
                                                 format!("{}lookahead: &{}",
                                                         grammar.prefix,
                                                         grammar.types.terminal_loc_type())])
                                     .collect();

    try!(rust.write_pub_fn_header(grammar,
                                  format!("{}action{}", grammar.prefix, index),
                                  vec![],
                                  arguments,
                                  ret_type,
                                  vec![]));
    rust!(rust, "{{");

    // For each inlined thing, compute the start/end locations.
    // Do this first so that none of the arguments have been moved
    // yet and we can easily access their locations.
    let mut arg_counter = 0;
    let mut temp_counter = 0;
    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(_, ref syms) => {
                if syms.len() > 0 {
                    // If we are reducing symbols, then start and end
                    // can be the start/end location of the first/last
                    // symbol respectively. Easy peezy.

                    rust!(rust, "let {}start{} = {}{}.0.clone();",
                          grammar.prefix, temp_counter,
                          grammar.prefix, arg_counter);

                    let last_arg_index = arg_counter + syms.len() - 1;
                    rust!(rust, "let {}end{} = {}{}.2.clone();",
                          grammar.prefix, temp_counter,
                          grammar.prefix, last_arg_index);
                } else {
                    // If we have no symbols, then `arg_counter`
                    // represents index of the first symbol after this
                    // inlined item (if any), and `arg_counter-1`
                    // represents index of the symbol before this
                    // item.

                    if arg_counter > 0 {
                        rust!(rust, "let {}start{} = {}{}.2.clone();",
                              grammar.prefix, temp_counter,
                              grammar.prefix, arg_counter - 1);
                    } else {
                        rust!(rust, "let {}start{} = {}lookbehind.clone();",
                              grammar.prefix, temp_counter,
                              grammar.prefix);
                    }

                    if arg_counter < num_flat_args {
                        rust!(rust, "let {}end{} = {}{}.0.clone();",
                              grammar.prefix, temp_counter,
                              grammar.prefix, arg_counter);
                    } else {
                        rust!(rust, "let {}end{} = {}lookahead.clone();",
                              grammar.prefix, temp_counter,
                              grammar.prefix);
                    }
                }

                temp_counter += 1;
                arg_counter += syms.len();
            }
        }
    }

    // Now create temporaries for the inlined things
    let mut arg_counter = 0;
    let mut temp_counter = 0;
    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(inlined_action, ref syms) => {
                // execute the inlined reduce action
                rust!(rust,
                      "let {}temp{} = {}action{}(",
                      grammar.prefix,
                      temp_counter,
                      grammar.prefix,
                      inlined_action.index());
                for parameter in &grammar.parameters {
                    rust!(rust, "{},", parameter.name);
                }
                for i in 0..syms.len() {
                    rust!(rust, "{}{},", grammar.prefix, arg_counter + i);
                }
                rust!(rust, "&{}start{},", grammar.prefix, temp_counter);
                rust!(rust, "&{}end{},", grammar.prefix, temp_counter);
                rust!(rust, ");");

                // wrap up the inlined value along with its span
                rust!(rust,
                      "let {}temp{} = ({}start{}, {}temp{}, {}end{});",
                      grammar.prefix, temp_counter,
                      grammar.prefix, temp_counter,
                      grammar.prefix, temp_counter,
                      grammar.prefix, temp_counter);

                temp_counter += 1;
                arg_counter += syms.len();
            }
        }
    }

    rust!(rust, "{}action{}(", grammar.prefix, data.action.index());
    for parameter in &grammar.parameters {
        rust!(rust, "{},", parameter.name);
    }
    let mut arg_counter = 0;
    let mut temp_counter = 0;
    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                rust!(rust, "{}{},", grammar.prefix, arg_counter);
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(_, ref syms) => {
                rust!(rust, "{}temp{},", grammar.prefix, temp_counter);
                temp_counter += 1;
                arg_counter += syms.len();
            }
        }
    }
    rust!(rust, "{}lookbehind,", grammar.prefix);
    rust!(rust, "{}lookahead,", grammar.prefix);
    rust!(rust, ")");

    rust!(rust, "}}");
    Ok(())
}
