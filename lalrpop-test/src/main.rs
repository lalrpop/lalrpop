extern crate lalrpop_util;

use lalrpop_util::ParseError;

/// demonstration from the Greene text; one of the simplest grammars
/// that still ensures we get parse tree correct
mod sub;

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

/// test that exercises internal token generation, as well as locations and spans
mod intern_tok;

/// test that exercises using a lifetime parameter in the token type
mod lifetime_tok;

/// library for lifetime_tok test
mod lifetime_tok_lib;

/// test that exercises locations and spans
mod loc;

/// test that uses `super` in paths in various places
mod use_super;

/// test that exercises locations and spans
mod error;

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
    use lalrpop_util::ParseError;
    match util::test_err_gen(error::parse_Items, "---+") {
        Err(ParseError::User { error: '+' }) => { /* OK! */ }
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }
}

