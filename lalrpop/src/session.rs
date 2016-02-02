use log::{Log, Level};

#[derive(Clone)]
pub struct Session {
    log: Log,
    force_build: bool,

    /// Stop after you find `max_errors` errors. If this value is 0,
    /// report *all* errors. Note that we MAY always report more than
    /// this value if we so choose.
    max_errors: usize,
}

impl Session {
    pub fn new() -> Session {
        Session {
            log: Log::new(Level::Informative),
            force_build: false,
            max_errors: 1,
        }
    }

    /// A session suitable for use in testing.
    #[cfg(test)]
    pub fn test() -> Session {
        Session {
            log: Log::new(Level::Debug),
            force_build: false,
            max_errors: 1,
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
