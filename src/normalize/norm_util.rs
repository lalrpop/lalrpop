use intern::InternedString;
use grammar::parse_tree::{Alternative, Symbol};

#[derive(Debug)]
pub enum AlternativeAction<'a> {
    User(&'a str),
    Default(Symbols<'a>),
}

#[derive(Debug)]
pub enum Symbols<'a> {
    Named(Vec<(InternedString, &'a Symbol)>),
    Anon(Vec<&'a Symbol>),
}

pub fn analyze_action<'a>(alt: &'a Alternative) -> AlternativeAction<'a> {
    // We can't infer types for alternatives with actions
    if let Some(ref code) = alt.action {
        return AlternativeAction::User(code);
    }

    AlternativeAction::Default(analyze_symbols(alt))
}

pub fn analyze_symbols<'a>(alt: &'a Alternative) -> Symbols<'a> {
    // First look for named symbols.
    let named_symbols: Vec<_> =
        alt.expr.symbols
                .iter()
                .filter_map(|sym| match *sym {
                    Symbol::Name(id, ref sub) => Some((id, &**sub)),
                    _ => None,
                })
                .collect();
    if !named_symbols.is_empty() {
        return Symbols::Named(named_symbols);
    }

    // Otherwise, make a tuple of the items they chose with a `~`.
    let chosen_symbol_types: Vec<_> =
        alt.expr.symbols
                .iter()
                .filter_map(|sym| match *sym {
                    Symbol::Choose(..) => Some(sym),
                    _ => None,
                })
                .collect();
    if !chosen_symbol_types.is_empty() {
        return Symbols::Anon(chosen_symbol_types);
    }

    // If they didn't choose anything with a `~`, make a tuple of everything.
    Symbols::Anon(alt.expr.symbols.iter().collect())
}
