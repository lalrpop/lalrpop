use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    if let Err(err) = main_() {
        eprintln!("{}", err);
        exit(1);
    }
}

fn main_() -> Result<(), Box<Error>> {
    // If compiling lalrpop itself, enable test parsers
    if Path::new("target").exists() {
        env::set_var("CARGO_FEATURE_TEST", "1");
        println!(r#"cargo:rustc-cfg=feature="test""#);
    }

    println!(r#"cargo:rerun-if-changed=src/parser/lrgrammar.lalrpop"#);
    let lalrpop_path = Path::new("target/debug/lalrpop");
    if lalrpop_path.exists() {
        let status = Command::new(lalrpop_path).status()?;
        if status.success() {
            Ok(())
        } else {
            Err("Compiling the .lalrpop file failed".into())
        }
    } else {
        Ok(())
    }
}
