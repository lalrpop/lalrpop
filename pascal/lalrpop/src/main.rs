use lalrpop_util::lalrpop_mod;

use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

use pico_args::Arguments;

lalrpop_mod!(pascal);

const USAGE: &str = "
Usage: pascal <inputs>...

Parses each input file.

-h --help Print help.
";

#[derive(Debug)]
struct Args {
    arg_inputs: Vec<OsString>,
    flag_help: bool,
}

fn parse_args(mut args: Arguments) -> Result<Args, Box<dyn std::error::Error>> {
    Ok(Args {
        flag_help: args.contains(["-h", "--help"]),
        arg_inputs: args.finish(),
    })
}

fn main() {
    let args = parse_args(Arguments::from_env()).unwrap();

    if args.flag_help {
        println!("{}", USAGE);
        return;
    }

    for input in &args.arg_inputs {
        let input = Path::new(input);
        let mut s = String::new();
        if let Err(err) = File::open(input).and_then(|mut f| f.read_to_string(&mut s)) {
            println!("Input `{}`: I/O Error {}", input.display(), err);
            continue;
        }

        let time_stamp = Instant::now();
        let result = pascal::fileParser::new().parse(&s);
        let elapsed = time_stamp.elapsed();
        let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

        match result {
            Ok(()) => println!("Input `{}` ({}s): OK", input.display(), elapsed),
            Err(err) => println!(
                "Input `{}` ({}s): parse error {:?}",
                input.display(),
                elapsed,
                err
            ),
        }
    }
}
