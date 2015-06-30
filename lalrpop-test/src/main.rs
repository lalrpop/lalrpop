mod expr;
mod sub;
mod util;

use expr::parse_Expr;

fn main() {
    println!("Hello, world!");
}

#[test]
fn expr_test1() {
    util::test(parse_Expr, "22 - 3", 22 - 3);
}

#[test]
fn expr_test2() {
    util::test(parse_Expr, "22 - (3 + 5)", 22 - (3 + 5));
}

#[test]
fn expr_test3() {
    util::test(parse_Expr, "22 - (3 - 5) - 13", 22 - (3 - 5) - 13);
}

#[test]
fn expr_test4() {
    util::test(parse_Expr, "22 * 3 - 6", 22 * 3 - 6);
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
