use std::path::Path;
use std::process::exit;

// Imports private to the visitor frontend
use lalrpop::grammar::parse_tree::EnumToken;
use lalrpop::grammar::parse_tree::GrammarItem as GI;

/// Re-exported for use by visitor backends
pub(crate) use lalrpop::grammar::parse_tree::Atom;
pub(crate) use lalrpop::grammar::parse_tree::ExprSymbol;
pub(crate) use lalrpop::grammar::parse_tree::Grammar;
pub(crate) use lalrpop::grammar::parse_tree::MacroSymbol;
pub(crate) use lalrpop::grammar::parse_tree::NonterminalData;
pub(crate) use lalrpop::grammar::parse_tree::NonterminalString;
pub(crate) use lalrpop::grammar::parse_tree::RepeatOp;
pub(crate) use lalrpop::grammar::parse_tree::RepeatSymbol;
pub(crate) use lalrpop::grammar::parse_tree::Symbol;
pub(crate) use lalrpop::grammar::parse_tree::SymbolKind as SK;
pub(crate) use lalrpop::grammar::parse_tree::TypeRef;
pub(crate) use lalrpop::grammar::repr::TerminalString;

use crate::util;

pub trait LalrpopVisitor {
    /// This is the entrypoint to the visitation process
    fn visit(&mut self, path: &Path)
    where
        Self: Sized,
    {
        let source = util::read_to_string(&path.to_string_lossy());

        let source = match source {
            Ok(source) => source,
            Err(e) => {
                eprintln!("{:?}", e);
                self.on_error(format!("Unable to read grammar file: {:?}", path));
                exit(-1);
            }
        };
        let grammar = lalrpop::parser::parse_grammar(&source);

        match grammar {
            Ok(grammar) => {
                self.on_prologue(path, &grammar);

                // Visit parser grammar
                self.on_grammar_start(path, &grammar);
                self.on_grammar(&grammar);
                self.on_grammar_end(&grammar);

                // Visit lexer grammar
                self.on_lexer_start();
                self.on_lexeme(grammar.enum_token());
                self.on_lexer_end();

                self.on_epilogue(path, &grammar);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                self.on_error(format!("Failed to parse grammar: {:?}", path));
            }
        }
    }
    /// Called before visitation begins
    fn on_prologue(&mut self, _path: &Path, _grammar: &Grammar) {
        // Do nothing by default
    }

    /// Called when grammar parsing begins
    fn on_grammar_start(&mut self, _source: &Path, _grammar: &Grammar) {
        // Do nothing by default
    }

    /// Called for a LALRPOP grammar reference
    fn on_grammar(&mut self, grammar: &Grammar)
    where
        Self: Sized,
    {
        for item in &grammar.items {
            match item {
                GI::Use(what) => self.on_use(what),
                GI::Nonterminal(rule) => {
                    self.on_rule_start(rule);
                    self.on_rule(rule);
                    self.on_rule_end(rule);
                }
                _ignore => (),
            }
        }
    }

    /// Called when grammar parsing begins
    fn on_grammar_end(&mut self, _grammar: &Grammar) {
        // Do nothing by default
    }

    /// Called before traversing an encountered grammar rule
    fn on_rule_start(&mut self, _rule: &NonterminalData)
    where
        Self: Sized,
    {
        // Do nothing by default
    }

    /// Called for each rule in a LALRPOP grammar
    fn on_rule(&mut self, rule: &NonterminalData)
    where
        Self: Sized,
    {
        self.on_type(&rule.type_decl);
        for alternative in &rule.alternatives {
            for symbol in &alternative.expr.symbols {
                self.on_symbol(symbol);
            }
        }
    }

    /// Called after traversing an encountered grammar rule
    fn on_rule_end(&mut self, _rule: &NonterminalData)
    where
        Self: Sized,
    {
        // Do nothing by default
    }

    /// Called when lexical procesing begins
    fn on_lexer_start(&mut self) {
        // Do nothing by default
    }

    fn on_lexeme(&mut self, _lexemes: Option<&EnumToken>) {
        // Do nothing by default
    }

    fn on_lexer_end(&mut self) {
        // Do nothing by default
    }

    /// Called for each `use` declaration in a LALRPOP file
    fn on_use(&mut self, _type_ref: &str) {
        // Do nothing by default
    }

    /// Called for any symbol encountered in a LALRPOP Grammar traversal
    fn on_symbol(&mut self, symbol: &Symbol)
    where
        Self: Sized,
    {
        match &symbol.kind {
            SK::Terminal(t) => self.on_terminal(t),
            SK::Repeat(r) => self.on_repetition(r),
            SK::Lookahead => self.on_lookahead(),
            SK::Lookbehind => self.on_lookbehind(),
            SK::Name(_name, ref symbol) => self.on_symbol(symbol),
            SK::Expr(ExprSymbol { symbols }) => {
                self.on_expr(symbols);
            }
            SK::Choose(symbol) => self.on_choice(symbol),
            SK::AmbiguousId(symbol) => self.on_ambiguous_id(symbol),
            SK::Macro(symbol) => self.on_macro(symbol),
            SK::Nonterminal(nts) => self.on_non_terminal(nts),
            SK::Error => (),
        }
    }

    /// Called when a rust type is encountered
    fn on_type(&mut self, _symbol: &Option<TypeRef>) {}

    /// Called when a terminal rule is encountered
    fn on_terminal(&mut self, _symbol: &TerminalString) {}

    /// Called when an expression is encountered
    fn on_expr(&mut self, _symbol: &[Symbol]) {}

    /// Called when repetition is encountered
    fn on_repetition(&mut self, _symbol: &RepeatSymbol) {}

    /// Called when a choice is encountered
    fn on_choice(&mut self, _symbol: &Symbol) {}

    /// Called when a macro identifier is encountered
    fn on_ambiguous_id(&mut self, _symbol: &Atom) {}

    /// Called when a non-terminal rule is encountered
    fn on_non_terminal(&mut self, _symbol: &NonterminalString) {}

    /// Called when a macro rule is encountered
    fn on_macro(&mut self, _symbol: &MacroSymbol) {}

    /// Called in the unlikely event of an error
    fn on_error(&mut self, _msg: String) {
        // Do nothing by default
    }

    /// Called when lookahead tokens are encountered
    fn on_lookahead(&mut self) {}

    /// Callec when lookbehind tokens are encountered
    fn on_lookbehind(&mut self) {}

    /// Called after all traversal has completed
    fn on_epilogue(&mut self, _path: &Path, _grammar: &Grammar) {
        // Do nothing by default
    }
}
