//! Utilies for running in a build script.

use grammar::parse_tree as pt;
use grammar::repr as r;
use lr1;
use normalize;
use parser;
use rust::RustWrite;
use self::filetext::FileText;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

mod filetext;

pub fn process_root() -> io::Result<()> {
    process_dir("src")
}

fn process_dir<P:AsRef<Path>>(root_dir: P) -> io::Result<()> {
    let lalrpop_files = try!(lalrpop_files(root_dir));
    for lalrpop_file in lalrpop_files {
        let rs_file = lalrpop_file.with_extension("rs");
        let grammar = try!(parse_and_normalize_grammar(lalrpop_file));
        try!(emit_recursive_ascent(&rs_file, &grammar));
    }
    Ok(())
}

fn lalrpop_files<P:AsRef<Path>>(root_dir: P) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];
    for entry in try!(fs::read_dir(root_dir)) {
        let entry = try!(entry);
        let file_type = try!(entry.file_type());

        let path = entry.path();

        if file_type.is_dir() {
            result.extend(try!(lalrpop_files(&path)));
        }

        if
            file_type.is_file() &&
            path.extension().is_some() &&
            path.extension().unwrap() == "lalrpop"
        {
            result.push(path);
        }
    }
    Ok(result)
}

fn parse_and_normalize_grammar(path: PathBuf) -> io::Result<r::Grammar> {
    let input = try!(FileText::from_path(path));

    let grammar = match parser::parse_grammar(input.text()) {
        Ok(grammar) => grammar,
        Err(error) => {
            report_error(&input,
                         pt::Span(error.offset, error.offset+1),
                         &format!("expected {}", error.expected))
        }
    };

    match normalize::normalize(grammar) {
        Ok(grammar) => Ok(grammar),
        Err(error) => {
            report_error(&input,
                         error.span,
                         &error.message)
        }
    }
}

fn report_error(file_text: &FileText, span: pt::Span, message: &str) -> ! {
    println!("{} error: {}", file_text.span_str(span), message);

    let out = io::stdout();
    let mut out = out.lock();
    file_text.highlight(span, &mut out).unwrap();

    exit(1);
}

fn emit_uses<W:Write>(grammar: &r::Grammar,
                      rust: &mut RustWrite<W>)
                      -> io::Result<()>
{
    rust.write_uses("", grammar)
}

fn emit_recursive_ascent(output_path: &Path, grammar: &r::Grammar) -> io::Result<()>
{
    let output_file = try!(fs::File::create(output_path));
    let mut rust = RustWrite::new(output_file);

    // We generate a module structure like this:
    //
    // ```
    // mod <output-file> {
    //     // For each public symbol:
    //     pub fn parse_XYZ();
    //     mod __XYZ { ... }
    //
    //     // For each bit of action code:
    //     <action-code>
    // }
    // ```
    //
    // Note that the action code goes in the outer module.  This is
    // intentional because it means that the foo.lalrpop file serves
    // as a module in the rust hierarchy, so if the action code
    // includes things like `super::` it will resolve in the natural
    // way.

    // often some of the uses are not used here
    rust!(rust, "#![allow(unused_imports)]");

    // we always thread the parameters through to the action code,
    // even if they are not used, and hence we need to disable the
    // unused variables lint, which otherwise gets very excited.
    if !grammar.parameters.is_empty() {
        rust!(rust, "#![allow(unused_variables)]");
    }

    try!(emit_uses(grammar, &mut rust));

    if grammar.start_nonterminals.is_empty() {
        println!("Error: no public symbols declared in grammar");
        exit(1);
    }

    for (&user_nt, &start_nt) in &grammar.start_nonterminals {
        // We generate these, so there should always be exactly 1
        // production. Otherwise the LR(1) algorithm doesn't know
        // where to stop!
        assert_eq!(grammar.productions_for(start_nt).len(), 1);

        let states = match lr1::build_states(&grammar, start_nt) {
            Ok(states) => states,
            Err(error) => {
                try!(lr1::report_error(&mut io::stdout(), &grammar, &error));
                exit(1)
            }
        };

        try!(lr1::ascent::compile(&grammar, user_nt, start_nt, &states, &mut rust));
    }

    try!(emit_action_code(grammar, &mut rust));
    Ok(())
}

fn emit_action_code<W:Write>(grammar: &r::Grammar,
                             rust: &mut RustWrite<W>)
                             -> io::Result<()>
{
    for (i, defn) in grammar.action_fn_defns.iter().enumerate() {
        rust!(rust, "");
        try!(rust.write_pub_fn_header(
            grammar,
            format!("{}action{}", grammar.prefix, i),
            vec![],
            defn.arg_patterns.iter()
                             .zip(defn.arg_types.iter())
                             .map(|(p, t)| format!("{}: {}", p, t))
                             .collect(),
            format!("{}", defn.ret_type),
            vec![]));
        rust!(rust, "{{");
        rust!(rust, "{}", defn.code);
        rust!(rust, "}}");
    }
    Ok(())
}
