//! If an extern token is provided, then this pass validates that
//! terminal IDs have conversions. Otherwise, it generates a
//! tokenizer. This can only be done after macro expansion because
//! some macro arguments never make it into an actual production and
//! are only used in `if` conditions; we use string literals for
//! those, but they do not have to have a defined conversion.

use super::{NormResult, NormError};

use intern::{self, intern};
use lexer::re;
use lexer::dfa::{self, DFAConstructionError, Precedence};
use lexer::nfa::NFAConstructionError::*;
use grammar::consts::*;
use grammar::parse_tree::*;
use collections::{Map, Set};

#[cfg(test)]
mod test;

pub fn validate(mut grammar: Grammar) -> NormResult<Grammar> {
    let (has_enum_token, match_block) = {
        let opt_match_token = grammar.match_token();

        let mut match_block = MatchBlock::default();

        if let Some(mt) = opt_match_token {
            // FIXME: This should probably move _inside_ the Validator
            for (idx, mc) in mt.contents.iter().enumerate() {
                let precedence = &mt.contents.len() - idx;
                for item in &mc.items {
                    // TODO: Maybe move this into MatchItem methods
                    match *item {
                        MatchItem::Unmapped(sym, span) => {
                            match_block.add_match_entry(precedence,
                                                 sym,
                                                 TerminalString::Literal(sym),
                                                 span)?;
                        }
                        MatchItem::Mapped(sym, user, span) => {
                            match_block.add_match_entry(precedence, sym, user, span)?;
                        }
                        MatchItem::CatchAll(_) => {
                            match_block.catch_all = true;
                        }
                    }
                }
            }
        } else {
            // no match block is equivalent to `match { _ }`
            match_block.catch_all = true;
        }

        let opt_enum_token = grammar.enum_token();
        let conversions = opt_enum_token.map(|et| {
                                                 et.conversions
                                                     .iter()
                                                     .map(|conversion| conversion.from)
                                                     .collect()
                                             });

        let mut validator = Validator {
            grammar: &grammar,
            conversions: conversions,
            match_block: match_block,
        };

        assert!(!opt_match_token.is_some() || !opt_enum_token.is_some(),
                "expected to not have both match and extern");

        try!(validator.validate());

        (opt_enum_token.is_some(), validator.match_block)
    };

    if !has_enum_token {
        try!(construct(&mut grammar, match_block));
    }

    Ok(grammar)
}

///////////////////////////////////////////////////////////////////////////
// Validation phase -- this phase walks the grammar and visits all
// terminals. If using an external set of tokens, it checks that all
// terminals have a defined conversion to some pattern. Otherwise,
// it collects all terminals into the `all_literals` set for later use.

struct Validator<'grammar> {
    grammar: &'grammar Grammar,

    /// If an external tokenizer is in use, then this will be
    /// `Some(_)` and will point to all the defined conversions. In
    /// that case, the other fields below are irrelevant.
    conversions: Option<Set<TerminalString>>,

    match_block: MatchBlock,
}

/// Data summarizing the `match { }` block, along with any literals we
/// scraped up.
#[derive(Default)]
struct MatchBlock {
    /// This map stores the `match { }` entries. If `match_catch_all`
    /// is true, then we will grow this set with "identity mappings"
    /// for new literals that we find.
    match_entries: Vec<MatchEntry>,

    /// The names of all terminals the user can legally type. If
    /// `match_catch_all` is true, then if we encounter additional
    /// terminal literals in the grammar, we will add them to this
    /// set.
    match_user_names: Set<TerminalString>,

    /// For each terminal literal that we have to match, the span
    /// where it appeared in user's source.  This can either be in the
    /// `match { }` section or else in the grammar somewhere (if added
    /// due to a catch-all, or there is no match section).
    spans: Map<TerminalLiteral, Span>,

    /// True if we should permit unrecognized literals to be used.
    catch_all: bool,
}

impl MatchBlock {
    fn add_match_entry(&mut self,
                       match_group_precedence: usize,
                       sym: TerminalLiteral,
                       user_name: TerminalString,
                       span: Span)
                       -> NormResult<()> {
        if let Some(_old_span) = self.spans.insert(sym, span) {
            return_err!(span, "multiple match entries for `{}`", sym);
        }

        // NB: It's legal for multiple regex to produce same terminal.
        self.match_user_names.insert(user_name);

        self.match_entries
            .push(MatchEntry {
                      precedence: match_group_precedence * 2 + sym.base_precedence(),
                      match_literal: sym,
                      user_name: user_name,
                  });
        Ok(())
    }

    fn add_literal_from_grammar(&mut self, sym: TerminalLiteral, span: Span) -> NormResult<()> {
        // Already saw this literal, maybe in a match entry, maybe in the grammar.
        if self.match_user_names
               .contains(&TerminalString::Literal(sym)) {
            return Ok(());
        }

        if !self.catch_all {
            return_err!(span,
                        "terminal `{}` does not have a match mapping defined for it",
                        sym);
        }

        self.match_user_names
            .insert(TerminalString::Literal(sym));

        self.match_entries
            .push(MatchEntry {
                      precedence: sym.base_precedence(),
                      match_literal: sym,
                      user_name: TerminalString::Literal(sym),
                  });

        self.spans.insert(sym, span);

        Ok(())
    }
}

impl<'grammar> Validator<'grammar> {
    fn validate(&mut self) -> NormResult<()> {
        for item in &self.grammar.items {
            match *item {
                GrammarItem::Use(..) => {}
                GrammarItem::MatchToken(..) => {}
                GrammarItem::ExternToken(_) => {}
                GrammarItem::InternToken(_) => {}
                GrammarItem::Nonterminal(ref data) => {
                    for alternative in &data.alternatives {
                        try!(self.validate_alternative(alternative));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_alternative(&mut self, alternative: &Alternative) -> NormResult<()> {
        assert!(alternative.condition.is_none()); // macro expansion should have removed these
        try!(self.validate_expr(&alternative.expr));
        Ok(())
    }

    fn validate_expr(&mut self, expr: &ExprSymbol) -> NormResult<()> {
        for symbol in &expr.symbols {
            try!(self.validate_symbol(symbol));
        }
        Ok(())
    }

    fn validate_symbol(&mut self, symbol: &Symbol) -> NormResult<()> {
        match symbol.kind {
            SymbolKind::Expr(ref expr) => {
                try!(self.validate_expr(expr));
            }
            SymbolKind::Terminal(term) => {
                try!(self.validate_terminal(symbol.span, term));
            }
            SymbolKind::Nonterminal(_) => {}
            SymbolKind::Repeat(ref repeat) => {
                try!(self.validate_symbol(&repeat.symbol));
            }
            SymbolKind::Choose(ref sym) |
            SymbolKind::Name(_, ref sym) => {
                try!(self.validate_symbol(sym));
            }
            SymbolKind::Lookahead | SymbolKind::Lookbehind | SymbolKind::Error => {}
            SymbolKind::AmbiguousId(id) => {
                panic!("ambiguous id `{}` encountered after name resolution", id)
            }
            SymbolKind::Macro(..) => {
                panic!("macro not removed: {:?}", symbol);
            }
        }

        Ok(())
    }

    fn validate_terminal(&mut self, span: Span, term: TerminalString) -> NormResult<()> {
        match self.conversions {
            // If there is an extern token definition, validate that
            // this terminal has a defined conversion.
            Some(ref c) => {
                if !c.contains(&term) {
                    return_err!(span,
                                "terminal `{}` does not have a pattern defined for it",
                                term);
                }
            }

            // If there is no extern token definition, then collect
            // the terminal literals ("class", r"[a-z]+") into a set.
            None => {
                match term {
                    TerminalString::Bare(_) => {
                        assert!(self.match_block.match_user_names.contains(&term),
                                "bare terminal without match entry: {}",
                                term)
                    }

                    TerminalString::Literal(l) => {
                        self.match_block.add_literal_from_grammar(l, span)?
                    }

                    // Error is a builtin terminal that always exists
                    TerminalString::Error => (),
                }
            }
        }

        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////
// Construction phase -- if we are constructing a tokenizer, this
// phase builds up an internal token DFA.

fn construct(grammar: &mut Grammar, match_block: MatchBlock) -> NormResult<()> {
    let MatchBlock {
        mut match_entries,
        spans,
        ..
    } = match_block;

    // Sort match entries by order of increasing precedence.
    match_entries.sort();

    // Build up two vectors, one of parsed regular expressions and
    // one of precedences, that are parallel with `literals`.
    let mut regexs = Vec::with_capacity(match_entries.len());
    let mut precedences = Vec::with_capacity(match_entries.len());
    try!(intern::read(|interner| {
        for match_entry in &match_entries {
            precedences.push(Precedence(match_entry.precedence));
            match match_entry.match_literal {
                TerminalLiteral::Quoted(s) => {
                    regexs.push(re::parse_literal(interner.data(s)));
                }
                TerminalLiteral::Regex(s) => {
                    match re::parse_regex(interner.data(s)) {
                        Ok(regex) => regexs.push(regex),
                        Err(error) => {
                            let literal_span = spans[&match_entry.match_literal];
                            // FIXME -- take offset into account for
                            // span; this requires knowing how many #
                            // the user used, which we do not track
                            return_err!(literal_span, "invalid regular expression: {}", error);
                        }
                    }
                }
            }
        }
        Ok(())
    }));

    let dfa = match dfa::build_dfa(&regexs, &precedences) {
        Ok(dfa) => dfa,
        Err(DFAConstructionError::NFAConstructionError { index, error }) => {
            let feature = match error {
                NamedCaptures => r#"named captures (`(?P<foo>...)`)"#,
                NonGreedy => r#""non-greedy" repetitions (`*?` or `+?`)"#,
                WordBoundary => r#"word boundaries (`\b` or `\B`)"#,
                LineBoundary => r#"line boundaries (`^` or `$`)"#,
                TextBoundary => r#"text boundaries (`^` or `$`)"#,
            };
            let literal = match_entries[index.index()].match_literal;
            return_err!(spans[&literal],
                        "{} are not supported in regular expressions",
                        feature)
        }
        Err(DFAConstructionError::Ambiguity { match0, match1 }) => {
            let literal0 = match_entries[match0.index()].match_literal;
            let literal1 = match_entries[match1.index()].match_literal;
            // FIXME(#88) -- it'd be nice to give an example here
            return_err!(spans[&literal0],
                        "ambiguity detected between the terminal `{}` and the terminal `{}`",
                        literal0,
                        literal1)
        }
    };

    grammar
        .items
        .push(GrammarItem::InternToken(InternToken {
                                           match_entries: match_entries,
                                           dfa: dfa,
                                       }));

    // we need to inject a `'input` lifetime and `input: &'input str` parameter as well:

    let input_lifetime = intern(INPUT_LIFETIME);
    for parameter in &grammar.type_parameters {
        match *parameter {
            TypeParameter::Lifetime(i) if i == input_lifetime => {
                return_err!(grammar.span,
                            "since there is no external token enum specified, \
                     the `'input` lifetime is implicit and cannot be declared");
            }
            _ => {}
        }
    }

    let input_parameter = intern(INPUT_PARAMETER);
    for parameter in &grammar.parameters {
        if parameter.name == input_parameter {
            return_err!(grammar.span,
                        "since there is no external token enum specified, \
                 the `input` parameter is implicit and cannot be declared");
        }
    }

    grammar
        .type_parameters
        .insert(0, TypeParameter::Lifetime(input_lifetime));

    let parameter = Parameter {
        name: input_parameter,
        ty: TypeRef::Ref {
            lifetime: Some(input_lifetime),
            mutable: false,
            referent: Box::new(TypeRef::Id(intern("str"))),
        },
    };
    grammar.parameters.push(parameter);

    Ok(())
}
