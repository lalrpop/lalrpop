use crate::session::Session;
use crate::util;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use super::visitor::*;

use railroad::svg;
use railroad::Choice;
use railroad::Comment;
use railroad::Diagram;
use railroad::Empty;
use railroad::End;
use railroad::LabeledBox;
use railroad::NonTerminal;
use railroad::Optional;
pub(crate) use railroad::RailroadNode;
use railroad::Repeat;
use railroad::Sequence;
use railroad::SimpleStart;
use railroad::Terminal;
use std::fs::File;
use std::io::Write;

pub const CSS: &str = r#"
svg.railroad {
  background-color: rgba(238, 238, 238, .91);
  background-size: 15px 15px;
  background-image: linear-gradient(to right, rgba(25, 25, 225, .65) 1px, transparent 1px),
                    linear-gradient(to bottom, rgba(25, 25, 225, .65) 1px, transparent 1px);
}

svg.railroad path {
  stroke-width: 3px;
  stroke: black;
  fill: transparent;
}

svg.railroad .debug {
  stroke-width: 1px;
  stroke: red;
}

svg.railroad text {
  font: 14px monospace;
  text-anchor: middle;
}

svg.railroad .nonterminal text {
  font-weight: bold;
}

svg.railroad text.comment {
  font: italic 12px monospace;
}

svg.railroad rect {
  stroke-width: 3px;
  stroke: black;
  fill:rgba(0, 0, 90, 0%);
}

svg.railroad .terminal rect {
  fill: aquamarine;  
}

svg.railroad .nonterminal rect {
  fill: orange;  
}

svg.railroad g.labeledbox > rect {
  border-radius: 25px;
  border: 2px solid #73AD21;
  stroke-width: 1.5px;
  stroke: grey;
  stroke-dasharray: 5px;
  fill:rgba(150, 150, 150, 25%);
}

"#;

fn css(session: &Rc<Session>) -> Result<String, Box<dyn Error>> {
    Ok(if let Some(custom_css_path) = &session.railroad_css {
        fs::read_to_string(&custom_css_path)?
    } else {
        CSS.to_string()
    })
}

fn svg_dir(session: &Rc<Session>) -> Result<String, Box<dyn Error>> {
    let svg_str = util::svg_dir(session)?.to_string_lossy().to_string();
    let svg_dir = Path::new(&svg_str);
    let svg_path = Path::new(&svg_dir);
    if svg_path.exists() && svg_path.is_dir() {
        Ok(svg_str)
    } else {
        fs::create_dir_all(svg_path)?;
        Ok(svg_str)
    }
}

pub(crate) struct RailroadNodeStack {
    scopes: Vec<Vec<Box<dyn RailroadNode>>>,
}

impl RailroadNodeStack {
    fn new() -> Self {
        Self {
            scopes: vec![vec![]],
        }
    }

    fn push(&mut self) {
        self.scopes.insert(0, vec![]);
    }

    fn pop(&mut self) -> Vec<Box<dyn RailroadNode>> {
        self.scopes.remove(0)
    }

    fn add(&mut self, item: Box<dyn RailroadNode>) {
        if let Some(top) = self.scopes.get_mut(0) {
            top.push(item)
        } else {
            panic!("")
        }
    }

    fn expr(&mut self) {
        let mut empty = vec![];
        std::mem::swap(&mut self.scopes[0], &mut empty);
        self.scopes[0].push(Box::new(Sequence::new(empty)));
    }

    fn macro_rule(&mut self, name: &str) {
        self.expr();
        let mut empty = vec![];
        std::mem::swap(&mut self.scopes[0], &mut empty);
        self.scopes[0].push(Box::new(LabeledBox::new(
            Sequence::new(empty),
            Comment::new(format!("Macro Rule: {}", name)),
        )));
    }

    fn repeat(&mut self, nodes: Vec<Box<dyn RailroadNode>>, kind: RepeatOp) {
        let current = Sequence::new(nodes);
        self.scopes[0].push(match kind {
            RepeatOp::Plus => Box::new(Repeat::new(current, Empty)),
            RepeatOp::Star => Box::new(Repeat::new(Empty, current)),
            RepeatOp::Question => Box::new(Optional::new(current)),
        });
    }

    fn clear(&mut self) {
        if let Some(top) = self.scopes.get_mut(0) {
            top.clear()
        }
    }
}

pub(crate) struct LalrpopToRailroad {
    session: Rc<Session>,
    svg_dir: String,
    css: String,
    pub(crate) tops: Option<Vec<String>>,
    pub(crate) top_rules: HashMap<String, Vec<String>>,
    pub(crate) current_top: Option<String>,
    pub(crate) current_rule: String,
    pub(crate) stack: RailroadNodeStack,
    pub(crate) diagrams: HashMap<String, (String, String)>,
}

impl LalrpopToRailroad {
    pub(crate) fn with_cuts(
        session: &Rc<Session>,
        top_rules: &[String],
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            session: session.clone(),
            svg_dir: svg_dir(session)?,
            tops: Some(top_rules.iter().map(|x| x.to_string()).collect()),
            top_rules: HashMap::new(),
            diagrams: HashMap::new(),
            css: css(session)?,
            current_top: None,
            current_rule: "".to_string(), // Overridden during traversal
            stack: RailroadNodeStack::new(),
        })
    }

    pub(crate) fn with_no_cuts(session: &Rc<Session>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            session: session.clone(),
            svg_dir: svg_dir(session)?,
            tops: Some(vec!["Full".to_string()]),
            top_rules: HashMap::new(),
            diagrams: HashMap::new(),
            css: css(session)?,
            current_top: None,
            current_rule: "Full".to_string(),
            stack: RailroadNodeStack::new(),
        })
    }

    pub(crate) fn is_top_level(&self, name: &String) -> bool {
        if let Some(tops) = &self.tops {
            if tops.contains(name) {
                return true;
            };
        }
        false
    }

    fn svg_to_ref(&self, name: &str, width: i64, height: i64) -> String {
        match self.session.railroad_mode.as_str() {
            "img" =>
                format!(
                    r#"<img src="{}svg/{}.svg" alt="{}" width="{}" height="{}"/>"#,
                    self.session.railroad_prefix,
                    name.to_ascii_lowercase(),
                    name.to_ascii_lowercase(),
                    width,
                    height,
                ),
           _otherwise =>
                format!(
                    "![{}]({}svg/{}.svg)",
                    name.to_string(),
                    self.session.railroad_prefix,
                    name.to_ascii_lowercase(),
                ),
        }
    }

    fn to_diagram(&mut self, rule: &NonterminalData) -> Result<(String, (String, String)), Box<dyn Error>> {
        let name = rule.name.to_string();
        let mut alts: Vec<Box<dyn RailroadNode>> = vec![];
        let is_singular = rule.alternatives.len() == 1;
        for alternative in &rule.alternatives {
            self.stack.clear();
            for symbol in &alternative.expr.symbols {
                self.on_symbol(symbol)?;
            }
            let mut pivot = vec![];
            std::mem::swap(&mut pivot, &mut self.stack.scopes[0]);
            alts.push(Box::new(Sequence::new(pivot)));
        }

        let mut nodes = Sequence::default();
        nodes.push(Box::new(SimpleStart));
        if is_singular {
            nodes.push(Box::new(Sequence::new(alts)));
        } else {
            nodes.push(Box::new(Choice::new(alts)));
        };
        nodes.push(Box::new(End));

        let mut dia = Diagram::new(nodes);
        dia.add_element(
            svg::Element::new("style")
                .set("type", "text/css")
                .text(&self.css),
        );

        let diagram_ref = self.svg_to_ref(&name, dia.width(), dia.height());
        let diagram_svg = dia.to_string();
        Ok((name, (diagram_ref, diagram_svg)))
    }
}

impl LalrpopVisitor for LalrpopToRailroad {
    fn on_rule(&mut self, rule: &NonterminalData) -> Result<(), Box<dyn Error>>
    where
        Self: Sized,
    {
        self.session.log.log(lalrpop::log::Level::Debug, || {
            format!("Railroad diagram visitor at rule {}", rule.name)
        });

        let name = rule.name.to_string();
        if self.is_top_level(&name) {
            self.current_top = Some(name.clone());
            self.top_rules.insert(name.clone(), vec![name.clone()]);
        } else if let Some(top) = &self.current_top {
            if let Some(rules) = self.top_rules.get_mut(top) {
                if !rules.contains(&name) {
                    rules.push(name.clone());
                }
            }
        }
        self.stack.clear();
        self.current_rule = name;
        let svg_file_name = format!("{}.svg", &self.current_rule.to_ascii_lowercase());
        let _txt_file_name = format!("{}.txt", &self.current_rule);

        let (name, (diagram_ref, diagram_svg)) = self.to_diagram(rule)?;
        self.diagrams
            .insert(name, (diagram_ref, diagram_svg.clone()));
        let mut f = File::create(format!("{}/{}", self.svg_dir, svg_file_name))?;
        f.write_all(diagram_svg.as_bytes())?;
        Ok(())
    }

    /// Called for any symbol encountered in a LALRPOP Grammar traversal
    fn on_symbol(&mut self, symbol: &Symbol) -> Result<(), Box<dyn Error>>
    where
        Self: Sized,
    {
        match &symbol.kind {
            SK::Terminal(t) => self.on_terminal(t)?,
            SK::Repeat(r) => self.on_repetition(r)?,
            SK::Lookahead => self.on_lookahead()?,
            SK::Lookbehind => self.on_lookbehind()?,
            SK::Name(_name, ref symbol) => self.on_symbol(symbol)?,
            SK::Expr(ExprSymbol { symbols }) => {
                self.on_expr(symbols)?;
            }
            SK::Choose(symbol) => self.on_choice(symbol)?,
            SK::AmbiguousId(symbol) => self.on_ambiguous_id(symbol)?,
            SK::Macro(symbol) => self.on_macro(symbol)?,
            SK::Nonterminal(nts) => self.on_non_terminal(nts)?,
            SK::Error => (), // self.on_error("Error".to_string()),
        }
        Ok(())
    }

    fn on_terminal(&mut self, symbol: &TerminalString) -> Result<(), Box<dyn Error>> {
        let t = Terminal::new(symbol.to_string());
        self.stack.add(Box::new(t));
        Ok(())
    }

    fn on_expr(&mut self, symbols: &[Symbol]) -> Result<(), Box<dyn Error>> {
        for symbol in symbols {
            self.on_symbol(symbol)?;
        }
        self.stack.expr();
        Ok(())
    }

    fn on_repetition(&mut self, symbol: &RepeatSymbol) -> Result<(), Box<dyn Error>> {
        self.stack.push();
        self.on_symbol(&symbol.symbol)?;
        let nodes = self.stack.pop();
        self.stack.repeat(nodes, symbol.op);
        Ok(())
    }

    fn on_choice(&mut self, symbol: &Symbol) -> Result<(), Box<dyn Error>> {
        self.stack.push();
        self.on_symbol(symbol)?;
        let nodes = self.stack.pop();
        self.stack.add(Box::new(Choice::new(nodes)));
        Ok(())
    }

    fn on_ambiguous_id(&mut self, symbol: &Atom) -> Result<(), Box<dyn Error>> {
        let nt = NonTerminal::new(symbol.to_string());
        self.stack.add(Box::new(nt));
        if let Some(top) = &self.current_top {
            if let Some(rules) = self.top_rules.get_mut(top) {
                let name = symbol.to_string();
                if !rules.contains(&name) {
                    rules.push(name);
                }
            }
        }
        Ok(())
    }

    fn on_non_terminal(&mut self, symbol: &NonterminalString) -> Result<(), Box<dyn Error>> {
        self.stack
            .add(Box::new(NonTerminal::new(symbol.to_string())));
        if let Some(top) = &self.current_top {
            if let Some(rules) = self.top_rules.get_mut(top) {
                let name = symbol.to_string();
                if !rules.contains(&name) {
                    rules.push(name);
                }
            }
        }
        Ok(())
    }

    fn on_macro(&mut self, symbol: &MacroSymbol) -> Result<(), Box<dyn Error>> {
        for arg in &symbol.args {
            self.on_symbol(arg)?;
        }
        self.stack.macro_rule(&symbol.name.to_string());
        if let Some(top) = &self.current_top {
            if let Some(rules) = self.top_rules.get_mut(top) {
                let name = symbol.to_string();
                if !rules.contains(&name) {
                    rules.push(name);
                }
            }
        }
        Ok(())
    }
}
