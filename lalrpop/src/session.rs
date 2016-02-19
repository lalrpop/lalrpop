use std::default::Default;
use style::{self, Style};
use log::{Log, Level};

#[derive(Copy, Clone)]
pub enum ColorConfig {
    /// Use ANSI colors.
    Yes,

    /// Do NOT use ANSI colors.
    No,

    /// Use them if we detect a TTY output (default).
    IfTty,
}

/// Various options to control debug output. Although this struct is
/// technically part of LALRPOP's exported interface, it is not
/// considered part of the semver guarantees as end-users are not
/// expected to use it.
#[derive(Clone)]
pub struct Session {
    log: Log,

    force_build: bool,

    /// Emit comments in generated code explaining the states and so
    /// forth.
    emit_comments: bool,

    color_config: ColorConfig,

    /// Stop after you find `max_errors` errors. If this value is 0,
    /// report *all* errors. Note that we MAY always report more than
    /// this value if we so choose.
    max_errors: usize,

    // Styles to use when formatting error reports

    /// Applied to the heading in a message.
    pub heading: Style,

    /// Applied to symbols in an ambiguity report (where there is no cursor)
    pub ambig_symbols: Style,

    /// Applied to symbols before the cursor in a local ambiguity report
    pub observed_symbols: Style,

    /// Applied to symbols at the cursor in a local ambiguity report,
    /// if it is a non-terminal
    pub cursor_symbol: Style,

    /// Applied to symbols after the cursor in a local ambiguity report
    pub unobserved_symbols: Style,

    /// Applied to terminal symbols, in addition to the above styles
    pub terminal_symbol: Style,

    /// Applied to nonterminal symbols, in addition to the above styles
    pub nonterminal_symbol: Style,

    /// Style to use when printing "Hint:"
    pub hint_text: Style,
}

impl Session {
    pub fn new() -> Session {
        Session {
            log: Log::new(Level::Informative),
            force_build: false,
            emit_comments: false,
            color_config: ColorConfig::default(),
            max_errors: 1,
            heading: style::FG_WHITE.with(style::BOLD),
            ambig_symbols: style::FG_WHITE,
            observed_symbols: style::FG_BRIGHT_GREEN,
            cursor_symbol: style::FG_BRIGHT_WHITE,
            unobserved_symbols: style::FG_BRIGHT_RED,
            terminal_symbol: style::BOLD,
            nonterminal_symbol: style::DEFAULT,
            hint_text: style::FG_BRIGHT_MAGENTA.with(style::BOLD),
        }
    }

    /// A session suitable for use in testing.
    #[cfg(test)]
    pub fn test() -> Session {
        Session {
            log: Log::new(Level::Debug),
            force_build: false,
            emit_comments: false,
            color_config: ColorConfig::IfTty,
            max_errors: 1,
            heading: Style::new(),
            ambig_symbols: Style::new(),
            observed_symbols: Style::new(),
            cursor_symbol: Style::new(),
            unobserved_symbols: Style::new(),
            terminal_symbol: Style::new(),
            nonterminal_symbol: Style::new(),
            hint_text: Style::new(),
        }
    }

    pub fn color_config(&self) -> ColorConfig {
        self.color_config
    }

    pub fn set_color_config(&mut self, config: ColorConfig) {
        self.color_config = config;
    }

    pub fn set_force_build(&mut self) {
        self.force_build = true;
    }

    pub fn set_emit_comments(&mut self) {
        self.emit_comments = true;
    }

    pub fn set_max_errors(&mut self, errors: usize) {
        self.max_errors = errors;
    }

    pub fn set_log_level(&mut self, level: Level) {
        self.log.set_level(level);
    }

    /// Indicates whether we should stop after `actual_errors` number
    /// of errors have been reported.
    pub fn stop_after(&self, actual_errors: usize) -> bool {
        self.max_errors != 0 && actual_errors >= self.max_errors
    }

    pub fn force_build(&self) -> bool {
        self.force_build
    }

    pub fn emit_comments(&self) -> bool {
        self.emit_comments
    }

    pub fn log<M>(&self, level: Level, message: M)
        where M: FnOnce() -> String
    {
        self.log.log(level, message)
    }
}

impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        ColorConfig::IfTty
    }
}
