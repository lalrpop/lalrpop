extern crate docopt;
extern crate lalrpop;
extern crate rustc_serialize;

use docopt::Docopt;
use lalrpop::{process_file, Level, Session};
use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    main1().unwrap();
}

fn main1() -> io::Result<()> {
    let mut stderr = std::io::stderr();

    let args: Args =
        Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit());

    let mut session = Session::new();

    match args.flag_level.unwrap_or(LevelFlag::Info) {
        LevelFlag::Quiet => session.set_log_level(Level::Taciturn),
        LevelFlag::Info => session.set_log_level(Level::Informative),
        LevelFlag::Verbose => session.set_log_level(Level::Verbose),
        LevelFlag::Debug => session.set_log_level(Level::Debug),
    }

    if args.flag_force {
        session.set_force_build();
    }

    if args.flag_help {
        try!(writeln!(stderr, "{}", USAGE));
        process::exit(1);
    }

    if args.arg_inputs.len() == 0 {
        try!(writeln!(stderr, "Error: no input files specified! Try -h for help."));
        process::exit(1);
    }

    for arg in args.arg_inputs {
        match process_file(&session, &arg) {
            Ok(()) => { }
            Err(err) => {
                try!(writeln!(stderr, "Error encountered processing `{}`: {}",
                              arg, err));
                process::exit(1);
            }
        }
    }

    Ok(())
}

const USAGE: &'static str = "
Usage: lalrpop-exe [options] <inputs>...

Options:
    -h, --help           Show this message.
    -l, --level LEVEL    Set the debug level. (Default: info)
                         Valid values: quiet, info, verbose, debug.
    -f, --force          Force execution, even if the .lalrpop file is older than the .rs file.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_inputs: Vec<String>,
    flag_level: Option<LevelFlag>,
    flag_force: bool,
    flag_help: bool,
}

#[derive(Debug, RustcDecodable)]
enum LevelFlag {
    Quiet, Info, Verbose, Debug
}
