use intern::intern;
use grammar::TypeName;
use grammar::token::*;

fn test_token_defn() -> TokenDefinition {
    TokenDefinition::new(TypeName::new(vec![intern("parser")],
                                       intern("Token"),
                                       vec![intern("'input")]),
                         vec![(intern("("), intern("LPAREN")),
                              (intern("R"), intern("LPAREN"))])
}

#[test]
fn test_match_pattern() {
    let defn = test_token_defn();
    assert_eq!(defn.match_pattern(intern("(")), "::parser::Token::LPAREN(..)");
}
