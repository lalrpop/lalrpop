use crate::grammar::parse_tree::{
    ActionKind, Alternative, ExprSymbol, Symbol, SymbolKind, TupleItem,
};

#[derive(Debug)]
pub enum AlternativeAction<'a> {
    User(&'a ActionKind),
    Default(Symbols<'a>),
}

#[derive(Debug)]
pub enum Symbols<'a> {
    Named(Vec<(usize, TupleItem, &'a Symbol)>),
    Anon(Vec<(usize, &'a Symbol)>),
}

pub fn analyze_action(alt: &Alternative) -> AlternativeAction<'_> {
    // We can't infer types for alternatives with actions
    if let Some(ref code) = alt.action {
        return AlternativeAction::User(code);
    }

    AlternativeAction::Default(analyze_expr(&alt.expr))
}

pub fn analyze_expr(expr: &ExprSymbol) -> Symbols<'_> {
    // First look for named symbols.
    let named_symbols: Vec<_> = expr
        .symbols
        .iter()
        .enumerate()
        .filter_map(|(idx, sym)| match sym.kind {
            SymbolKind::Name(ref id, ref sub) => Some((idx, TupleItem::Name(id.clone()), &**sub)),
            SymbolKind::Tuple(ref ids, ref sub) => {
                Some((idx, TupleItem::Tuple(ids.clone()), &**sub))
            }
            _ => None,
        })
        .collect();
    if !named_symbols.is_empty() {
        return Symbols::Named(named_symbols);
    }

    // Otherwise, make a tuple of the items they chose with `<>`.
    let chosen_symbol_types: Vec<_> = expr
        .symbols
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Presence {
    None,
    InCurlyBrackets,
    Normal,
}

impl Presence {
    pub fn is_in_curly_brackets(self) -> bool {
        self == Presence::InCurlyBrackets
    }
}

pub fn check_between_braces(action: &str) -> Presence {
    if let Some(funky_index) = action.find("<>") {
        let (before, after) = {
            let (before, after) = action.split_at(funky_index);
            (before.trim(), after[2..].trim())
        };

        // If we have an odd number of quotes on both sides, we're inside a string, and therefore,
        // this is a format arg, not a struct.  That's considered "Normal" here, because this is
        // detecting if our expansion is for a struct definition.
        let before_quotes = before.chars().filter(|c| *c == '"').count();
        let after_quotes = after.chars().filter(|c| *c == '"').count();

        if before_quotes % 2 == 1 && after_quotes % 2 == 1 {
            return Presence::Normal;
        }

        let last_before = before.chars().last();
        let next_after = after.chars().next();
        if let (Some('{'), Some('}')) = (last_before, next_after) {
            Presence::InCurlyBrackets
        } else {
            Presence::Normal
        }
    } else {
        Presence::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detecting_normal_funky_expression() {
        assert_eq!(Presence::Normal, check_between_braces("<>"));
        assert_eq!(Presence::Normal, check_between_braces("ble <> blaa"));
        assert_eq!(Presence::Normal, check_between_braces("ble <> } b"));
        assert_eq!(Presence::Normal, check_between_braces("bl{ e <> } b"));
        assert_eq!(Presence::Normal, check_between_braces("bl{ e <>} b"));
        assert_eq!(Presence::Normal, check_between_braces("bl{ e <> e } b"));
        assert_eq!(Presence::Normal, check_between_braces("bl{ <> e } b"));
        assert_eq!(Presence::Normal, check_between_braces("bl{<> e } b"));
        assert_eq!(Presence::Normal, check_between_braces("bl{<>"));
        assert_eq!(Presence::Normal, check_between_braces("<>}"));
    }

    #[test]
    fn detecting_nopresence_of_funky_expression() {
        assert_eq!(Presence::None, check_between_braces("< >"));
        assert_eq!(Presence::None, check_between_braces("ble <b> blaa"));
    }

    #[test]
    fn detecting_incurlybrackets_funky_expression() {
        assert_eq!(Presence::InCurlyBrackets, check_between_braces("{<>}"));
        assert_eq!(
            Presence::InCurlyBrackets,
            check_between_braces("ble{<> }blaa")
        );
        assert_eq!(
            Presence::InCurlyBrackets,
            check_between_braces("ble{ <> } b")
        );
        assert_eq!(
            Presence::InCurlyBrackets,
            check_between_braces("bl{         <>} b")
        );
        assert_eq!(Presence::InCurlyBrackets, check_between_braces("bl{<>} b"));
        assert_eq!(
            Presence::InCurlyBrackets,
            check_between_braces("bl{<>         } b")
        );
    }

    #[test]
    fn in_curly_braces_with_quotes() {
        assert_eq!(Presence::Normal, check_between_braces("\"a{<>}\""));
        assert_eq!(Presence::Normal, check_between_braces("\"foo\",\"a{<>}\""));
        assert_eq!(
            Presence::InCurlyBrackets,
            check_between_braces("\"foo\",a{<>}")
        );
    }
}
