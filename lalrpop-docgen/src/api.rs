use lalrpop::log::Level;
use std::error::Error;
use std::ffi::OsString;
use std::{env, path::Path, path::PathBuf, rc::Rc};

use crate::build;
use crate::session::Session;

#[derive(Clone, Default)]
pub struct Configuration {
    session: Session,
}

impl Configuration {
    /// Creates the default configuration; equivalent to `Configuration::default`.
    pub fn new() -> Configuration {
        Configuration::default()
    }

    /// Specify a custom directory to use when writing output files.
    /// By default, the output directory is the same as the input
    /// directory.
    pub fn set_out_dir<P>(&mut self, dir: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.session.out_dir = Some(dir.into());
        self
    }

    /// Specify a custom directory to use when writing output files.
    /// This directory contains optional markdown prolog files for generated
    /// markdown rules. The content is inserted into generated markdown for
    /// each grammar rule after the title ( rule name ) is emitted but before
    /// syntax diagram railroad and EBNF forms are emitted.
    ///
    pub fn set_prolog_dir<P>(&mut self, dir: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.session.prolog_dir = Some(dir.into());
        self
    }

    pub fn set_prolog_not_found_err(&mut self, val: bool) -> &mut Configuration {
        self.session.prolog_not_found_is_err = val;
        self
    }

    /// Specify a custom directory to use when writing output files.
    /// This directory contains optional markdown epilog files for generated
    /// markdown rules. The content is inserted into generated markdown for
    /// each grammar rule after the title prolog and generated syntax diagram
    /// and EBNF forms are emitted.
    ///
    pub fn set_epilog_dir<P>(&mut self, dir: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.session.epilog_dir = Some(dir.into());
        self
    }

    pub fn set_epilog_not_found_err(&mut self, val: bool) -> &mut Configuration {
        self.session.epilog_not_found_is_err = val;
        self
    }

    /// Specify a custom CSS stylesheet file to use when creating railroad
    /// diagram SVG source. If not specified a default internal CSS stylesheet
    /// is used during SVG generation of railroad diagrams.
    ///
    pub fn set_railroad_css<P>(&mut self, css_file: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.session.railroad_css = Some(css_file.into());
        self
    }

    pub fn set_grammar_cuts(&mut self, grammar_cuts: &[String]) -> &mut Self {
        self.session.grammar_cuts = if grammar_cuts.is_empty() {
            None
        } else {
            Some(grammar_cuts.to_vec())
        };
        self
    }

    /// If true, emit EBNF generated grammar.
    pub fn emit_ebnf(&mut self, val: bool) -> &mut Configuration {
        self.session.emit_ebnf = val;
        self
    }

    /// If true, emit railroad diagrams generated from grammar rules.
    pub fn emit_railroad(&mut self, val: bool) -> &mut Configuration {
        self.session.emit_railroad = val;
        self
    }

    /// Optional url style for svg reference 
    pub fn set_railroad_mode(&mut self, mode: String) -> &mut Configuration {
        self.session.railroad_mode= mode;
        self
    }

    /// Optional url prefix for svg reference
    pub fn set_railroad_prefix(&mut self, prefix: String) -> &mut Configuration {
        self.session.railroad_prefix = prefix;
        self
    }

    /// If true, emit EBNF generated grammar.
    pub fn emit_markdown(&mut self, val: bool) -> &mut Configuration {
        self.session.emit_markdown = val;
        self
    }

    pub fn set_markdown_lint(&mut self, val: bool) -> &mut Configuration {
        self.session.lint = val;
        self
    }

    /// Minimal logs: only for errors that halt progress.
    pub fn log_quiet(&mut self) -> &mut Configuration {
        self.session.log.set_level(Level::Taciturn);
        self
    }

    /// Informative logs: give some high-level indications of
    /// progress (default).
    pub fn log_info(&mut self) -> &mut Configuration {
        self.session.log.set_level(Level::Informative);
        self
    }

    /// Verbose logs: more than info, but still not overwhelming.
    pub fn log_verbose(&mut self) -> &mut Configuration {
        self.session.log.set_level(Level::Verbose);
        self
    }

    /// Debug logs: better redirect this to a file. Intended for
    /// debugging LALRPOP itself.
    pub fn log_debug(&mut self) -> &mut Configuration {
        self.session.log.set_level(Level::Debug);
        self
    }

    /// Process all `.lalrpop` files in `path`.
    pub fn process_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let mut session = self.session.clone();

        // If in dir is empty, use `src` by default.
        // See https://github.com/lalrpop/lalrpop/issues/280
        if session.in_dir.is_none() {
            let mut in_dir = env::current_dir()?;
            in_dir.push("src");
            session.in_dir = Some(in_dir);
        }

        // If out dir is empty, use `docs` by default.
        // See https://github.com/lalrpop/lalrpop/issues/280
        if session.out_dir.is_none() {
            session.log.log(Level::Debug, || {
                "Using environment variable `OUT_DIR` to set target directory".to_string()
            });
            let out_dir = match env::var_os("OUT_DIR") {
                Some(var) => var,
                None => OsString::from("docs"),
            };
            session.out_dir = Some(PathBuf::from(out_dir));
        }

        let session = Rc::new(session);
        build::process_dir(&session, path.as_ref())?;
        Ok(())
    }

    /// Process the given `.lalrpop` file.
    pub fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let session = Rc::new(self.session.clone());
        build::process_file(&session, path.as_ref())?;
        Ok(())
    }
}
