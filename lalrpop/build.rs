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

fn find_lalrpop_binary(prefix: &str) -> Option<PathBuf> {
    let lalrpop_path = Path::new(prefix)
        .join("target")
        .join(env::var("PROFILE").unwrap())
        .join("lalrpop")
        .with_extension(env::consts::EXE_EXTENSION);
    if lalrpop_path.exists() {
        Some(lalrpop_path)
    } else {
        None
    }
}

fn main_() -> Result<(), Box<dyn Error>> {
    let grammar_file = "src/parser/lrgrammar.lalrpop";
    println!(r#"cargo:rerun-if-changed={}"#, grammar_file);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("cargo did not set OUT_DIR"));

    fs::create_dir_all(out_dir.join("src/parser"))?;

    if env::var("CARGO_FEATURE_TEST").is_ok() {
        let lalrpop_path = find_lalrpop_binary("..").or_else(|| find_lalrpop_binary("."))
            .unwrap_or_else(|| {
                panic!(
                    "Can't find a lalrpop binary to use for the snapshot. Make sure it is built and exists at target/{}/lalrpop!",
                    env::var("PROFILE").unwrap()
                )
            });

        let copied_grammar = out_dir.join("src/parser/lrgrammar.lalrpop");
        fs::copy(grammar_file, &copied_grammar)
            .map_err(|err| format!("Unable to grammar to OUT_DIR: {}", err))?;
        let status = Command::new(lalrpop_path)
            .args([
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
