use style::{self, Style};
use log::{Log, Level};

/// Various options to control debug output. Although this struct is
/// technically part of LALRPOP's exported interface, it is not
/// considered part of the semver guarantees as end-users are not
/// expected to use it.
#[derive(Clone)]
pub struct Session {
    log: Log,
    force_build: bool,

    /// Stop after you find `max_errors` errors. If this value is 0,
    /// report *all* errors. Note that we MAY always report more than
    /// this value if we so choose.
    max_errors: usize,

    // Styles to use when formatting error reports
    pub heading: Style,
    pub ambig_symbols: Style,
    pub observed_symbols: Style,
    pub cursor_symbol: Style,
    pub unobserved_symbols: Style,
}

impl Session {
    pub fn new() -> Session {
        Session {
            log: Log::new(Level::Informative),
            force_build: false,
            max_errors: 1,
            heading: style::FG_WHITE.with(style::BOLD),
            ambig_symbols: style::FG_WHITE,
            observed_symbols: style::FG_BRIGHT_GREEN,
            cursor_symbol: style::FG_BRIGHT_WHITE,
            unobserved_symbols: style::FG_BRIGHT_RED,
        }
    }

    /// A session suitable for use in testing.
    #[cfg(test)]
    pub fn test() -> Session {
        Session {
            log: Log::new(Level::Debug),
            force_build: false,
            max_errors: 1,
            heading: Style::new(),
            ambig_symbols: Style::new(),
            observed_symbols: Style::new(),
            cursor_symbol: Style::new(),
            unobserved_symbols: Style::new(),
        }
    }

    pub fn set_force_build(&mut self) {
        self.force_build = true;
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

    pub fn log<M>(&self, level: Level, message: M)
        where M: FnOnce() -> String
    {
        self.log.log(level, message)
    }
}
