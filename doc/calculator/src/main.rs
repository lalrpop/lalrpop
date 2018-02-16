extern crate lalrpop_util;

pub mod calculator1; // syntesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

pub mod calculator2;

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator2::TermParser::new().parse("((22)").is_err());
}

pub mod calculator2b;

#[test]
fn calculator2b() {
    assert_eq!(calculator2b::TermParser::new().parse("33").unwrap(), "33");
    assert_eq!(calculator2b::TermParser::new().parse("foo33").unwrap(), "Id(foo33)");
    assert_eq!(calculator2b::TermParser::new().parse("(22)").unwrap(), "Twenty-two!");
    assert_eq!(calculator2b::TermParser::new().parse("(222)").unwrap(), "222");
}

pub mod calculator3;

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(calculator3::ExprParser::new().parse(stringify!($expr)).unwrap(), $expr);
    }
}

#[test]
fn calculator3() {
    test3!(22 + 44);
    test3!(22 - 44 - 66);
    test3!(22 * 44 + 66);
    test3!(22 * 44 + 66 / 3);
    test3!(22 * (44 + 66) / 3);
}

pub mod calculator4;
pub mod ast;

#[test]
fn calculator4() {
    assert_eq!(&format!("{:?}", calculator4::ExprParser::new().parse("22 * 44 + 66").unwrap()),
               "((22 * 44) + 66)");
}

pub mod calculator5;

#[test]
fn calculator5() {
    assert_eq!(&format!("{:?}", calculator5::ExprsParser::new().parse("").unwrap()),
               "[]");
    assert_eq!(&format!("{:?}", calculator5::ExprsParser::new().parse("22 * 44 + 66").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator5::ExprsParser::new().parse("22 * 44 + 66,").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator5::ExprsParser::new().parse("22 * 44 + 66, 13*3").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", calculator5::ExprsParser::new().parse("22 * 44 + 66, 13*3,").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
}

pub mod calculator6;

#[test]
fn calculator6() {
    let mut errors = Vec::new();
    assert_eq!(&format!("{:?}", calculator6::ExprsParser::new().parse(&mut errors, "22 * + 3").unwrap()),
               "[((22 * error) + 3)]");
    assert_eq!(&format!("{:?}", calculator6::ExprsParser::new().parse(&mut errors, "22 * 44 + 66, *3").unwrap()),
               "[((22 * 44) + 66), (error * 3)]");
    assert_eq!(&format!("{:?}", calculator6::ExprsParser::new().parse(&mut errors, "*").unwrap()),
               "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
