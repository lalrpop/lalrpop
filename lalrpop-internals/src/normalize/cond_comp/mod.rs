//! Compute cfg directives.

use super::NormResult;
use crate::grammar::consts::CFG;
use crate::grammar::parse_tree::*;
use crate::session::Session;
use string_cache::DefaultAtom as Atom;

#[cfg(test)]
mod test;

pub fn remove_disabled_decls(session: &Session, mut grammar: Grammar) -> NormResult<Grammar> {
    grammar.items.retain_mut(|item| match item {
        GrammarItem::ExternToken(et) => match &mut et.enum_token {
            Some(EnumToken { conversions, .. }) => {
                conversions.retain_mut(|c| cfg_active(session, &c.attributes));
                true
            }
            None => true,
        },

        GrammarItem::Nonterminal(nt) => {
            let active = cfg_active(session, &nt.attributes);
            if active {
                nt.alternatives
                    .retain_mut(|prod| cfg_active(session, &prod.attributes));
            }
            active
        }
        _ => true,
    });
    Ok(grammar)
}

pub fn cfg_active(session: &Session, attrs: &[Attribute]) -> bool {
    fn test_feat_attr(attr: &Attribute, session: &Session) -> bool {
        match &attr.arg {
            AttributeArg::Paren(attrs) if attr.id == *"not" => attrs
                .first()
                .is_some_and(|attr| !test_feat_attr(attr, session)),
            AttributeArg::Paren(attrs) if attr.id == *"all" => {
                attrs.iter().all(|attr| test_feat_attr(attr, session))
            }
            AttributeArg::Paren(attrs) if attr.id == *"any" => {
                attrs.iter().any(|attr| test_feat_attr(attr, session))
            }
            AttributeArg::Equal(feature) if attr.id == *"feature" => session
                .features
                .as_ref()
                .is_some_and(|features| features.contains(feature)),
            _ => false,
        }
    }

    let cfg_atom = Atom::from(CFG);
    attrs
        .iter()
        .filter(|attr| attr.id == cfg_atom)
        .all(|attr| match &attr.arg {
            AttributeArg::Paren(attr) => attr
                .first()
                .is_some_and(|attr| test_feat_attr(attr, session)),
            _ => false,
        })
}
