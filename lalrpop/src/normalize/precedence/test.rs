use crate::parser;
use crate::test_util::compare;

use super::expand_precedence;
use super::resolve::resolve;

#[test]
fn multilevel() {
    let grammar = parser::parse_grammar(
        r#"
grammar;
    Expr: u32 = {
       #[precedence(level="1")]
       <left:Expr> "*" <right:Expr> => 0,
       #[precedence(level="1")]
       <left:Expr> "/" <right:Expr> => 0,
       #[precedence(level="2")]
       <left:Expr> "+" <right:Expr> => 0,
       #[precedence(level="2")]
       <left:Expr> "-" <right:Expr> => 0,
       #[precedence(level="3")]
       <left:Expr> "%" <right:Expr> => 0,
    }

    Ext: u32 = Expr;
"#,
    )
    .unwrap();

    let expected = parser::parse_grammar(
        r#"
grammar;
    Expr1: u32 = {
       <left:Expr1> "*" <right:Expr1> => 0,
       <left:Expr1> "/" <right:Expr1> => 0,
    }

    Expr2: u32 = {
       <left:Expr2> "+" <right:Expr2> => 0,
       <left:Expr2> "-" <right:Expr2> => 0,
       Expr1,
    }

    Expr: u32 = {
       <left:Expr> "%" <right:Expr> => 0,
       Expr2,
    }

    Ext: u32 = Expr;
"#,
    )
    .unwrap();

    compare(expand_precedence(grammar), resolve(expected));
}
