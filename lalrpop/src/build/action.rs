//! Code for generating action code.

use grammar::repr as r;
use rust::RustWrite;
use std::io::{self, Write};

pub fn emit_action_code<W:Write>(grammar: &r::Grammar,
                                 rust: &mut RustWrite<W>)
                                 -> io::Result<()>
{
    for (i, defn) in grammar.action_fn_defns.iter().enumerate() {
        rust!(rust, "");

        match defn.kind {
            r::ActionFnDefnKind::User(ref data) =>
                try!(emit_user_action_code(grammar, rust, i, defn, data)),
            r::ActionFnDefnKind::Lookaround(ref variant) =>
                try!(emit_lookaround_action_code(grammar, rust, i, defn, variant)),
            r::ActionFnDefnKind::Inline(ref data) =>
                try!(emit_inline_action_code(grammar, rust, i, defn, data)),
        }
    }

    Ok(())
}

fn ret_type_string(grammar: &r::Grammar,
                   defn: &r::ActionFnDefn)
                   -> String
{
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

fn emit_user_action_code<W:Write>(grammar: &r::Grammar,
                                  rust: &mut RustWrite<W>,
                                  index: usize,
                                  defn: &r::ActionFnDefn,
                                  data: &r::UserActionFnDefn)
                                  -> io::Result<()>
{
    let ret_type = ret_type_string(grammar, defn);

    let lookarounds = vec![
        format!("{}lookbehind: &Option<{}>", grammar.prefix, grammar.types.terminal_loc_type()),
        format!("{}lookahead: &Option<{}>", grammar.prefix, grammar.types.triple_type())];

    try!(rust.write_pub_fn_header(
        grammar,
        format!("{}action{}", grammar.prefix, index),
        vec![],
        data.arg_patterns.iter()
                         .zip(data.arg_types.iter())
                         .map(|(p, t)| format!("{}: {}", p, t))
                         .chain(lookarounds)
                         .collect(),
        ret_type,
        vec![]));
    rust!(rust, "{{");
    rust!(rust, "{}", data.code);
    rust!(rust, "}}");
    Ok(())
}

fn emit_lookaround_action_code<W:Write>(grammar: &r::Grammar,
                                        rust: &mut RustWrite<W>,
                                        index: usize,
                                        _defn: &r::ActionFnDefn,
                                        data: &r::LookaroundActionFnDefn)
                                        -> io::Result<()>
{
    try!(rust.write_pub_fn_header(
        grammar,
        format!("{}action{}", grammar.prefix, index),
        vec![],
        vec![format!("{}lookbehind: &Option<{}>", grammar.prefix, grammar.types.terminal_loc_type()),
             format!("{}lookahead: &Option<{}>", grammar.prefix, grammar.types.triple_type())],
        format!("{}", grammar.types.terminal_loc_type()),
        vec![]));

    rust!(rust, "{{");
    match *data {
        r::LookaroundActionFnDefn::Lookahead => {
            // take the lookahead, if any; otherwise, we are
            // at EOF, so taker the lookbehind (end of last
            // pushed token); if that is missing too, then
            // supply default.
            rust!(rust, "{}lookahead.as_ref()\
                         .map(|o| ::std::clone::Clone::clone(&o.0))\
                         .or_else(|| ::std::clone::Clone::clone(&{}lookbehind))\
                         .unwrap_or_default()",
                  grammar.prefix, grammar.prefix);
        }
        r::LookaroundActionFnDefn::Lookbehind => {
            // take lookbehind or supply default
            rust!(rust, "::std::clone::Clone::clone(&{}lookbehind).unwrap_or_default()",
                  grammar.prefix);
        }
    }
    rust!(rust, "}}");
    Ok(())
}

fn emit_inline_action_code<W:Write>(grammar: &r::Grammar,
                                    rust: &mut RustWrite<W>,
                                    index: usize,
                                    defn: &r::ActionFnDefn,
                                    data: &r::InlineActionFnDefn)
                                    -> io::Result<()>
{
    let ret_type = ret_type_string(grammar, defn);

    let arg_types: Vec<_> =
        data.symbols.iter()
                    .flat_map(|sym| match *sym {
                        r::InlinedSymbol::Original(s) => vec![s],
                        r::InlinedSymbol::Inlined(_, ref syms) => syms.clone(),
                    })
                    .map(|s| s.ty(&grammar.types))
                    .collect();

    let arguments: Vec<_> =
        arg_types.iter()
                 .enumerate()
                 .map(|(i, t)| format!("{}{}: {}", grammar.prefix, i, t))
                 .chain(vec![
                     format!("{}lookbehind: &Option<{}>",
                             grammar.prefix,
                             grammar.types.terminal_loc_type()),
                     format!("{}lookahead: &Option<{}>",
                             grammar.prefix,
                             grammar.types.triple_type())])
                 .collect();

    try!(rust.write_pub_fn_header(
        grammar,
        format!("{}action{}", grammar.prefix, index),
        vec![],
        arguments,
        ret_type,
        vec![]));
    rust!(rust, "{{");

    // create temporaries for the inlined things
    let mut arg_counter = 0;
    let mut temp_counter = 0;
    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(inlined_action, ref syms) => {
                rust!(rust, "let {}temp{} = {}action{}(",
                      grammar.prefix, temp_counter,
                      grammar.prefix, inlined_action.index());
                for parameter in &grammar.parameters {
                    rust!(rust, "{},", parameter.name);
                }
                for _ in syms {
                    rust!(rust, "{}{},", grammar.prefix, arg_counter);
                    arg_counter += 1;
                }
                rust!(rust, "{}lookbehind,", grammar.prefix);
                rust!(rust, "{}lookahead,", grammar.prefix);
                rust!(rust, ");");
                temp_counter += 1;
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


