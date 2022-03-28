use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::{error::Error, rc::Rc};

use crate::ebnf::LalrpopToEbnf;
use crate::lint;
use crate::railroad::LalrpopToRailroad;
use crate::session::Session;
use crate::util::{maybe_mkdirp, out_dir};
use crate::visitor::LalrpopVisitor;

fn target_dir(session: &Rc<Session>) -> Result<String, Box<dyn Error>> {
    if let Some(out_dir) = &session.out_dir {
        let out_dir = out_dir.to_string_lossy().to_string();
        let out_path = Path::new(&out_dir);
        if out_path.exists() && out_path.is_dir() {
            Ok(out_dir)
        } else {
            fs::create_dir_all(out_path)?;
            Ok(out_dir)
        }
    } else {
        let out_dir = "docs".to_string();
        let out_path = Path::new(&out_dir);
        if out_path.exists() && out_path.is_dir() {
            Ok(out_dir)
        } else {
            fs::create_dir_all(out_path)?;
            Ok(out_dir)
        }
    }
}

fn find_grammars<P: AsRef<Path>>(
    session: &Session,
    path: P,
) -> Result<Vec<String>, Box<dyn Error>> {
    let path = path.as_ref();
    let mut found = vec![];
    if path.is_dir() {
        for file in path.read_dir()? {
            match file {
                Ok(entry) => {
                    if entry.path().is_symlink() {
                        continue;
                    }
                    if entry.path().is_file() {
                        match entry.path().extension() {
                            Some(ext) => {
                                if Some("lalrpop") == ext.to_str() {
                                    session.log.log(lalrpop::log::Level::Debug, || {
                                        format!("Found Grammar {:?}", entry.path())
                                    });
                                    let path = entry.path().to_string_lossy().to_string();
                                    found.push(path)
                                }
                            }
                            _otherwise => continue,
                        }
                    }
                    if entry.path().is_dir() {
                        let mut rest = find_grammars(session, entry.path())?;
                        found.append(&mut rest);
                    }
                }
                _ => continue,
            }
        }
    }

    Ok(found)
}

pub fn process_dir(session: &Rc<Session>, path: &Path) -> Result<(), Box<dyn Error>> {
    let grammars = find_grammars(session, path)?;

    for grammar_path in grammars {
        process_file(session, Path::new(&grammar_path))?;
    }

    Ok(())
}

pub fn process_file(session: &Rc<Session>, path: &Path) -> Result<(), Box<dyn Error>> {
    maybe_mkdirp(out_dir(&session)?)?;

    let mut ebnf = LalrpopToEbnf::new(session)?;
    let mut diagram = if let Some(grammar_cuts) = &session.grammar_cuts {
        LalrpopToRailroad::with_cuts(session, grammar_cuts)?
    } else {
        LalrpopToRailroad::with_no_cuts(session)?
    };

    if session.emit_markdown || session.emit_ebnf {
        ebnf.visit(path);
    }

    if session.emit_railroad || session.emit_markdown {
        diagram.visit(path);
    }

    if session.emit_markdown {
        // Produce a markdown page for each cut of the grammar
        for (name, content) in &diagram.top_rules {
            to_markdown(session, &diagram, name, content, &ebnf)?
        }
        // Produce a markdown page for entire grammar ( all rules )
        all_markdown(session, &diagram, "Full", &ebnf)?;
        lint::lint(session, &diagram)?;
    }

    Ok(())
}

fn prolog(session: &Rc<Session>, name: &str) -> Result<Option<String>, Box<dyn Error>> {
    if let Some(prolog_dir) = &session.prolog_dir {
        let prolog_path = format!("{}/{}.md", prolog_dir.to_string_lossy(), name.to_ascii_lowercase());
        if Path::new(&prolog_path).exists() {
            Ok(Some(fs::read_to_string(prolog_path)?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn epilog(session: &Rc<Session>, name: &str) -> Result<Option<String>, Box<dyn Error>> {
    if let Some(epilog_dir) = &session.epilog_dir {
        let epilog_path = format!("{}/{}.md", epilog_dir.to_string_lossy(), name.to_ascii_lowercase());
        if Path::new(&epilog_path).exists() {
            Ok(Some(fs::read_to_string(epilog_path)?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn rule(
    session: &Rc<Session>,
    name: &str,
    ebnf: Option<&String>,
    diagram: &str,
    s: &mut String,
) -> Result<(), Box<dyn Error>> {
    s.push_str(&format!("## Rule {}\n\n", name));
    if let Some(prolog) = prolog(session, name)? {
        s.push_str(&format!("{}\n\n{}\n\n", prolog, diagram));
    } else {
        s.push_str(&format!("{}\n\n", diagram));
    }
    if let Some(ebnf) = ebnf {
        s.push_str(&format!("```ebnf\n{}\n```\n\n", ebnf));
    }
    s.push('\n');
    if let Some(epilog) = epilog(session, name)? {
        s.push_str(&format!("\n{}\n", epilog));
    }
    s.push('\n');
    Ok(())
}

fn all_markdown(
    session: &Rc<Session>,
    railroad: &LalrpopToRailroad,
    partition: &str,
    ebnf: &LalrpopToEbnf,
) -> Result<(), Box<dyn Error>> {
    let target_dir = target_dir(session)?;
    let md_file = format!("{}/{}.md", target_dir, partition.to_ascii_lowercase());
    let mut f = File::create(md_file).expect("Unable to create file");

    let mut s = String::new();
    s.push_str(&format!("# {} Grammar\n", partition));
    s.push('\n');

    for name in &ebnf.decl_order {
        let ebnf = ebnf.rules.get(name);
        if let Some((diagram_ref, _diagram_svg)) = railroad.diagrams.get(name) {
            rule(session, name, ebnf, diagram_ref, &mut s)?;
        }
    }

    s.push('\n');
    f.write_all(s.as_bytes())
        .expect("Writing to full markdown file failed");
    Ok(())
}

fn to_markdown(
    session: &Rc<Session>,
    railroad: &LalrpopToRailroad,
    partition: &str,
    rule_cuts: &[String],
    ebnf: &LalrpopToEbnf,
) -> Result<(), Box<dyn Error>> {
    let target_dir = target_dir(session)?;
    let md_file = format!("{}/{}.md", target_dir, partition.to_ascii_lowercase());
    let mut f = File::create(md_file).expect("Unable to create file");

    let mut s = String::new();
    s.push_str(&format!("# {} Grammar\n", partition));
    s.push('\n');

    for name in rule_cuts {
        let ebnf = ebnf.rules.get(name);
        if let Some((diagram, _)) = railroad.diagrams.get(name) {
            rule(session, name, ebnf, diagram, &mut s)?;
        }
    }
    s.push('\n');
    f.write_all(s.as_bytes())
        .expect("Writing to grammar cut markdown file failed");
    Ok(())
}
