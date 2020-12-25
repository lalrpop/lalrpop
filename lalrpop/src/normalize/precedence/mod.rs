//! Precedence expander.
//!
//! Precedence expansion rewrites rules that contain precedence annotation into several rules
//! without annotations. A new rule is created for each level of precedence. Recursive occurrences
//! of the original rule are syntactically substituted for a level rule in each alternative, where
//! the choice of the precise rule is determined by the precedence level, the possible
//! associativity and the position of this occurrence.
//!
//! For concrete examples, see the [`test`](../tests/index.html) module.
use super::resolve;
use super::NormResult;
use crate::grammar::parse_tree::{
    Alternative, ExprSymbol, Grammar, GrammarItem, NonterminalData, NonterminalString, Symbol,
    SymbolKind,
};
use std::fmt;
use std::str::FromStr;
use string_cache::DefaultAtom as Atom;

#[cfg(test)]
mod test;

pub const PREC_ANNOT: &str = "precedence";
pub const LVL_ARG: &str = "level";
pub const ASSOC_ANNOT: &str = "assoc";
pub const SIDE_ARG: &str = "side";

/// Associativity of an alternative.
///
/// An alternative may have zero or more recursive occurrence of the current rule. Take for example
/// the common ternary conditional operator `x ? y : z`:
/// ```
/// #precedence(level="3")
/// <left: Expression> "?" <middle: Expression> : <right: Expression> => ..
/// ```
/// ## Left
///
/// Left associativity means that the construction may be iterated on the left. In this case, `x ? y : z ? foo
/// : bar` is parsed as `(x ? y : z) ? foo : bar`. When such associativity is selected, the
/// expander replaces the first recursive occurrence of `Expression` by the current level, and all
/// others by the previous level:
///
/// ```
/// <left: Expression3> "?" <middle: Expression2> : <right: Expression2> => ..
/// ```
///
/// ## Right
///
/// Right associativity means that the construction may be iterated on the right. In this case, `x ? y : z ? foo
/// : bar` is parsed as `x ? y : (z ? foo : bar)`. When such associativity is selected, the
/// expander replaces the last recursive occurrence  of `Expression` by the current level, and all
/// others by the previous level:
///
/// ```
/// <left: Expression2> "?" <middle: Expression2> : <right: Expression3> => ..
/// ```
///
/// ## None
///
/// Non-associativity means that it is not legal to iterate the rule, turning our example to
/// a parsing error. In this case, all recursive occurrences of the current rule are replaced with
/// the rule corresponding to the previous level:
///
/// ```
/// <left: Expression2> "?" <middle: Expression2> : <right: Expression3> => ..
/// ```
///
/// ## Default
///
/// By default, if no associativity is provided, all recursive occurrences are replaced with the
/// current level, which is different from non-associativity. This can be useful for unary
/// operators that may be iterated, such as `-` or `!`.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Assoc {
    Left,
    Right,
    NonAssoc,
}

/// Substitution plan.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Substitution<'a> {
    /// Replace the first encountered occurrence by the first argument, and all the following by
    /// the second. Used for associativity: typically, a left associativity on level `3` perform a
    /// `OneThen(Rule3, Rule2)`.
    OneThen(&'a SymbolKind, &'a SymbolKind),
    /// Standard substitution mode. Replace every encountered occurrence with the same given
    /// symbol.
    Every(&'a SymbolKind),
}

/// Direction for substitution.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAssocError {
    _priv: (),
}

impl fmt::Display for ParseAssocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided value was neither `left`, `right` nor `none`".fmt(f)
    }
}

impl FromStr for Assoc {
    type Err = ParseAssocError;

    fn from_str(s: &str) -> Result<Assoc, ParseAssocError> {
        match s {
            "left" => Ok(Assoc::Left),
            "right" => Ok(Assoc::Right),
            "none" => Ok(Assoc::NonAssoc),
            _ => Err(ParseAssocError { _priv: () }),
        }
    }
}

/// Perform precedence expansion. Rewrite rules where at least one alternative have a precedence
/// annotation, and generate derived rules for each level of precedence.
pub fn expand_precedence(input: Grammar) -> NormResult<Grammar> {
    let input = resolve::resolve(input)?;
    let mut result: Vec<GrammarItem> = Vec::with_capacity(input.items.len());

    for item in input.items.into_iter() {
        match item {
            GrammarItem::Nonterminal(d) if has_prec_annot(&d) => result.extend(expand_nonterm(d)?),
            item => result.push(item),
        };
    }

    Ok(Grammar {
        items: result,
        ..input
    })
}

/// Determine if an alternative has a precedence annotation.
pub fn has_prec_annot(non_term: &NonterminalData) -> bool {
    // After prevalidation, either each or none of the alternative of a nonterminal have precedence
    // annotations, so we just have to check the first one.
    non_term
        .alternatives
        .first()
        .map(|alt| {
            alt.annotations
                .iter()
                .any(|ann| ann.id == Atom::from(PREC_ANNOT) || ann.id == Atom::from(ASSOC_ANNOT))
        })
        .unwrap_or(false)
}

/// Expand a rule with precedence annotations. As it implies to generate new rules, return a vector
/// of grammar items.
fn expand_nonterm(mut nonterm: NonterminalData) -> NormResult<Vec<GrammarItem>> {
    let alt_with_ann = Vec::with_capacity(nonterm.alternatives.len());

    let (mut lvls, alts_with_ann) = nonterm.alternatives.drain(..).fold(
        (vec![], alt_with_ann),
        |(mut lvls, mut acc): (Vec<u32>, Vec<(u32, Option<Assoc>, Alternative)>), mut alt| {
            // All the following unsafe `unwrap()`, `panic!()`, etc. should never panic thanks to
            // prevalidation. Prevalidation must ensure that each alternative is annotated with at
            // least a precedence level, each precedence annotation must have an argument which
            // is parsable as an integer, and each optional assoc annotation must have an argument
            // that is either "right", "left" or "none".

            // Extract and remove precedence and associativity annotations
            let lvl: u32 = {
                let index = alt
                    .annotations
                    .iter()
                    .position(|ann| ann.id == Atom::from(PREC_ANNOT))
                    .unwrap();
                let (_, val) = alt.annotations.remove(index).arg.unwrap();
                val.parse().unwrap()
            };
            let assoc: Option<Assoc> = alt
                .annotations
                .iter()
                .position(|ann| ann.id == Atom::from(ASSOC_ANNOT))
                .map(|index| {
                    let (_, val) = alt.annotations.remove(index).arg.unwrap();
                    val.parse().unwrap()
                });

            acc.push((lvl, assoc, alt));
            lvls.push(lvl);
            (lvls, acc)
        },
    );

    lvls.sort();
    lvls.dedup();

    let rest = &mut alts_with_ann.into_iter();

    let lvl_max = *lvls.last().unwrap();
    // Iterate on pairs (lvls[i], lvls[i+1])
    let result = Some(None)
        .into_iter()
        .chain(lvls.iter().map(Some))
        .zip(lvls.iter())
        .map(|(lvl_prec_opt, lvl)| {
            // The generated non terminal corresponding to the last level keeps the same name as the
            // initial item, so that all external references to it are still valid. Other levels get
            // the names `Name1`, `Name2`, etc. where `Name` is the name of the initial item.
            let name = NonterminalString(Atom::from(if *lvl == lvl_max {
                format!("{}", nonterm.name)
            } else {
                format!("{}{}", nonterm.name, lvl)
            }));

            let nonterm_prev = lvl_prec_opt.map(|lvl_prec| {
                SymbolKind::Nonterminal(NonterminalString(Atom::from(format!(
                    "{}{}",
                    nonterm.name, lvl_prec
                ))))
            });

            let (alts_with_prec, new_rest): (Vec<_>, Vec<_>) =
                rest.partition(|(l, _, _)| *l == *lvl);
            *rest = new_rest.into_iter();

            let mut alts_with_assoc: Vec<_> = alts_with_prec
                .into_iter()
                .map(|(_, assoc, alt)| (assoc, alt))
                .collect();

            let symbol_kind = &SymbolKind::Nonterminal(name.clone());
            for (assoc, alt) in &mut alts_with_assoc {
                let err_msg = "unexpected associativity annotation on the first precedence level";
                let (subst, dir) = match assoc {
                    Some(Assoc::Left) => (
                        Substitution::OneThen(symbol_kind, &nonterm_prev.as_ref().expect(err_msg)),
                        Direction::Forward,
                    ),
                    Some(Assoc::Right) => (
                        Substitution::OneThen(symbol_kind, &nonterm_prev.as_ref().expect(err_msg)),
                        Direction::Backward,
                    ),
                    Some(Assoc::NonAssoc) => (
                        Substitution::Every(&nonterm_prev.as_ref().expect(err_msg)),
                        Direction::Forward,
                    ),
                    None => (Substitution::Every(symbol_kind), Direction::Forward),
                };
                replace_nonterm(alt, &nonterm.name, subst, dir)
            }

            let mut alternatives: Vec<_> =
                alts_with_assoc.into_iter().map(|(_, alt)| alt).collect();

            // Include the previous level
            if let Some(kind) = nonterm_prev {
                alternatives.push(Alternative {
                    // Don't really know what span should we put here
                    span: nonterm.span,
                    expr: ExprSymbol {
                        symbols: vec![Symbol {
                            kind,
                            span: nonterm.span,
                        }],
                    },
                    condition: None,
                    action: None,
                    annotations: vec![],
                });
            }

            GrammarItem::Nonterminal(NonterminalData {
                visibility: nonterm.visibility.clone(),
                name,
                annotations: nonterm.annotations.clone(),
                span: nonterm.span,
                args: nonterm.args.clone(), // macro arguments
                type_decl: nonterm.type_decl.clone(),
                alternatives,
            })
        });

    let items = result.collect();
    assert!(rest.next().is_none());
    Ok(items)
}

/// Perform substitution of on an non-terminal in an alternative.
fn replace_nonterm(
    alt: &mut Alternative,
    target: &NonterminalString,
    subst: Substitution,
    dir: Direction,
) {
    replace_symbols(&mut alt.expr.symbols, target, subst, dir);
}

/// Perform substitution of on an non-terminal in an array of symbols.
fn replace_symbols<'a>(
    symbols: &mut [Symbol],
    target: &NonterminalString,
    subst: Substitution<'a>,
    dir: Direction,
) -> Substitution<'a> {
    match dir {
        Direction::Forward => symbols.iter_mut().fold(subst, |subst, symbol| {
            replace_symbol(symbol, target, subst, dir)
        }),
        Direction::Backward => symbols.iter_mut().rev().fold(subst, |subst, symbol| {
            replace_symbol(symbol, target, subst, dir)
        }),
    }
}

/// Perform substitution of on an non-terminal in a symbol.
fn replace_symbol<'a>(
    symbol: &mut Symbol,
    target: &NonterminalString,
    subst: Substitution<'a>,
    dir: Direction,
) -> Substitution<'a> {
    match symbol.kind {
        SymbolKind::AmbiguousId(ref id) => {
            panic!("ambiguous id `{}` encountered after name resolution", id)
        }
        SymbolKind::Nonterminal(ref name) if name == target => match subst {
            Substitution::Every(sym_kind) => {
                symbol.kind = sym_kind.clone();
                subst
            }
            Substitution::OneThen(fst, snd) => {
                symbol.kind = fst.clone();
                Substitution::Every(snd)
            }
        },
        SymbolKind::Macro(ref mut m) => {
            if dir == Direction::Forward {
                m.args
                    .iter_mut()
                    .fold(subst, |subst, sym| replace_symbol(sym, target, subst, dir))
            } else {
                m.args
                    .iter_mut()
                    .rev()
                    .fold(subst, |subst, sym| replace_symbol(sym, target, subst, dir))
            }
        }
        SymbolKind::Expr(ref mut expr) => replace_symbols(&mut expr.symbols, target, subst, dir),
        SymbolKind::Repeat(ref mut repeat) => {
            replace_symbol(&mut repeat.symbol, target, subst, dir)
        }
        SymbolKind::Choose(ref mut sym) | SymbolKind::Name(_, ref mut sym) => {
            replace_symbol(sym, target, subst, dir)
        }
        SymbolKind::Terminal(_)
        | SymbolKind::Nonterminal(_)
        | SymbolKind::Error
        | SymbolKind::Lookahead
        | SymbolKind::Lookbehind => subst,
    }
}
