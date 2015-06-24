use intern::InternedString;
use grammar::parse_tree::{Alternative, ExprSymbol, Symbol};

#[derive(Debug)]
pub enum AlternativeAction<'a> {
    User(&'a str),
    Default(Symbols<'a>),
}

#[derive(Debug)]
pub enum Symbols<'a> {
    Named(Vec<(usize, InternedString, &'a Symbol)>),
    Anon(Vec<(usize, &'a Symbol)>),
}

pub fn analyze_action<'a>(alt: &'a Alternative) -> AlternativeAction<'a> {
    // We can't infer types for alternatives with actions
    if let Some(ref code) = alt.action {
        return AlternativeAction::User(code);
    }

    AlternativeAction::Default(analyze_expr(&alt.expr))
}

pub fn analyze_expr<'a>(expr: &'a ExprSymbol) -> Symbols<'a> {
    // First look for named symbols.
    let named_symbols: Vec<_> =
        expr.symbols
            .iter()
            .enumerate()
            .filter_map(|(idx, sym)| match *sym {
                Symbol::Name(id, ref sub) => Some((idx, id, &**sub)),
                _ => None,
            })
            .collect();
    if !named_symbols.is_empty() {
        return Symbols::Named(named_symbols);
    }

    // Otherwise, make a tuple of the items they chose with a `~`.
    let chosen_symbol_types: Vec<_> =
        expr.symbols
            .iter()
            .enumerate()
            .filter_map(|(idx, sym)| match *sym {
                Symbol::Choose(ref sub) => Some((idx, &**sub)),
                _ => None,
            })
            .collect();
    if !chosen_symbol_types.is_empty() {
        return Symbols::Anon(chosen_symbol_types);
    }

    // If they didn't choose anything with a `~`, make a tuple of everything.
    Symbols::Anon(expr.symbols.iter().enumerate().collect())
}
