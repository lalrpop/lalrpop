#![cfg(not(test))]

use intern::{intern, InternedString};
use parser;
use grammar::parse_tree as pt;
use grammar::repr as r;
use lr1;
use normalize;
use rust::RustWrite;

use std::env;
use std::io::{self, Write};
use std::process::exit;

mod filetext;

use self::filetext::FileText;

struct Options {
    input: Option<String>,
    start: Option<InternedString>,
}

pub fn main() {
    let options = parse_options();
    let grammar = parse_and_normalize_grammar(&options);
    emit_recursive_ascent(&options, &grammar);
}

fn usage(msg: &str) -> ! {
    println!("Usage: lalrpop [options]");
    println!("  --grammar <file>      read grammar from <file>, not stdin");
    println!("  --start <ident>       start nonterminal");
    println!("");
    println!("{}", msg);

    exit(1)
}

fn parse_options() -> Options {
    let mut args = env::args().skip(1);

    let mut options = Options {
        input: None,
        start: None,
    };

    while let Some(arg) = args.next() {
        if arg == "--grammar" {
            if options.input.is_some() {
                usage("Error: multiple grammars provided");
            }

            options.input = Some(expect_arg(&arg, args.next()));
        } else if arg == "--start" {
            if options.start.is_some() {
                usage("Error: multiple start terminals provided");
            }

            let s = expect_arg(&arg, args.next());
            options.start = Some(intern(&s));
        } else if arg.starts_with("-") {
            usage(&format!("Error: unknown argument: {}", arg));
        } else {
            options.input = Some(arg);
        }
    }

    options
}

fn expect_arg<T>(arg: &str, o: Option<T>) -> T {
    match o {
        Some(o) => o,
        None => usage(&format!("Error: `{}` expects an argument", arg)),
    }
}

fn parse_and_normalize_grammar(options: &Options) -> r::Grammar {
    let input = match options.input {
        None => check_io(FileText::from_stdin()),
        Some(ref path) => check_io(FileText::from_path(path.clone())),
    };

    let grammar = match parser::parse_grammar(input.text()) {
        Ok(grammar) => grammar,
        Err(error) => {
            report_error(&input,
                         pt::Span(error.offset, error.offset+1),
                         &format!("expected {}", error.expected))
        }
    };

    match normalize::normalize(grammar) {
        Ok(grammar) => grammar,
        Err(error) => {
            report_error(&input,
                         error.span,
                         &error.message)
        }
    }
}

fn emit_recursive_ascent(options: &Options,
                         grammar: &r::Grammar)
{
    let stdout: &mut Write = &mut io::stdout();
    let mut rust = RustWrite::new(stdout);

    check_io(emit_uses(grammar, &mut rust));

    if grammar.start_nonterminals.is_empty() {
        println!("Error: no public symbols declared in grammar");
        exit(1);
    }

    for &start_nt in &grammar.start_nonterminals {
        if grammar.productions_for(start_nt).is_empty() {
            println!("Error: public symbol {} has no defined productions", start_nt);
            exit(1);
        }

        let states = match lr1::build_states(&grammar, start_nt) {
            Ok(states) => states,
            Err(error) => {
                check_io(lr1::report_error(&mut io::stdout(), &grammar, &error));
                exit(1)
            }
        };

        check_io(lr1::ascent::compile(&grammar, start_nt, &states, &mut rust));
    }

    check_io(emit_action_code(options, grammar, &mut rust));
}

fn emit_uses<W:Write>(grammar: &r::Grammar,
                      rust: &mut RustWrite<W>)
                      -> io::Result<()>
{
    for u in &grammar.uses {
        rust!(rust, "use {};", u);
    }
    rust!(rust, "");
    Ok(())
}

fn emit_action_code<W:Write>(_options: &Options,
                             grammar: &r::Grammar,
                             rust: &mut RustWrite<W>)
                             -> io::Result<()>
{
    for (i, defn) in grammar.action_fn_defns.iter().enumerate() {
        rust!(rust, "fn {}action{}(", grammar.prefix, i);
        for (p, t) in defn.arg_patterns.iter().zip(defn.arg_types.iter()) {
            rust!(rust, "{}: {},", p, t);
        }
        rust!(rust, ") {{");
        rust!(rust, "{}", defn.code);
        rust!(rust, "}}");
    }
    Ok(())
}

fn report_error(file_text: &FileText, span: pt::Span, message: &str) -> ! {
    println!("{} error: {}", file_text.span_str(span), message);

    let out = io::stdout();
    let mut out = out.lock();
    file_text.highlight(span, &mut out).unwrap();

    exit(1);
}

fn check_io<T>(r: io::Result<T>) -> T {
    r.unwrap() // TODO print a nicer error
}
