use std::collections::{HashMap, HashSet};
use intern::{intern, read, InternedString};
use grammar::parse_tree::{Alternative, Condition, ConditionOp, ExprSymbol, Grammar, GrammarItem,
                          MacroSymbol, NonterminalData, RepeatSymbol, Span, Symbol, TypeRef};
use normalize::{NormResult, NormError};
use regex::Regex;
use std::mem;

pub fn infer_types(mut grammar: Grammar) -> NormResult<Grammar> {
    {
        let mut inferencer = TypeInferencer::new(&mut grammar.items);
        try!(inferencer.infer_types());
    }
    Ok(grammar)
}

struct TypeInferencer<'a> {
    stack: Vec<InternedString>,
    nonterminals: HashMap<InternedString, NT<'a>>,
}

struct NT<'a> {
    type_decl: &'a mut Option<TypeRef>,
    alternatives: &'a Vec<Alternative>,
}

impl<'a> TypeInferencer<'a> {
    fn new(items: &'a mut [GrammarItem]) -> TypeInferencer<'a> {
        let nonterminals =
            items.into_iter()
                 .filter_map(|item| {
                     match *item {
                         GrammarItem::TokenType(..) =>
                             None,
                         GrammarItem::Nonterminal(ref mut data) => {
                             assert!(!data.is_macro_def()); // normalized away by now
                             Some((data.name, NT::new(data)))
                         }
                     }
                 })
                 .collect();
        TypeInferencer { stack: vec![], nonterminals: nonterminals }
    }

    fn infer_types(&mut self) -> NormResult<()> {
        let ids: Vec<InternedString> =
            self.nonterminals.iter()
                             .filter(|&(_, nt)| nt.type_decl.is_none())
                             .map(|(&id, _)| id)
                             .collect();

        for id in ids {
            try!(self.infer_type_for(id));
        }

        Ok(())
    }

    fn infer_type_for(&mut self, id: InternedString) -> NormResult<()> {
        let alternatives;

        {
            let nt = &self.nonterminals[&id];
            if nt.type_decl.is_some() {
                return Ok(());
            }
            alternatives = nt.alternatives;
        }

        self.stack.push(id);

        let alternative_types: Vec<TypeRef> =
            try!(alternatives.iter()
                             .map(|alt| self.compute_expr_type(&alt.expr))
                             .collect());

        let ty0 = &alternative_types[0];
        for (tyN, altN) in alternative_types[1..].iter().zip(&alternatives[1..]) {
            if ty0 != tyN {
                return_err!(altN.expr.span,
                            "type of this alternative is `{}`, \
                             but type of first alternative is `{}`",
                            tyN, ty0);
            }
        }

        self.stack.pop().unwrap();

        Ok(())
    }

    fn compute_expr_type(&mut self, expr: &ExprSymbol) -> NormResult<TypeRef> {
        loop { }
    }
}

impl<'a> NT<'a> {
    fn new(data: &'a mut NonterminalData) -> NT<'a> {
        NT { type_decl: &mut data.type_decl, alternatives: &data.alternatives }
    }
}
