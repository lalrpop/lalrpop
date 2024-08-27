//! Compute cfg directives.

use super::NormResult;
use crate::grammar::consts::CFG;
use crate::grammar::parse_tree::*;
use crate::session::Session;
use string_cache::DefaultAtom as Atom;

#[cfg(test)]
mod test;

pub fn remove_disabled_decls(session: &Session, mut grammar: Grammar) -> NormResult<Grammar> {
    grammar.items.retain(|item| match item {
        GrammarItem::Nonterminal(nt) => cfg_active(session, nt),
        _ => true,
    });
    Ok(grammar)
}

fn cfg_active(session: &Session, nt: &NonterminalData) -> bool {
    fn test_feat_attr(attr: &Attribute, session: &Session) -> bool {
        if attr.id == *"not" {
            match &attr.arg {
                AttributeArg::Paren(attrs) => attrs
                    .first()
                    .map_or(false, |attr| !test_feat_attr(attr, session)),
                _ => false,
            }
        } else if attr.id == *"all" {
            match &attr.arg {
                AttributeArg::Paren(attrs) => {
                    attrs.iter().all(|attr| test_feat_attr(attr, session))
                }
                _ => false,
            }
        } else if attr.id == *"any" {
            match &attr.arg {
                AttributeArg::Paren(attrs) => {
                    attrs.iter().any(|attr| test_feat_attr(attr, session))
                }
                _ => false,
            }
        } else if attr.id == *"feature" {
            match &attr.arg {
                AttributeArg::Equal(feature) => session
                    .features
                    .as_ref()
                    .map_or(false, |features| features.contains(feature)),
                _ => false,
            }
        } else {
            false
        }
    }

    let cfg_atom = Atom::from(CFG);
    nt.attributes
        .iter()
        .filter(|attr| attr.id == cfg_atom)
        .all(|attr| match &attr.arg {
            AttributeArg::Paren(attr) => attr
                .first()
                .map_or(false, |attr| test_feat_attr(attr, session)),
            _ => false,
        })
}
