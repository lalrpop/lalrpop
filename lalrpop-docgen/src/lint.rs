use std::{
    error::Error,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{railroad::LalrpopToRailroad, session::Session, util};

fn check(prefix: &str, predicate: bool, suffix: &str) {
    println!("{}", prefix);
    if predicate {
        println!("  OK:  {}", suffix);
    } else {
        println!("  ERROR: {}", suffix);
    }
}

fn check_stems(rules: &[String], path: PathBuf, ext: &str, subject: &str) {
    if let Ok(file_entry) = path.read_dir() {
        file_entry.for_each(|file_entry| {
            if let Ok(file_path) = file_entry.map(|f| f.path()) {
                if Some(ext) == file_path.extension().and_then(|e| e.to_str()) {
                    if let Some(stem) = file_path.file_stem() {
                        let stem = stem.to_string_lossy().to_string();
                        let stem_upper = stem.to_ascii_uppercase();
                        if !rules.contains(&stem_upper) {
                            println!(
                                "  ERROR {} has no associated rule `{}` in the grammar",
                                subject, &stem
                            );
                        }
                    }
                }
            }
        });
    }
}

pub(crate) fn lint(
    session: &Rc<Session>,
    railroad: &LalrpopToRailroad,
) -> Result<(), Box<dyn Error>> {
    if session.lint {
        lint_all(session, railroad)?;
    }

    Ok(())
}

fn lint_all(session: &Rc<Session>, railroad: &LalrpopToRailroad) -> Result<(), Box<dyn Error>> {
    let out_dir = util::out_dir(&session)?;
    let prolog_dir = util::prolog_dir(&session)?;
    let epilog_dir = util::epilog_dir(&session)?;
    let svg_dir = util::svg_dir(&session)?;
    let prolog_not_found_is_err = session.prolog_not_found_is_err;
    let epilog_not_found_is_err = session.epilog_not_found_is_err;

    check(
        "Has output folder?",
        out_dir.exists() && out_dir.is_dir(),
        &format!("<out_dir> => {}", &out_dir.to_string_lossy().to_string()),
    );
    println!();

    check(
        "Has svg diagrams folder?",
        out_dir.exists() && out_dir.is_dir(),
        &format!("<out_dir>/svg/` => {}/svg", out_dir.to_string_lossy()),
    );
    println!();

    println!("Has well formed grammar rules content?");
    println!(
        "  <svg_dir>/<rule>.svg => {}/<rule>.svg",
        svg_dir.to_string_lossy()
    );
    println!(
        "  <prolog_dir> => {}/<rule>.md",
        prolog_dir.to_string_lossy()
    );
    println!(
        "  <epilog_dir> => {}/<rule>.md",
        epilog_dir.to_string_lossy()
    );
    println!("  <rule> => Rule name in Grammar, lower case");
    println!();
    for rule_name in railroad.diagrams.keys() {
        let rule_lower = rule_name.to_ascii_lowercase();
        let prolog_where = format!("{}/{}.md", prolog_dir.to_string_lossy(), rule_lower);
        let prolog_path = Path::new(&prolog_where);
        let epilog_where = format!("{}/{}.md", epilog_dir.to_string_lossy(), rule_lower);
        let epilog_path = Path::new(&epilog_where);
        let svg_where = format!("{}/{}.svg", svg_dir.to_string_lossy(), rule_lower);
        let svg_path = Path::new(&svg_where);
        let svg_ok = svg_path.is_file() && svg_path.exists();
        let prolog_ok = if prolog_not_found_is_err {
            prolog_path.exists() && prolog_path.is_file()
        } else {
            true
        };
        let epilog_ok = if epilog_not_found_is_err {
            epilog_path.exists() && epilog_path.is_file()
        } else {
            true
        };
        if svg_ok && prolog_ok && epilog_ok {
            println!("  Rule: {}  OK", rule_name);
        } else {
            println!("  Rule: {}  ERROR", rule_name);
            if !prolog_ok {
                println!("    Prolog expected, but not found: {}", &prolog_where);
            }
            if !epilog_ok {
                println!("    Epilog expected, but not found: {}", &epilog_where);
            }
            if !svg_ok {
                println!("    SVG expected, but not found: {}", &svg_where);
            }
        }
    }
    println!();

    let rules: Vec<String> = railroad.diagrams.keys().map(|x| x.to_uppercase()).collect();
    println!("Check for orphaned rule SVG diagrams ( eg: rule renamed, deleted)?");
    check_stems(&rules, svg_dir, "svg", "SVG railroad diagram");
    println!();

    println!("Check for orphaned rule Prolog content ( eg: rule renamed, deleted)?");
    check_stems(&rules, prolog_dir, "md", "Prolog markdown");
    println!();

    println!("Check for orphaned rule Epilog content ( eg: rule renamed, deleted)?");
    check_stems(&rules, epilog_dir, "md", "Epilog markdown");
    println!();

    Ok(())
}
