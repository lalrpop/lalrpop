use log::{Log, Level};

#[derive(Clone)]
pub struct Session {
    log: Log,
    force_build: bool,
}

impl Session {
    pub fn new() -> Session {
        Session {
            log: Log::new(Level::Informative),
            force_build: false,
        }
    }

    pub fn set_force_build(&mut self) {
        self.force_build = true;
    }

    pub fn set_log_level(&mut self, level: Level) {
        self.log.set_level(level);
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
