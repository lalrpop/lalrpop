//! Precedence expander.
use crate::grammar::parse_tree::{
    Alternative, ExprSymbol, Grammar, GrammarItem, NonterminalData, NonterminalString,
    Symbol, SymbolKind,
};
use super::resolve;
use super::NormResult;
use std::mem;
use std::fmt;
use std::str::FromStr;
use string_cache::DefaultAtom as Atom;

#[cfg(test)]
mod test;

pub const PREC_ANNOT: &str = "precedence";
pub const LVL_ARG: &str = "level";
pub const ASSOC_ANNOT: &str = "assoc";
pub const SIDE_ARG: &str = "side";

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Assoc {
    Left,
    Right,
    NonAssoc,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Substitution<'a> {
    OneThen(&'a SymbolKind, &'a SymbolKind),
    Every(&'a SymbolKind),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Forward,
    Backward
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAssocError {
    _priv: (),
}

impl fmt::Display for ParseAssocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided value was neither `right` nor `left`".fmt(f)
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

pub fn expand_precedence(input: Grammar) -> NormResult<Grammar> {
    let input = resolve::resolve(input)?;

    let items = input.items;

    // Filter non terminals that have at least one precedence annotation
    let (non_terms_prec, items): (Vec<_>, Vec<_>) = items
        .into_iter()
        .partition(|input| match input {
            GrammarItem::Nonterminal(d) => has_prec_annot(d),
            _ => false
        });

    let nonterms_prec: NormResult<Vec<Vec<GrammarItem>>> = non_terms_prec.into_iter().map(|item| match item {
        GrammarItem::Nonterminal(d) => expand_nonterm(d),
        _ => panic!()
    }).collect();
    let mut result: Vec<GrammarItem> = nonterms_prec?.into_iter().flatten().collect();
    result.extend(items);

    Ok(Grammar { items: result, ..input })
}

pub fn has_prec_annot(non_term: &NonterminalData) -> bool {
    // After prevalidation, either each or none of the alternative of a nonterminal have precedence
    // annotations, so we just have to check the first one.
    non_term.alternatives.first().map(|alt|
        alt.annotations.iter().any(|ann|
            ann.id == Atom::from(PREC_ANNOT)
            || ann.id == Atom::from(ASSOC_ANNOT)
        )
    )
    .unwrap_or(false)
}

fn expand_nonterm(mut nonterm: NonterminalData) -> NormResult<Vec<GrammarItem>> {
    let alt_with_ann = Vec::with_capacity(nonterm.alternatives.len());

    let (mut lvls, alts_with_ann) = mem::replace(&mut nonterm.alternatives, vec![]).into_iter().fold(
        (vec![], alt_with_ann),
        |(mut lvls, mut acc): (Vec<u32>, Vec<(u32, Option<Assoc>, Alternative)>), mut alt| {
            // All the following unsafe `unwrap()`, `panic!()`, etc. should never panic thanks to
            // prevalidation. Prevalidation must ensure that each alternative is annotated with at
            // least a precedence level, each precedence annotation must have an argument which
            // is parsable as an integer, and each optional assoc annotation must have an argument
            // that is either "right" or "left".

            // Extract and remove precedence and associativity annotations
            let lvl: u32 = {
                let index = alt.annotations.iter().position(|ann| ann.id == Atom::from(PREC_ANNOT)).unwrap();
                let (_, val) = alt.annotations.remove(index).arg.unwrap();
                val.parse().unwrap()
            };
            let assoc: Option<Assoc> = alt.annotations.iter().position(|ann| ann.id == Atom::from(ASSOC_ANNOT)).map(|index| {
                let (_, val) = alt.annotations.remove(index).arg.unwrap();
                val.parse().unwrap()
            });

            acc.push((lvl, assoc, alt));
            lvls.push(lvl);
            (lvls, acc)
        });

    lvls.sort();
    lvls.dedup();

    let rest = &mut alts_with_ann.into_iter();

    let lvl_max = *lvls.last().unwrap();
    let result = lvls.into_iter().map(|lvl| {
        // The generated non terminal corresponding to the last level keeps the same name as the
        // initial item, so that all external references to it are still valid. Other levels get
        // the names `Name1`, `Name2`, etc. where `Name` is the name of the initial item.
        let name = NonterminalString(Atom::from(
            if lvl == lvl_max {
                format!("{}", nonterm.name)
            }
            else {
                format!("{}{}", nonterm.name, lvl)
            }
        ));

        let nonterm_prev = if lvl > 1 {
                Some(SymbolKind::Nonterminal(NonterminalString(Atom::from(
                format!("{}{}", nonterm.name, lvl - 1)))))
            }
            else {
                None
            };

        let (alts_with_prec, new_rest) : (Vec<_>, Vec<_>) = rest.partition(|(l, _, _)| *l == lvl);
        *rest = new_rest.into_iter();

        let mut alts_with_assoc: Vec<_> = alts_with_prec.into_iter().map(|(_, assoc, alt)| (assoc, alt)).collect();

        let symbol_kind = &SymbolKind::Nonterminal(name.clone());
        for (assoc, alt) in &mut alts_with_assoc {
            let err_msg = "unexpected associativity annotation on the first precedence level";
            let (subst,dir) = match assoc {
                Some(Assoc::Left) => {
                    (Substitution::OneThen(
                        symbol_kind,
                        &nonterm_prev.as_ref().expect(err_msg),
                    ), Direction::Forward)
                }
                Some(Assoc::Right) => {
                    (Substitution::OneThen(
                        symbol_kind,
                        &nonterm_prev.as_ref().expect(err_msg),
                    ), Direction::Backward)
                }
                Some(Assoc::NonAssoc) => (Substitution::Every(&nonterm_prev.as_ref().expect(err_msg)), Direction::Forward),
                None => (Substitution::Every(symbol_kind), Direction::Forward),
            };
            replace_nonterm(alt, &nonterm.name, subst, dir)
        }

        let mut alternatives: Vec<_> = alts_with_assoc.into_iter().map(|(_, alt)| alt).collect();

        // Include the previous level
        if let Some(kind) = nonterm_prev {
            alternatives.push(Alternative {
                // Don't really know what span should we put here
                span: nonterm.span,
                expr: ExprSymbol { symbols: vec![Symbol {kind, span: nonterm.span} ] },
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

fn replace_nonterm(alt: &mut Alternative, target: &NonterminalString, subst: Substitution, dir: Direction) {
   replace_symbols(&mut alt.expr.symbols, target, subst, dir);
}

fn replace_symbols<'a>(symbols: &mut [Symbol], target: &NonterminalString, subst: Substitution<'a>, dir: Direction) -> Substitution<'a> {
    match dir {
        Direction::Forward =>
            symbols.iter_mut().fold(subst, |subst, symbol| replace_symbol(symbol, target, subst, dir)),
        Direction::Backward =>
            symbols.iter_mut().rev().fold(subst, |subst, symbol| replace_symbol(symbol, target, subst, dir)),
    }
}

fn replace_symbol<'a>(symbol: &mut Symbol, target: &NonterminalString, subst: Substitution<'a>, dir: Direction) -> Substitution<'a> {
    match symbol.kind {
        SymbolKind::AmbiguousId(ref id) => {
            panic!("ambiguous id `{}` encountered after name resolution", id)
        }
        SymbolKind::Nonterminal(ref name) if name == target => {
            match subst {
                Substitution::Every(sym_kind) => {
                    mem::replace(&mut symbol.kind, sym_kind.clone());
                    subst
                },
                Substitution::OneThen(fst, snd) => {
                    mem::replace(&mut symbol.kind, fst.clone());
                    Substitution::Every(snd)
                },
            }
        }
        SymbolKind::Macro(ref mut m) => {
            if dir == Direction::Forward {
                m.args.iter_mut().fold(subst, |subst, sym| replace_symbol(sym, target, subst, dir))
            }
            else {
                m.args.iter_mut().rev().fold(subst, |subst, sym| replace_symbol(sym, target, subst, dir))
            }
        }
        SymbolKind::Expr(ref mut expr) => {
            replace_symbols(&mut expr.symbols, target, subst, dir)
        }
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
        | SymbolKind::Lookbehind => {
            subst
        }
    }
}
