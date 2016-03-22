extern crate docopt;
extern crate rustc_serialize;
extern crate time;

use docopt::Docopt;
use std::env;
use std::io::Read;
use std::fs::File;

mod pascal;

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.argv(env::args()).decode())
                         .unwrap_or_else(|e| e.exit());

    for input in &args.arg_inputs {
        let mut s = String::new();
        if let Err(err) = File::open(input).and_then(|mut f| f.read_to_string(&mut s)) {
            println!("Input `{}`: I/O Error {}",
                     input, err);
            continue;
        }

        let time_stamp = time::precise_time_s();
        let result = pascal::parse_file(&s);
        let elapsed = time::precise_time_s() - time_stamp;

        match result {
            Ok(()) => println!("Input `{}` ({}s): OK", input, elapsed),
            Err(err) => println!("Input `{}` ({}s): parse error {:?}", input, elapsed, err),
        }
    }
}

const USAGE: &'static str = "
Usage: pascal <inputs>...

Parses each input file.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_inputs: Vec<String>,
}
