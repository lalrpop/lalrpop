extern crate util;

mod expr;

use expr::parse_Expr;

#[cfg(not(test))]
fn main() {
    util::main(parse_Expr);
}

#[test]
fn test1() {
    util::test(parse_Expr, "22 - 3", 22 - 3);
}

#[test]
fn test2() {
    util::test(parse_Expr, "22 - (3 + 5)", 22 - (3 + 5));
}

#[test]
fn test3() {
    util::test(parse_Expr, "22 - (3 - 5) - 13", 22 - (3 - 5) - 13);
}

#[test]
fn test4() {
    util::test(parse_Expr, "22 * 3 - 6", 22 * 3 - 6);
}

