use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

fn main() {
    if let Err(err) = main_() {
        eprintln!("{}", err);
        exit(1);
    }
}

fn main_() -> Result<(), Box<Error>> {
    let grammar_file = "src/parser/lrgrammar.lalrpop";
    println!(r#"cargo:rerun-if-changed={}"#, grammar_file);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("cargo did not set OUT_DIR"));

    fs::create_dir_all(out_dir.join("src/parser"))?;

    let target_dir = if Path::new("target").exists() {
        Path::new("target")
    } else {
        Path::new("../target")
    };

    let lalrpop_path = target_dir
        .join("debug/lalrpop")
        .with_extension(env::consts::EXE_EXTENSION);
    println!(r#"cargo:rerun-if-changed={}"#, lalrpop_path.display());

    if lalrpop_path.exists() {
        // If compiling lalrpop itself, enable test parsers
        if target_dir.exists() {
            env::set_var("CARGO_FEATURE_TEST", "1");
            println!(r#"cargo:rustc-cfg=feature="test""#);
        }

        let copied_grammar = out_dir.join("src/parser/lrgrammar.lalrpop");
        fs::copy(grammar_file, &copied_grammar)
            .map_err(|err| format!("Unable to grammar to OUT_DIR: {}", err))?;
        let status = Command::new(lalrpop_path)
            .args(&[
                "--force",
                "--features",
                "test",
                copied_grammar
                    .to_str()
                    .expect("grammar path is not valid UTF-8"),
            ])
            .status()?;
        if !status.success() {
            return Err("Compiling the .lalrpop file failed".into());
        }
    }
    Ok(())
}
