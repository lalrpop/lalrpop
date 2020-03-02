//! Generates an iterator type `Matcher` that looks roughly like

use grammar::parse_tree::InternToken;
use grammar::repr::{Grammar, TerminalLiteral};
use lexer::re;
use rust::RustWrite;
use std::io::{self, Write};

pub fn compile<W: Write>(
    grammar: &Grammar,
    intern_token: &InternToken,
    out: &mut RustWrite<W>,
) -> io::Result<()> {
    let prefix = &grammar.prefix;

    rust!(out, "#[cfg_attr(rustfmt, rustfmt_skip)]");
    rust!(out, "mod {}intern_token {{", prefix);
    rust!(out, "#![allow(unused_imports)]");
    out.write_uses("", &grammar)?;
    rust!(
        out,
        "pub fn new_builder() -> {}lalrpop_util::lexer::MatcherBuilder {{",
        prefix
    );

    // create a vector of rust string literals with the text of each
    // regular expression
    let regex_strings: Vec<String> = {
        intern_token
            .match_entries
            .iter()
            .map(|match_entry| match match_entry.match_literal {
                TerminalLiteral::Quoted(ref s) => re::parse_literal(&s),
                TerminalLiteral::Regex(ref s) => re::parse_regex(&s).unwrap(),
            })
            .map(|regex| {
                // make sure all regex are anchored at the beginning of the input
                format!("^({})", regex)
            })
            .map(|regex_str| {
                // create a rust string with text of the regex; the Debug impl
                // will add quotes and escape
                format!("{:?}", regex_str)
            })
            .collect()
    };

    rust!(out, "let {}strs: &[&str] = &[", prefix);
    for literal in &regex_strings {
        rust!(out, "{},", literal);
    }
    rust!(out, "];");

    rust!(
        out,
        "{p}lalrpop_util::lexer::MatcherBuilder::new({p}strs).unwrap()",
        p = prefix
    );

    rust!(out, "}}"); // fn
    rust!(out, "}}"); // mod
    Ok(())
}
