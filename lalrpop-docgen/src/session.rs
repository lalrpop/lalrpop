use std::path;

use lalrpop::log::{Level, Log};

/// Various options to control debug output. Although this struct is
/// technically part of LALRPOP's exported interface, it is not
/// considered part of the semver guarantees as end-users are not
/// expected to use it.
#[derive(Clone)]
pub struct Session {
    pub log: lalrpop::log::Log,

    pub force_build: bool,

    pub in_dir: Option<path::PathBuf>,

    pub out_dir: Option<path::PathBuf>,

    pub prolog_dir: Option<path::PathBuf>,

    pub epilog_dir: Option<path::PathBuf>,

    pub railroad_css: Option<path::PathBuf>,

    pub grammar_cuts: Option<Vec<String>>,

    /// Emit EBNF grammar from input grammar.
    pub emit_ebnf: bool,

    /// Emit railroad diagrams from input grammar.
    pub emit_railroad: bool,

    /// Emit markdown source from input grammar.
    pub emit_markdown: bool,
}

impl Session {
    fn new() -> Self {
        Session {
            log: Log::new(Level::Informative),
            force_build: false,
            in_dir: None,
            out_dir: None,
            prolog_dir: None,
            epilog_dir: None,
            railroad_css: None,
            grammar_cuts: None,
            emit_ebnf: true,
            emit_railroad: true,
            emit_markdown: true,
        }
    }
}
impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}
