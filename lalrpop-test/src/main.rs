extern crate diff;
extern crate lalrpop_util;

use std::cell::RefCell;

use lalrpop_util::{ErrorRecovery, ParseError};

use util::tok::Tok;

/// demonstration from the Greene text; one of the simplest grammars
/// that still ensures we get parse tree correct
mod sub;

/// test something other than test-all
mod sub_ascent;
mod sub_table;

/// more interesting demonstration of parsing full expressions
mod expr;

/// more interesting demonstration of parsing full expressions, using LALR not LR
mod expr_lalr;

/// more interesting demonstration of parsing full expressions, using intern tok
mod expr_intern_tok;

/// test that passes in lifetime/type/formal parameters and threads
/// them through, building an AST from the result
mod expr_arena;

/// definitions of the AST
mod expr_arena_ast;

/// expr defined with a generic type `F`
mod expr_generic;

mod generics_issue_104;
mod generics_issue_104_lib;

/// test of inlining
mod inline;

/// test that exercises internal token generation, as well as locations and spans
mod intern_tok;

/// test that exercises using a lifetime parameter in the token type
mod lifetime_tok;

/// library for lifetime_tok test
mod lifetime_tok_lib;

/// test that exercises locations and spans
mod loc;

/// regression test for location issue #90
mod loc_issue_90;
mod loc_issue_90_lib;

/// test that uses `super` in paths in various places
mod use_super;

/// test that exercises locations and spans
mod error;

/// Test error recovery
mod error_recovery;

/// test for inlining expansion issue #55
mod issue_55;

/// test for unit action code
mod unit;

mod util;

/// This constant is here so that some of the generator parsers can
/// refer to it in order to test `super::` handling in action code.
const ZERO: i32 = 0;

#[test]
fn expr_test1() {
    util::test(|v| expr::parse_Expr(1, v), "22 - 3", 22 - 3);
}

#[test]
fn expr_test2() {
    util::test(|v| expr::parse_Expr(1, v), "22 - (3 + 5)", 22 - (3 + 5));
}

#[test]
fn expr_test3() {
    util::test(|v| expr::parse_Expr(1, v), "22 - (3 - 5) - 13", 22 - (3 - 5) - 13);
}

#[test]
fn expr_test4() {
    util::test(|v| expr::parse_Expr(1, v), "22 * 3 - 6", 22 * 3 - 6);
}

#[test]
fn expr_test5() {
    util::test(|v| expr::parse_Expr(11, v), "22 * 3 - 6", 22*11 * 3*11 - 6*11);
}

#[test]
fn expr_intern_tok_test1() {
    assert_eq!(expr_intern_tok::parse_Expr(1, "22 - 3").unwrap(), 22 - 3);
}

#[test]
fn expr_intern_tok_test2() {
    assert_eq!(expr_intern_tok::parse_Expr(1, "22 - (3 - 5) - 13").unwrap(), 22 - (3 - 5) - 13);
}

#[test]
fn expr_intern_tok_test_err() {
    match expr_intern_tok::parse_Expr(1, "22 - (3 - 5) - X") {
        //                                0123456789012345
        Err(ParseError::InvalidToken { location }) => {
            assert_eq!(location, 15);
        }
        r => {
            panic!("invalid result {:?}", r);
        }
    }
}

#[test]
fn expr_lifetime_tok1() {
    // the problem here was that we were improperly pruning the 'input from the
    let tokens = lifetime_tok_lib::lt_tokenize("x");
    let tree = lifetime_tok::parse_Expr(tokens).unwrap();
    assert_eq!(tree, vec!["x"]);
}

#[test]
fn expr_lalr_test1() {
    util::test(|v| expr_lalr::parse_Expr(1, v), "22 - 3", 22 - 3);
}

#[test]
fn expr_lalr_test2() {
    util::test(|v| expr_lalr::parse_Expr(1, v), "22 - (3 + 5)", 22 - (3 + 5));
}

#[test]
fn expr_lalr_test3() {
    util::test(|v| expr_lalr::parse_Expr(1, v), "22 - (3 - 5) - 13", 22 - (3 - 5) - 13);
}

#[test]
fn expr_lalr_test4() {
    util::test(|v| expr_lalr::parse_Expr(1, v), "22 * 3 - 6", 22 * 3 - 6);
}

#[test]
fn expr_lalr_test5() {
    util::test(|v| expr_lalr::parse_Expr(11, v), "22 * 3 - 6", 22*11 * 3*11 - 6*11);
}

#[test]
fn inline_test1() {
    assert_eq!(inline::parse_E("& L L").unwrap(), "& L L");
}

#[test]
fn sub_test1() {
    util::test(sub::parse_S, "22 - 3", 22 - 3);
}

#[test]
fn sub_test2() {
    util::test(sub::parse_S, "22 - (3 - 5)", 22 - (3 - 5));
}

#[test]
fn sub_test3() {
    util::test(sub::parse_S, "22 - (3 - 5) - 13", 22 - (3 - 5) - 13);
}

#[test]
fn sub_ascent_test1() {
    util::test(sub_ascent::parse_S, "22 - 3", 22 - 3);
}

#[test]
fn sub_table_test1() {
    util::test(sub_table::parse_S, "22 - 3", 22 - 3);
}

#[test]
fn expr_arena_test1() {
    use expr_arena_ast::*;
    let arena = Arena::new();
    let expected =
        arena.alloc(Node::Binary(Op::Sub,
                                 arena.alloc(Node::Binary(Op::Mul,
                                                          arena.alloc(Node::Value(22)),
                                                          arena.alloc(Node::Value(3)))),
                                 arena.alloc(Node::Value(6))));
    util::test_loc(|v| expr_arena::parse_Expr(&arena, v), "22 * 3 - 6", expected);
}

#[test]
fn expr_arena_test2() {
    use expr_arena_ast::*;
    let arena = Arena::new();
    let expected =
        arena.alloc(Node::Reduce(Op::Mul,
                                 vec![arena.alloc(Node::Value(22)),
                                      arena.alloc(Node::Value(3)),
                                      arena.alloc(Node::Value(6))]));;
    util::test_loc(|v| expr_arena::parse_Expr(&arena, v), "*(22, 3, 6)", expected);
    util::test_loc(|v| expr_arena::parse_Expr(&arena, v), "*(22, 3, 6,)", expected);
}

#[test]
fn expr_arena_test3() {
    use expr_arena_ast::*;
    let arena = Arena::new();
    let expected =
        arena.alloc(
            Node::Binary(Op::Mul,
                         arena.alloc(Node::Value(22)),
                         arena.alloc(Node::Paren(
                             arena.alloc(
                                 Node::Binary(Op::Sub,
                                              arena.alloc(Node::Value(3)),
                                              arena.alloc(Node::Value(6))))))));
    util::test_loc(|v| expr_arena::parse_Expr(&arena, v), "22 * (3 - 6)", expected);
}

#[test]
fn expr_generic_test1() {
    let expected: i32 = 22 * 3 - 6;
    let actual = expr_generic::parse_Expr::<i32>("22 * 3 - 6").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn intern_tok_test1() {
    let expected = vec![(0, 0), // spans of `+` characters, measured in bytes
                        (2, 3),
                        (4, 5),
                        (8, 9)];
    let actual = intern_tok::parse_Items("--+-+---+").unwrap();
    //                                    012345678
    assert_eq!(actual, expected);
}

#[test]
fn loc_test1() {
    let expected = vec![(0, 0), // note that tok.rs generates odd spans, measured in 2*chars
                        (4, 5),
                        (8, 9),
                        (16, 17)];
    util::test_loc(|v| loc::parse_Items(v), "--+-+---+", expected);
    //                                       000001111
    //                                       024680246
}

#[test]
fn loc_test2() {
    util::test_loc(|v| loc::parse_Items(v), "+", vec![(0, 0),
                                                      (0, 1)]);
}

#[test]
fn loc_empty() {
    // test what happens when `@L` and `@R` are invoked on an empty input
    util::test_loc(|v| loc::parse_Items(v), "", vec![(0, 0)]);
}

#[test]
fn use_super_test1() {
    util::test(|v| use_super::parse_S(v), "()", 0);
}

#[test]
fn error_test1() {
    match util::test_err_gen(error::parse_Items, "---+") {
        Err(ParseError::User { error: '+' }) => { /* OK! */ }
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }
}

#[test]
fn error_recovery_eof() {
    let errors = RefCell::new(vec![]);
    util::test(|v| error_recovery::parse_Item(&errors, v), "--", '!'.to_string());

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(errors.borrow()[0], ErrorRecovery {
        error: ParseError::UnrecognizedToken {
            token: None,
            expected: vec![],
        },
        dropped_tokens: vec![],
    });
}

#[test]
fn error_recovery_eof_without_recovery() {
    let errors = RefCell::new(vec![]);
    let tokens = util::tok::tokenize("-").into_iter().map(|t| t.1);
    let result = error_recovery::parse_Item(&errors, tokens);
    assert_eq!(result, Err(ParseError::UnrecognizedToken {
        token: None,
        expected: vec![],
    }));
}

#[test]
fn error_recovery_extra_token() {
    let errors = RefCell::new(vec![]);
    util::test(|v| error_recovery::parse_Item(&errors, v), "(++)", "()".to_string());

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(errors.borrow()[0], ErrorRecovery {
        error: ParseError::UnrecognizedToken {
            token: Some(((), Tok::Plus,())),
            expected: vec![],
        },
        dropped_tokens: vec![((), Tok::Plus, ())],
    });
}

#[test]
fn error_recovery_dont_drop_unrecognized_token() {
    let errors = RefCell::new(vec![]);
    util::test(|v| error_recovery::parse_Item(&errors, v), "(--)", "(!)".to_string());

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(errors.borrow()[0], ErrorRecovery {
        error: ParseError::UnrecognizedToken {
            token: Some(((), Tok::RParen,())),
            expected: vec![],
        },
        dropped_tokens: vec![],
    });
}

#[test]
fn error_recovery_multiple_extra_tokens() {
    let errors = RefCell::new(vec![]);
    util::test(|v| error_recovery::parse_Item(&errors, v), "(+++)", "()".to_string());

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(errors.borrow()[0], ErrorRecovery {
        error: ParseError::UnrecognizedToken {
            token: Some(((), Tok::Plus,())),
            expected: vec![],
        },
        dropped_tokens: vec![((), Tok::Plus, ()), ((), Tok::Plus, ())],
    });
}

#[test]
fn issue_55_test1() {
    // Issue 55 caused us to either accept NO assoc types or assoc
    // types both before and after, so check that we can parse with
    // assoc types on either side.

    let (a, b, c) = issue_55::parse_E("{ type X; type Y; enum Z { } }").unwrap();
    assert_eq!(a, vec!["X", "Y"]);
    assert_eq!(b, "Z");
    assert!(c.is_empty());

    let (a, b, c) = issue_55::parse_E("{ enum Z { } type X; type Y; }").unwrap();
    assert!(a.is_empty());
    assert_eq!(b, "Z");
    assert_eq!(c, vec!["X", "Y"]);
}

#[test]
fn unit_test1() {
    assert!(unit::parse_Expr("3 + 4 * 5").is_ok());
    assert!(unit::parse_Expr("3 + +").is_err());
}

#[test]
fn generics_issue_104_test1() {
    // The real thing `generics_issue_104` is testing is that the code
    // *compiles*, even though the type parameter `T` does not appear
    // in any of the arguments.
    assert!(generics_issue_104::parse_Schema::<()>("grammar { foo }").is_ok());
}
