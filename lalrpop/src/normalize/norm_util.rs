use intern::InternedString;
use grammar::parse_tree::{ActionKind, Alternative, ExprSymbol, Symbol, SymbolKind};

#[derive(Debug)]
pub enum AlternativeAction<'a> {
    User(&'a ActionKind),
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
            .filter_map(|(idx, sym)| match sym.kind {
                SymbolKind::Name(id, ref sub) => Some((idx, id, &**sub)),
                _ => None,
            })
            .collect();
    if !named_symbols.is_empty() {
        return Symbols::Named(named_symbols);
    }

    // Otherwise, make a tuple of the items they chose with `<>`.
    let chosen_symbol_types: Vec<_> =
        expr.symbols
            .iter()
            .enumerate()
            .filter_map(|(idx, sym)| match sym.kind {
                SymbolKind::Choose(ref sub) => Some((idx, &**sub)),
                _ => None,
            })
            .collect();
    if !chosen_symbol_types.is_empty() {
        return Symbols::Anon(chosen_symbol_types);
    }

    // If they didn't choose anything with `<>`, make a tuple of everything.
    Symbols::Anon(expr.symbols.iter().enumerate().collect())
}

// @TODO: When 'struct field shorthands' become stable then curly-brackets detection mechanism will be not-needed
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FunkyExpressionPresence
{
    None,
    InCurlyBrackets,
    Normal
}

pub fn check_funky_expression(action: &str) -> FunkyExpressionPresence
{
    if let Some(funky_index) = action.find("<>") {
        let (before, after) = {
            let (before, after) = action.split_at(funky_index);
            (before.trim(), after[2..].trim())
        };

        let last_before = before.chars().last();
        let next_after = after.chars().next();
        if let (Some('{'), Some('}')) = (last_before, next_after) {
            FunkyExpressionPresence::InCurlyBrackets
        } else {
            FunkyExpressionPresence::Normal
        }
    } else {
        FunkyExpressionPresence::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detecting_normal_funky_expression() {
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("<>"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("ble <> blaa"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("ble <> } b"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("bl{ e <> } b"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("bl{ e <>} b"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("bl{ e <> e } b"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("bl{ <> e } b"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("bl{<> e } b"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("bl{<>"));
        assert_eq!(FunkyExpressionPresence::Normal, check_funky_expression("<>}"));
    }

    #[test]
    fn detecting_nopresence_of_funky_expression() {
        assert_eq!(FunkyExpressionPresence::None, check_funky_expression("< >"));
        assert_eq!(FunkyExpressionPresence::None, check_funky_expression("ble <b> blaa"));
    }

    #[test]
    fn detecting_incurlybrackets_funky_expression() {
        assert_eq!(FunkyExpressionPresence::InCurlyBrackets, check_funky_expression("{<>}"));
        assert_eq!(FunkyExpressionPresence::InCurlyBrackets, check_funky_expression("ble{<> }blaa"));
        assert_eq!(FunkyExpressionPresence::InCurlyBrackets, check_funky_expression("ble{ <> } b"));
        assert_eq!(FunkyExpressionPresence::InCurlyBrackets, check_funky_expression("bl{         <>} b"));
        assert_eq!(FunkyExpressionPresence::InCurlyBrackets, check_funky_expression("bl{<>} b"));
        assert_eq!(FunkyExpressionPresence::InCurlyBrackets, check_funky_expression("bl{<>         } b"));
    }

}