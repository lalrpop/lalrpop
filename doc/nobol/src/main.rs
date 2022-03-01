#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub nobol1); // syntesized by LALRPOP

#[derive(PartialEq, Debug)]
pub struct Var(char);

#[derive(PartialEq, Debug)]
pub struct Lit(String);

#[derive(PartialEq, Debug)]
pub struct Stmt(Var, Lit);

impl From<char> for Var { fn from(c: char) -> Self { Var(c) } }
impl From<&str> for Lit { fn from(s: &str) -> Self { Lit(s.into()) } }
impl From<String> for Lit { fn from(s: String) -> Self { Lit(s.into()) } }
impl From<(Var, Lit)> for Stmt { fn from((v, l): (Var, Lit)) -> Self { Stmt(v, l) } }
impl From<(char, &str)> for Stmt { fn from((v, l): (char, &str)) -> Self { Stmt(v.into(), l.into()) } }

#[test]
fn nobol1() {
    assert_eq!(nobol1::VarParser::new().parse("x"), Ok('x'.into()));
    assert_eq!(nobol1::LitParser::new().parse(r#""abc""#), Ok("abc".into()));
    assert_eq!(nobol1::StmtParser::new().parse(r#"x = "a""#), Ok(('x', "a").into()));
    assert_eq!(nobol1::StmtParser::new().parse(r#"y = "bc""#), Ok(('y', "bc").into()));
}

#[test]
#[should_panic]
fn no_ball() {
    assert_eq!(nobol1::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
}

lalrpop_mod!(pub nobol2); // syntesized by LALRPOP

#[test]
fn fair_ball() {
    assert_eq!(nobol2::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
}

#[test]
#[should_panic]
fn foul_ball() {
    assert_eq!(nobol2::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
}

lalrpop_mod!(pub nobol3); // syntesized by LALRPOP

#[test]
fn bunt_ball() {
    assert_eq!(nobol3::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
}

lalrpop_mod!(pub nobol4); // syntesized by LALRPOP

#[test]
#[should_panic]
fn spaceballs() {
    assert_eq!(nobol4::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
}

lalrpop_mod!(pub nobol5); // syntesized by LALRPOP

#[test]
fn homerun() {
    assert_eq!(nobol5::VarParser::new().parse("x"), Ok('x'.into()));
    assert_eq!(nobol5::LitParser::new().parse(r#""abc""#), Ok("abc".into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"x = "a""#), Ok(('x', "a").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"y = "bc""#), Ok(('y', "bc").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
    assert_eq!(nobol5::StmtParser::new().parse(r#"z = "x y z""#), Ok(('z', "x y z").into()));
}

lalrpop_mod!(pub nobol6); // syntesized by LALRPOP

#[test]
fn nobol6() {
    assert_eq!(nobol6::VarParser::new().parse("x"), Ok('x'.into()));
    assert_eq!(nobol6::LitParser::new().parse(r#""abc""#), Ok("abc".into()));
    assert_eq!(nobol6::StmtParser::new().parse(r#"x = "a""#), Ok(('x', "a").into()));
    assert_eq!(nobol6::StmtParser::new().parse(r#"y = "bc""#), Ok(('y', "bc").into()));
    assert_eq!(nobol6::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
    assert_eq!(nobol6::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
    assert_eq!(nobol6::StmtParser::new().parse(r#"z = "x y z""#), Ok(('z', "x y z").into()));
}

#[test]
#[should_panic]
fn popfly() {
    assert_eq!(nobol6::StmtParser::new().parse(r#"z = "\"\\""#), Ok(('z', "\"\\").into()));
}

#[test]
fn flyball() {
    assert_eq!(nobol6::StmtParser::new().parse(r#"z = "\"\\""#), Ok(('z', "\\\"\\\\").into()));
}

lalrpop_mod!(pub nobol7); // syntesized by LALRPOP

#[test]
fn nobol7() {
    assert_eq!(nobol7::VarParser::new().parse("x"), Ok('x'.into()));
    assert_eq!(nobol7::LitParser::new().parse(r#""abc""#), Ok("abc".into()));
    assert_eq!(nobol7::StmtParser::new().parse(r#"x = "a""#), Ok(('x', "a").into()));
    assert_eq!(nobol7::StmtParser::new().parse(r#"y = "bc""#), Ok(('y', "bc").into()));
    assert_eq!(nobol7::StmtParser::new().parse(r#"z = "xyz""#), Ok(('z', "xyz").into()));
    assert_eq!(nobol7::StmtParser::new().parse(r#"z = "x""#), Ok(('z', "x").into()));
    assert_eq!(nobol7::StmtParser::new().parse(r#"z = "x y z""#), Ok(('z', "x y z").into()));
}

#[test]
fn grandslam() {
    assert_eq!(nobol7::StmtParser::new().parse(r#"z = "\"\\""#), Ok(('z', "\"\\").into()));
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
