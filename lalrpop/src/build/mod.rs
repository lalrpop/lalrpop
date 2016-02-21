//! Utilies for running in a build script.

use atty;
use file_text::FileText;
use grammar::parse_tree as pt;
use grammar::repr as r;
use lalrpop_util::ParseError;
use lexer::intern_token;
use lr1;
use message::{Content, Message};
use message::builder::InlineBuilder;
use normalize;
use parser;
use rust::RustWrite;
use session::{ColorConfig, Session};
use term;
use tls::Tls;
use tok;
use walkdir;

use std::env::current_dir;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;

mod action;
mod fake_term;

use self::fake_term::FakeTerminal;

pub fn process() -> io::Result<()> {
    let session = Session::new();

    let src_dir = try!(current_dir()).join("src");

    // The environment variable OUT_DIR is set by cargo, and specifies
    // a directory where generated code should be put.
    process_dir(&session, &src_dir, Path::new(env!("OUT_DIR")))
}

pub fn process_root() -> io::Result<()> {
    let session = Session::new();
    let cwd = try!(current_dir());
    process_dir(&session, &cwd, &cwd)
}

pub fn process_root_unconditionally() -> io::Result<()> {
    let mut session = Session::new();
    session.set_force_build();
    let cwd = try!(current_dir());
    process_dir(&session, &cwd, &cwd)
}

fn process_dir(session: &Session, in_dir: &Path, out_dir: &Path) -> io::Result<()> {
    for relative_lalrpop_file in try!(lalrpop_files(in_dir)) {
        let rs_file = out_dir.join(&relative_lalrpop_file).with_extension("rs");
        let lalrpop_file = in_dir.join(&relative_lalrpop_file);
        try!(process_file_to(session, &lalrpop_file, &rs_file));
    }
    Ok(())
}

pub fn process_file<P:AsRef<Path>>(session: &Session, lalrpop_file: P) -> io::Result<()> {
    let rs_file = lalrpop_file.as_ref().with_extension("rs");
    process_file_to(session, lalrpop_file.as_ref(), &rs_file)
}

pub fn process_file_to<InPath, OutPath>(session: &Session,
                                        lalrpop_file: InPath,
                                        rs_file: OutPath)
                                        -> io::Result<()>
    where InPath: AsRef<Path>,
          OutPath: AsRef<Path>
{
    // Promote the session to an Rc so that we can stick it in TLS. I
    // don't want this to be part of LALRPOP's "official" interface
    // yet so don't take an `Rc<Session>` as an argument.
    let session = Rc::new(session.clone());
    let lalrpop_file = lalrpop_file.as_ref();
    let rs_file = rs_file.as_ref();
    if session.force_build() || try!(needs_rebuild(&lalrpop_file, &rs_file)) {
        log!(session, Informative, "processing file `{}`", lalrpop_file.to_string_lossy());
        try!(make_read_only(&rs_file, false));
        try!(remove_old_file(&rs_file));

        // Load the LALRPOP source text for this file:
        let file_text = Rc::new(try!(FileText::from_path(lalrpop_file.to_path_buf())));

        // Store the session and file-text in TLS -- this is not
        // intended to be used in this high-level code, but it gives
        // easy access to this information pervasively in the
        // low-level LR(1) and grammar normalization code. This is
        // particularly useful for error-reporting.
        let _tls = Tls::install(session.clone(), file_text.clone());

        // Do the LALRPOP processing itself and write the resulting
        // buffer into a file. We use a buffer so that if LR(1)
        // generation fails at some point, we don't leave a partial
        // file behind.
        {
            let grammar = try!(parse_and_normalize_grammar(&session, &file_text));
            let buffer = try!(emit_recursive_ascent(&session, &grammar));

            if let Some(parent) = rs_file.parent() {
                try!(fs::create_dir_all(parent));
            }

            let mut output_file = try!(fs::File::create(&rs_file));
            try!(output_file.write_all(&buffer));
        }

        try!(make_read_only(&rs_file, true));
    }
    Ok(())
}

fn remove_old_file(rs_file: &Path) -> io::Result<()> {
    match fs::remove_file(rs_file) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Unix reports NotFound, Windows PermissionDenied!
            match e.kind() {
                io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => Ok(()),
                _ => Err(e),
            }
        }
    }
}

fn needs_rebuild(lalrpop_file: &Path,
                 rs_file: &Path)
                 -> io::Result<bool>
{
    return match fs::metadata(&rs_file) {
        Ok(rs_metadata) => {
            let lalrpop_metadata = try!(fs::metadata(&lalrpop_file));
            Ok(compare_modification_times(&lalrpop_metadata, &rs_metadata))
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => Ok(true),
                _ => Err(e),
            }
        }
    };

    #[cfg(unix)]
    fn compare_modification_times(lalrpop_metadata: &fs::Metadata,
                                  rs_metadata: &fs::Metadata)
                                  -> bool
    {
        use std::os::unix::fs::MetadataExt;
        lalrpop_metadata.mtime() >= rs_metadata.mtime()
    }

    #[cfg(windows)]
    fn compare_modification_times(lalrpop_metadata: &fs::Metadata,
                                  rs_metadata: &fs::Metadata)
                                  -> bool
    {
        use std::os::windows::fs::MetadataExt;
        lalrpop_metadata.last_write_time() >= rs_metadata.last_write_time()
    }

    #[cfg(not(any(unix,windows)))]
    fn compare_modification_times(lalrpop_metadata: &fs::Metadata,
                                  rs_metadata: &fs::Metadata)
                                  -> bool
    {
        true
    }
}

fn make_read_only(rs_file: &Path, ro: bool) -> io::Result<()> {
    if rs_file.is_file() {
        let rs_metadata = try!(fs::metadata(&rs_file));
        let mut rs_permissions = rs_metadata.permissions();
        rs_permissions.set_readonly(ro);
        fs::set_permissions(&rs_file, rs_permissions)
    } else {
        Ok(())
    }
}

fn lalrpop_files(root_dir: &Path) -> io::Result<Vec<PathBuf>> {
    // Note: this function returns paths relative to root_dir
    let mut result = vec![];
    let depth = root_dir.components().count();
    for entry in walkdir::WalkDir::new(root_dir).follow_links(true) {
        let entry = try!(entry);
        let file_type = entry.file_type();
        let path = entry.path();

        if
            file_type.is_file() &&
            path.extension().is_some() &&
            path.extension().unwrap() == "lalrpop"
        {
            // Ensure that we can safely do the component stripping
            assert!(path.starts_with(root_dir));

            let mut components = path.components();

            // Can't use components.skip(depth) because then we can't
            // call as_path() after
            for _ in 0..depth {
                components.next();
            }

            result.push(components.as_path().to_path_buf());
        }
    }
    Ok(result)
}

fn parse_and_normalize_grammar(session: &Session,
                               file_text: &FileText)
                               -> io::Result<r::Grammar> {
    let grammar = match parser::parse_grammar(file_text.text()) {
        Ok(grammar) => grammar,

        Err(ParseError::InvalidToken { location }) => {
            let ch = file_text.text()[location..].chars().next().unwrap();
            report_error(&file_text,
                         pt::Span(location, location),
                         &format!("invalid character `{}`", ch));
        }

        Err(ParseError::UnrecognizedToken { token: None, expected: _ }) => {
            let len = file_text.text().len();
            report_error(&file_text,
                         pt::Span(len, len),
                         &format!("unexpected end of file"));
        }

        Err(ParseError::UnrecognizedToken { token: Some((lo, _, hi)), expected }) => {
            assert!(expected.is_empty()); // didn't implement this yet :)
            let text = &file_text.text()[lo..hi];
            report_error(&file_text,
                         pt::Span(lo, hi),
                         &format!("unexpected token: `{}`", text));
        }

        Err(ParseError::ExtraToken { token: (lo, _, hi) }) => {
            let text = &file_text.text()[lo..hi];
            report_error(&file_text,
                         pt::Span(lo, hi),
                         &format!("extra token at end of input: `{}`", text));
        }

        Err(ParseError::User { error }) => {
            let string = match error.code {
                tok::ErrorCode::UnrecognizedToken =>
                    "unrecognized token",
                tok::ErrorCode::UnterminatedEscape =>
                    "unterminated escape; missing '`'?",
                tok::ErrorCode::UnterminatedStringLiteral =>
                    "unterminated string literal; missing `\"`?",
                tok::ErrorCode::ExpectedStringLiteral =>
                    "expected string literal; missing `\"`?",
                tok::ErrorCode::UnterminatedCode =>
                    "unterminated code block; perhaps a missing `;`, `)`, `]` or `}`?"
            };

            report_error(&file_text,
                         pt::Span(error.location, error.location + 1),
                         string)
        }
    };

    match normalize::normalize(session, grammar) {
        Ok(grammar) => Ok(grammar),
        Err(error) => {
            report_error(&file_text,
                         error.span,
                         &error.message)
        }
    }
}

fn report_error(file_text: &FileText, span: pt::Span, message: &str) -> ! {
    println!("{} error: {}", file_text.span_str(span), message);

    let out = io::stderr();
    let mut out = out.lock();
    file_text.highlight(span, &mut out).unwrap();

    exit(1);
}

fn report_messages(messages: Vec<Message>) -> term::Result<()> {
    let builder = InlineBuilder::new().begin_paragraphs();
    let builder = messages.into_iter().fold(builder, |b, m| b.push(Box::new(m)));
    let content = builder.end().end();
    report_content(&*content)
}

fn report_content(content: &Content) -> term::Result<()> {
    // FIXME -- can we query the size of the terminal somehow?
    let canvas = content.emit_to_canvas(80);

    let try_colors = match Tls::session().color_config() {
        ColorConfig::Yes => true,
        ColorConfig::No => false,
        ColorConfig::IfTty => atty::is(),
    };

    if try_colors {
        if let Some(mut stdout) = term::stdout() {
            return canvas.write_to(&mut *stdout);
        }
    }

    let stdout = io::stdout();
    let mut stdout = FakeTerminal::new(stdout.lock());
    canvas.write_to(&mut stdout)
}

fn emit_uses<W:Write>(grammar: &r::Grammar,
                      rust: &mut RustWrite<W>)
                      -> io::Result<()>
{
    rust.write_uses("", grammar)
}

fn emit_recursive_ascent(session: &Session,
                         grammar: &r::Grammar)
                         -> io::Result<Vec<u8>>
{
    let mut rust = RustWrite::new(vec![]);

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

        log!(session, Verbose, "Building states for public nonterminal `{}`", user_nt);

        let states = match lr1::build_states(&grammar, start_nt) {
            Ok(states) => states,
            Err(error) => {
                let messages = lr1::report_error(&grammar, &error);
                let _ = report_messages(messages);
                exit(1) // FIXME -- propagate up instead of calling `exit`
            }
        };

        try!(lr1::ascent::compile(&grammar, user_nt, start_nt, &states, &mut rust));

        rust!(rust, "pub use self::{}parse{}::parse_{};",
              grammar.prefix, start_nt, user_nt);
    }

    if let Some(ref intern_token) = grammar.intern_token {
        try!(intern_token::compile(&grammar, intern_token, &mut rust));
    }

    try!(action::emit_action_code(grammar, &mut rust));

    try!(emit_to_triple_trait(grammar, &mut rust));

    Ok(rust.into_inner())
}

fn emit_to_triple_trait<W:Write>(grammar: &r::Grammar,
                                 rust: &mut RustWrite<W>)
                                 -> io::Result<()>
{
    #![allow(non_snake_case)]

    let L = grammar.types.terminal_loc_type();
    let T = grammar.types.terminal_token_type();
    let E = grammar.types.error_type();

    let mut user_type_parameters = String::new();
    for type_parameter in &grammar.type_parameters {
        user_type_parameters.push_str(&format!("{}, ", type_parameter));
    }

    rust!(rust, "");
    rust!(rust, "pub trait {}ToTriple<{}> {{", grammar.prefix, user_type_parameters);
    rust!(rust, "type Error;");
    rust!(rust, "fn to_triple(value: Self) -> Result<({},{},{}),Self::Error>;", L, T, L);
    rust!(rust, "}}");

    rust!(rust, "");
    if grammar.types.opt_terminal_loc_type().is_some() {
        rust!(rust, "impl<{}> {}ToTriple<{}> for ({}, {}, {}) {{",
              user_type_parameters, grammar.prefix, user_type_parameters, L, T, L);
        rust!(rust, "type Error = {};", E);
        rust!(rust, "fn to_triple(value: Self) -> Result<({},{},{}),{}> {{", L, T, L, E);
        rust!(rust, "Ok(value)");
        rust!(rust, "}}");
        rust!(rust, "}}");

        rust!(rust, "impl<{}> {}ToTriple<{}> for Result<({}, {}, {}),{}> {{",
              user_type_parameters, grammar.prefix, user_type_parameters, L, T, L, E);
        rust!(rust, "type Error = {};", E);
        rust!(rust, "fn to_triple(value: Self) -> Result<({},{},{}),{}> {{", L, T, L, E);
        rust!(rust, "value");
        rust!(rust, "}}");
        rust!(rust, "}}");
    } else {
        rust!(rust, "impl<{}> {}ToTriple<{}> for {} {{",
              user_type_parameters, grammar.prefix, user_type_parameters, T);
        rust!(rust, "type Error = {};", E);
        rust!(rust, "fn to_triple(value: Self) -> Result<((),{},()),{}> {{", T, E);
        rust!(rust, "Ok(((), value, ()))");
        rust!(rust, "}}");
        rust!(rust, "}}");

        rust!(rust, "impl<{}> {}ToTriple<{}> for Result<({}),{}> {{",
              user_type_parameters, grammar.prefix, user_type_parameters, T, E);
        rust!(rust, "type Error = {};", E);
        rust!(rust, "fn to_triple(value: Self) -> Result<((),{},()),{}> {{", T, E);
        rust!(rust, "value.map(|v| ((), v, ()))");
        rust!(rust, "}}");
        rust!(rust, "}}");
    }

    Ok(())
}
