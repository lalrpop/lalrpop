use crate::session::Session;

use super::visitor::*;
use std::{collections::HashMap, error::Error, fs::File, io::Write, path::Path, rc::Rc};

pub struct LalrpopToEbnf {
    session: Rc<Session>,
    pub(crate) decl_order: Vec<String>,
    pub(crate) rules: HashMap<String, String>,
    current_rule_ebnf: String,
    ebnf_file: Option<File>,
    ebnf_md: Option<File>,
}

macro_rules! append {
    ($this:ident) => {
      $this.current_rule_ebnf.push_str("\n");
    };
    ($this:ident, $format:expr) => {{
      $this.current_rule_ebnf.push_str(&format!($format));
    }};
    ($this:ident, $format:expr, $($args:expr),*) => {{
      $this.current_rule_ebnf.push_str(&format!($format, $($args),*));
    }}
}

impl LalrpopToEbnf {
    pub(crate) fn new(session: &Rc<Session>) -> Result<Self, Box<dyn Error>> {
        Ok(LalrpopToEbnf {
            session: session.clone(),
            ebnf_file: if let Some(out) = &session.out_dir {
                Some(File::create(format!(
                    "{}/grammar.ebnf",
                    out.to_string_lossy()
                ))?)
            } else {
                Some(File::create("docs/grammar.ebnf")?)
            },
            ebnf_md: if let Some(out) = &session.out_dir {
                Some(File::create(format!(
                    "{}/grammar.md",
                    out.to_string_lossy()
                ))?)
            } else {
                Some(File::create("docs/grammar.md")?)
            },
            decl_order: vec![],
            rules: HashMap::new(),
            current_rule_ebnf: "".to_string(),
        })
    }
}

impl LalrpopVisitor for LalrpopToEbnf {
    fn on_prologue(&mut self, path: &Path, _grammar: &Grammar) -> Result<(), Box<dyn Error>> {
        if let Some(ebnf_md) = &mut self.ebnf_md {
            writeln!(
                ebnf_md,
                "## EBNF Grammar\n\nThis EBNF grammar was generated from: {:?}\n\n```ebnf\n\n",
                path
            )?;
        }
        Ok(())
    }

    fn on_epilogue(&mut self, _path: &Path, _grammar: &Grammar) -> Result<(), Box<dyn Error>> {
        if let Some(ebnf_md) = &mut self.ebnf_md {
            writeln!(ebnf_md, "```\n")?;
        }
        Ok(())
    }

    fn on_rule_start(&mut self, rule: &NonterminalData) -> Result<(), Box<dyn Error>> {
        if !rule.args.is_empty() {
            append!(self, "macro {}", rule.name);
        } else {
            append!(self, "rule {}", rule.name);
        }
        Ok(())
    }

    fn on_rule(&mut self, rule: &NonterminalData) -> Result<(), Box<dyn Error>> {
        self.session.log.log(lalrpop::log::Level::Debug, || {
            format!("EBNF visitor at rule {}", rule.name)
        });

        if !rule.args.is_empty() {
            append!(self, "<");
            append!(
                self,
                "{}",
                rule.args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            append!(self, ">");
        }
        append!(self, " ::=");
        append!(self);
        for (i, alternative) in rule.alternatives.iter().enumerate() {
            if i == 0 {
                append!(self, "    ")
            } else {
                append!(self, "  | ");
            }
            for symbol in &alternative.expr.symbols {
                self.on_symbol(symbol)?;
            }
            append!(self);
        }
        Ok(())
    }

    fn on_rule_end(&mut self, rule: &NonterminalData) -> Result<(), Box<dyn Error>> {
        append!(self, "  ;");
        append!(self);

        let content = self.current_rule_ebnf.clone();
        if let Some(ebnf_file) = &mut self.ebnf_file {
            writeln!(ebnf_file, "{}", &content)?;
        }
        if let Some(ebnf_md) = &mut self.ebnf_md {
            writeln!(ebnf_md, "{}", &content)?;
        }
        self.rules
            .insert(rule.name.to_string(), self.current_rule_ebnf.clone());
        self.decl_order.push(rule.name.to_string());
        self.current_rule_ebnf.clear();
        Ok(())
    }

    fn on_symbol(&mut self, symbol: &Symbol) -> Result<(), Box<dyn Error>> {
        match &symbol.kind {
            SK::Terminal(t) => self.on_terminal(t)?,
            SK::Repeat(r) => self.on_repetition(r)?,
            SK::Lookahead => self.on_lookahead()?,
            SK::Lookbehind => self.on_lookbehind()?,
            SK::Name(_name, ref symbol) => self.on_symbol(symbol)?,
            SK::Expr(ExprSymbol { symbols }) => self.on_expr(symbols)?,
            SK::Choose(symbol) => self.on_choice(symbol)?,
            SK::AmbiguousId(symbol) => self.on_ambiguous_id(symbol)?,
            SK::Macro(symbol) => self.on_macro(symbol)?,
            SK::Nonterminal(nts) => self.on_non_terminal(nts)?,
            SK::Error => (),
        }
        Ok(())
    }

    fn on_type(&mut self, symbol: &Option<TypeRef>) -> Result<(), Box<dyn Error>> {
        match symbol {
            Some(t) => {
                append!(self, "{}", t);
            }
            None => append!(self, "()"),
        }
        Ok(())
    }

    fn on_terminal(&mut self, symbol: &TerminalString) -> Result<(), Box<dyn Error>> {
        append!(
            self,
            r#" '{}' "#,
            symbol.to_string().replace('\"', "").replace('\\', "\\\\")
        );
        Ok(())
    }

    fn on_expr(&mut self, symbol: &[Symbol]) -> Result<(), Box<dyn Error>> {
        append!(self, "( ");
        for symbol in symbol {
            self.on_symbol(symbol)?;
        }
        append!(self, ") ");
        Ok(())
    }

    fn on_repetition(&mut self, symbol: &RepeatSymbol) -> Result<(), Box<dyn Error>> {
        match &symbol.op {
            RepeatOp::Plus => {
                self.on_symbol(&symbol.symbol)?;
                append!(self, "+  ");
            }
            RepeatOp::Star => {
                self.on_symbol(&symbol.symbol)?;
                append!(self, "*  ");
            }
            RepeatOp::Question => {
                self.on_symbol(&symbol.symbol)?;
                append!(self, "?  ");
            }
        }
        Ok(())
    }

    fn on_choice(&mut self, symbol: &Symbol) -> Result<(), Box<dyn Error>> {
        self.on_symbol(symbol)?;
        Ok(())
    }

    fn on_ambiguous_id(&mut self, symbol: &Atom) -> Result<(), Box<dyn Error>> {
        // Reference to another rule in the grammar - possibly not yet declared
        append!(self, "{} ", symbol);
        Ok(())
    }

    fn on_non_terminal(&mut self, symbol: &NonterminalString) -> Result<(), Box<dyn Error>> {
        append!(self, "{}", symbol.0.to_string());
        Ok(())
    }

    fn on_macro(&mut self, symbol: &MacroSymbol) -> Result<(), Box<dyn Error>> {
        let args = symbol
            .args
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        append!(self, "{}!({}) ", symbol.name, args);
        Ok(())
    }
}
