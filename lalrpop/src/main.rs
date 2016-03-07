extern crate docopt;
extern crate lalrpop;
extern crate rustc_serialize;

use docopt::Docopt;
use lalrpop::Configuration;
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

    let mut config = Configuration::new();

    match args.flag_level.unwrap_or(LevelFlag::Info) {
        LevelFlag::Quiet => config.log_quiet(),
        LevelFlag::Info => config.log_info(),
        LevelFlag::Verbose => config.log_verbose(),
        LevelFlag::Debug => config.log_debug(),
    };

    if args.flag_force {
        config.force_build(true);
    }

    if args.flag_color {
        config.always_use_colors();
    }

    if args.flag_comments {
        config.emit_comments(true);
    }

    if args.arg_inputs.len() == 0 {
        try!(writeln!(stderr, "Error: no input files specified! Try --help for help."));
        process::exit(1);
    }

    for arg in args.arg_inputs {
        match config.process_file(&arg) {
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
Usage: lalrpop [options] inputs...
       lalrpop --help

Convert each of the given inputs (which should be a `.lalrpop` file)
into a `.rs` file, just as a `build.rs` script using LALRPOP would do.

Options:
    -l, --level LEVEL    Set the debug level. (Default: info)
                         Valid values: quiet, info, verbose, debug.
    -f, --force          Force execution, even if the .lalrpop file is older than the .rs file.
    -c, --color          Force colorful output, even if this is not a TTY.
    --comments           Enable comments in the generated code.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_inputs: Vec<String>,
    flag_level: Option<LevelFlag>,
    flag_force: bool,
    flag_color: bool,
    flag_comments: bool,
}

#[derive(Debug, RustcDecodable)]
enum LevelFlag {
    Quiet, Info, Verbose, Debug
}
