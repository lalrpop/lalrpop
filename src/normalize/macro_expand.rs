use std::collections::HashSet;
use intern::{intern, InternedString};
use grammar::parse_tree::{Grammar, GrammarItem, MacroSymbol, Symbol};

pub fn expand_macros(input: Grammar) {
    let Grammar { type_name, items } = input;
    let mut expander = MacroExpander::new();
}

struct MacroExpander {
    expansion_stack: Vec<MacroSymbol>,
    expansion_set: HashSet<InternedString>,
}

impl MacroExpander {
    fn new() -> MacroExpander {
        MacroExpander { expansion_stack: Vec::new(), expansion_set: HashSet::new() }
    }

    fn expand(&mut self, items: &mut Vec<GrammarItem>) {
        let mut counter = 0;
        loop {
            // Find any macro uses in items added since last round and
            // replace them in place with the expanded version:
            for item in &mut items[counter..] {
                self.replace_item(item);
            }
            counter = items.len();

            // Drain macro queue:
        }
    }

    fn replace_item(&mut self, item: &mut GrammarItem) {
        match *item {
            GrammarItem::TokenType(..) => { }
            GrammarItem::Nonterminal(ref mut data) => {
                // Ignore macro definitions. They will be expanded in
                // due course.
                if !data.args.is_empty() {
                    return;
                }

                for alternative in &mut data.alternatives {
                    self.replace_symbols(&mut alternative.expr);
                }
            }
        }
    }

    fn replace_symbols(&mut self, symbols: &mut [Symbol]) {
        for symbol in symbols {
            self.replace_symbol(symbol);
        }
    }

    fn replace_symbol(&mut self, symbol: &mut Symbol) {
        let key;

        match *symbol {
            Symbol::Macro(ref mut m) => {
                for sym in &mut m.args {
                    self.replace_symbol(sym);
                }

                key = intern(&m.canonical_form());
                if self.expansion_set.insert(key) {
                    self.expansion_stack.push(m.clone());
                }
            }
            Symbol::Terminal(_) |
            Symbol::Nonterminal(_) => {
                return;
            }
            Symbol::Plus(ref mut sym) |
            Symbol::Question(ref mut sym) |
            Symbol::Star(ref mut sym) |
            Symbol::Choose(ref mut sym) |
            Symbol::Name(_, ref mut sym) => {
                self.replace_symbol(sym);
                return;
            }
            Symbol::Expr(ref mut syms) => {
                self.replace_symbols(syms);
                return;
            }
        }

        // we only get here if this is a macro expansion
        *symbol = Symbol::Nonterminal(key);
    }
}
