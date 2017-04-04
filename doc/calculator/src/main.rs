extern crate lalrpop_util;

pub mod calculator1; // syntesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("22").is_ok());
    assert!(calculator1::parse_Term("(22)").is_ok());
    assert!(calculator1::parse_Term("((((22))))").is_ok());
    assert!(calculator1::parse_Term("((22)").is_err());
}

pub mod calculator2;

#[test]
fn calculator2() {
    assert!(calculator2::parse_Term("22").is_ok());
    assert!(calculator2::parse_Term("(22)").is_ok());
    assert!(calculator2::parse_Term("((((22))))").is_ok());
    assert!(calculator2::parse_Term("((22)").is_err());
}

pub mod calculator2b;

#[test]
fn calculator2b() {
    assert_eq!(calculator2b::parse_Term("33").unwrap(), "33");
    assert_eq!(calculator2b::parse_Term("foo33").unwrap(), "Id(foo33)");
    assert_eq!(calculator2b::parse_Term("(22)").unwrap(), "Twenty-two!");
    assert_eq!(calculator2b::parse_Term("(222)").unwrap(), "222");
}

pub mod calculator3;

macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(calculator3::parse_Expr(stringify!($expr)).unwrap(), $expr);
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
    assert_eq!(&format!("{:?}", calculator4::parse_Expr("22 * 44 + 66").unwrap()),
               "((22 * 44) + 66)");
}

pub mod calculator5;

#[test]
fn calculator5() {
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("").unwrap()),
               "[]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66,").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66, 13*3").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("22 * 44 + 66, 13*3,").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
}

pub mod calculator6;

#[test]
fn calculator6() {
    let mut errors = Vec::new();
    assert_eq!(&format!("{:?}", calculator6::parse_Exprs(&mut errors, "22 * + 3").unwrap()),
               "[((22 * error) + 3)]");
    assert_eq!(&format!("{:?}", calculator6::parse_Exprs(&mut errors, "22 * 44 + 66, *3").unwrap()),
               "[((22 * 44) + 66), (error * 3)]");
    assert_eq!(&format!("{:?}", calculator6::parse_Exprs(&mut errors, "*").unwrap()),
               "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
