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

    /// Folder to contain generated output files
    pub out_dir: Option<path::PathBuf>,

    /// Folder containing prolog markdown snippets
    pub prolog_dir: Option<path::PathBuf>,

    /// Flag to determine whether missing prolog files is ok or an error
    pub prolog_not_found_is_err: bool,

    /// Folder containing epilog markdown snippets
    pub epilog_dir: Option<path::PathBuf>,

    /// Flag to determine whether missing epilog files is ok or an error
    pub epilog_not_found_is_err: bool,

    /// Optional custom CSS for railroad diagrams
    pub railroad_css: Option<path::PathBuf>,

    /// Optional specification of slices of a grammar
    pub grammar_cuts: Option<Vec<String>>,

    /// Flag to control lint reports ( default: lint enabled )
    pub lint: bool,

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
            prolog_not_found_is_err: true,
            epilog_dir: None,
            epilog_not_found_is_err: true,
            railroad_css: None,
            grammar_cuts: None,
            emit_ebnf: true,
            emit_railroad: true,
            emit_markdown: true,
            lint: false,
        }
    }
}
impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}
