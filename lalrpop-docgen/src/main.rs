use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

mod api;
mod build;
mod ebnf;
mod lint;
mod railroad;
pub(crate) mod session;
mod util;
pub mod visitor;

use api::Configuration;
use pico_args::Arguments;

static VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
Usage:   lalrpop-railroad [options] <inputs>...
         lalrpop-railroad --help
         lalrpop-railroad (-V | --version)

Options:
    -h,   --help            Print help.
    -V,   --version         Print version.
    -ll,  --log-level LEVEL Set the debug level. (Default: info)
                            Valid values: quiet, info, verbose, debug.
    -o,   --out-dir DIR     Sets the directory in which to output the generated file(s).
    -e,   --ebnf            Generate equivalent EBNF for input grammar(s).
    -r,   --railroad        Generate railroad diagrams for input grammar(s).
    -rc,  --railroad-css    Provide a custom CSS stylesheet for railroad diagrams.
    -rp,  --railroad-prefix Prefix for generated SVG resource file img src references.
    -m,   --markdown        Generate markdown files for input grammar(s).
                            Markdown enables EBNF and Railroad generation.
    -mp,  --markdown-prolog Sets directory for markdown prolog content. 1 file per rule.
    -me,  --markdown-epilog Sets directory for markdown epilog content. 1 file per rule.
    -l,  --lint             Sets lint mode for markdown lint checks ( default: enabled )
    -lp, --lint-prolog-err  Disables lint mode for markdown prolog content ommissions as errors ( default: enabled ).
    -le, --lint-epilog-err  Disables lint mode for markdown epilog content ommissions as errors ( default: enabled ).
    -gc,  --grammar-cuts    A list of grammar rules to split traversal by.
";

#[derive(Debug)]
struct Args {
    arg_inputs: Vec<String>,
    flag_out_dir: Option<PathBuf>,
    flag_level: Option<LevelFlag>,
    flag_help: bool,
    flag_ebnf: bool,
    flag_railroad: bool,
    flag_railroad_css: Option<PathBuf>,
    flag_railroad_prefix: Option<String>,
    flag_markdown: bool,
    flag_markdown_prolog: Option<PathBuf>,
    flag_markdown_epilog: Option<PathBuf>,
    flag_lint: bool,
    flag_lint_prolog_error: bool,
    flag_lint_epilog_error: bool,
    flag_grammar_cuts: Option<Vec<String>>,
    flag_version: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum LevelFlag {
    Quiet,
    Info,
    Verbose,
    Debug,
}

impl FromStr for LevelFlag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::LevelFlag::*;
        match s {
            "quiet" => Ok(Quiet),
            "info" => Ok(Info),
            "verbose" => Ok(Verbose),
            "debug" => Ok(Debug),
            x => Err(format!("Unknown level {}", x)),
        }
    }
}

fn parse_args(mut args: Arguments) -> Result<Args, Box<dyn std::error::Error>> {
    Ok(Args {
        flag_out_dir: args.opt_value_from_fn(["-o", "--out-dir"], PathBuf::from_str)?,
        flag_level: args.opt_value_from_fn(["-ll", "--log-level"], LevelFlag::from_str)?,
        flag_help: args.contains(["-h", "--help"]),
        flag_ebnf: args.contains(["-e", "--ebnf"]),
        flag_railroad: args.contains(["-r", "--railroad"]),
        flag_railroad_css: args.opt_value_from_fn(["-rc", "--railroad-css"], PathBuf::from_str)?,
        flag_railroad_prefix: args.opt_value_from_str(["-rp", "--railroad-prefix"])?,
        flag_markdown: args.contains(["-m", "--markdown"]),
        flag_markdown_prolog: args
            .opt_value_from_fn(["-mp", "--markdown-prolog"], PathBuf::from_str)?,
        flag_markdown_epilog: args
            .opt_value_from_fn(["-me", "--markdown-epilog"], PathBuf::from_str)?,
        flag_lint: args.contains(["-l", "--lint"]),
        flag_lint_prolog_error: args.contains(["-lp", "--lint-prolog-err"]),
        flag_lint_epilog_error: args.contains(["-le", "--lint-epilog-err"]),
        flag_grammar_cuts: args
            .opt_value_from_str(["-gc", "--grammar-cuts"])?
            .map_or_else(
                || None,
                |x: String| {
                    Some(
                        x.split(',')
                            .filter(|y| !y.trim().is_empty())
                            .map(|y| y.trim().to_string())
                            .collect::<Vec<String>>(),
                    )
                },
            ),
        flag_version: args.contains(["-V", "--version"]),
        arg_inputs: args
            .finish()
            .into_iter()
            .map(|x| x.to_string_lossy().to_string())
            .collect(),
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stderr = std::io::stderr();
    let mut stdout = std::io::stdout();

    let args = parse_args(Arguments::from_env())?;
    let mut config = Configuration::new();

    if args.flag_help {
        writeln!(stderr, "{}", USAGE)?;
        process::exit(0);
    }

    if args.flag_version {
        writeln!(stdout, "{}", VERSION)?;
        process::exit(0);
    }

    if let Some(log_level) = args.flag_level {
        match log_level {
            LevelFlag::Debug => config.log_debug(),
            LevelFlag::Info => config.log_info(),
            LevelFlag::Quiet => config.log_quiet(),
            LevelFlag::Verbose => config.log_verbose(),
        };
    }

    if let Some(ref in_dir) = args.flag_markdown_prolog {
        config.set_prolog_dir(in_dir);
    }

    if let Some(ref in_dir) = args.flag_markdown_epilog {
        config.set_epilog_dir(in_dir);
    }

    if args.flag_lint {
        config.set_markdown_lint(true);

        if args.flag_lint_prolog_error {
            config.set_prolog_not_found_err(false);
        }

        if args.flag_lint_epilog_error {
            config.set_epilog_not_found_err(false);
        }
    }

    if let Some(ref out_dir) = args.flag_out_dir {
        config.set_out_dir(out_dir);
    }

    if args.flag_ebnf {
        config.emit_ebnf(true);
    }

    if args.flag_railroad {
        config.emit_railroad(true);
    }

    if let Some(ref railroad_css) = args.flag_railroad_css {
        config.set_railroad_css(railroad_css);
    }

    if let Some(ref railroad_prefix) = args.flag_railroad_prefix {
        config.set_railroad_prefix(railroad_prefix.to_string());
    }

    if args.flag_markdown {
        config.emit_markdown(true);
        config.emit_railroad(true);
        config.emit_ebnf(true);
    }

    if let Some(ref grammar_cuts) = args.flag_grammar_cuts {
        config.set_grammar_cuts(grammar_cuts);
    }

    if args.arg_inputs.is_empty() {
        writeln!(
            stderr,
            "Error: no input files specified! Try --help for help."
        )?;
        process::exit(1);
    }

    for arg in args.arg_inputs {
        let arg = Path::new(arg.as_str());

        let arg = if arg.is_relative() {
            format!(
                "{}/{}",
                std::env::current_dir()?.to_string_lossy(),
                arg.to_string_lossy()
            )
        } else {
            arg.to_string_lossy().to_string()
        };

        let arg = Path::new(arg.as_str());
        let disposition = if arg.is_dir() {
            config.process_dir(arg)
        } else {
            config.process_file(arg)
        };
        match disposition {
            Ok(()) => {}
            Err(err) => {
                writeln!(
                    stderr,
                    "Error encountered processing `{}`: {}",
                    arg.display(),
                    err
                )?;
                process::exit(1);
            }
        }
    }

    Ok(())
}
