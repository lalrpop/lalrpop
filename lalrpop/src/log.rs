#[derive(Clone)]
pub struct Log {
    level: Level,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Level {
    /// No updates unless an error arises.
    Taciturn,

    /// Timing and minimal progress.
    Informative,

    /// More details, but still stuff an end-user is likely to understand.
    Verbose,

    /// Everything you could ever want and then some more.
    Debug,
}

impl Log {
    pub fn new(level: Level) -> Log {
        Log { level: level }
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    pub fn log<M>(&self, level: Level, message: M)
        where M: FnOnce() -> String
    {
        if self.level >= level {
            println!("{}", message());
        }
    }
}

macro_rules! log {
    ($session:expr, $level:ident, $($args:expr),*) => {
        $session.log(::log::Level::$level, || ::std::fmt::format(format_args!($($args),*)))
    }
}
