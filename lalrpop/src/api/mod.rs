use build;
use log::Level;
use session::{ColorConfig, Session};
use std::default::Default;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

/// Configure various aspects of how LALRPOP works.
/// Intended for use within a `build.rs` script.
/// To get the default configuration, use `Configuration::new`.
#[derive(Clone, Default)]
pub struct Configuration {
    session: Session
}

impl Configuration {
    /// Creates the default configuration; equivalent to `Configuration::default`.
    pub fn new() -> Configuration {
        Configuration::default()
    }

    /// Always use ANSI colors in output, even if output does not appear to be a TTY.
    pub fn always_use_colors(&mut self) -> &mut Configuration {
        self.session.color_config = ColorConfig::Yes;
        self
    }

    /// Never use ANSI colors in output, even if output appears to be a TTY.
    pub fn never_use_colors(&mut self) -> &mut Configuration {
        self.session.color_config = ColorConfig::No;
        self
    }

    /// Use ANSI colors in output if output appears to be a TTY, but
    /// not otherwise. This is the default.
    pub fn use_colors_if_tty(&mut self) -> &mut Configuration {
        self.session.color_config = ColorConfig::IfTty;
        self
    }

    /// If true, always convert `.lalrpop` files into `.rs` files, even if the
    /// `.rs` file is newer. Default is false.
    pub fn force_build(&mut self, val: bool) -> &mut Configuration {
        self.session.force_build = val;
        self
    }

    /// If true, emit comments into the generated code. This makes the
    /// generated code significantly larger. Default is false.
    pub fn emit_comments(&mut self, val: bool) -> &mut Configuration {
        self.session.emit_comments = val;
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

    /// Process all files in the current directory, which -- unless you
    /// have changed it -- is typically the root of the crate being compiled.
    pub fn process_current_dir(&self) -> Result<(), Box<Error>> {
        self.process_dir(try!(current_dir()))
    }

    /// Process all `.lalrpop` files in `path`.
    pub fn process_dir<P:AsRef<Path>>(&self, path: P) -> Result<(), Box<Error>> {
        let session = Rc::new(self.session.clone());
        try!(build::process_dir(session, path));
        Ok(())
    }

    /// Process the given `.lalrpop` file.
    pub fn process_file<P:AsRef<Path>>(&self, path: P) -> Result<(), Box<Error>> {
        let session = Rc::new(self.session.clone());
        try!(build::process_file(session, path));
        Ok(())
    }
}

/// Process all files in the current directory, which -- unless you
/// have changed it -- is typically the root of the crate being compiled.
///
/// Equivalent to `Configuration::new().process_current_dir()`.
pub fn process_root() -> Result<(), Box<Error>> {
    Configuration::new().process_current_dir()
}

/// Deprecated in favor of `Configuration`. Try:
///
/// ```rust
/// Configuration::new().force_build(true).process_current_dir()
/// ```
///
/// instead.
pub fn process_root_unconditionally() -> Result<(), Box<Error>> {
    Configuration::new().force_build(true).process_current_dir()
}
