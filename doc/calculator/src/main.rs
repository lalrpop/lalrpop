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

pub mod calculator7;

#[test]
fn calculator7() {
    assert_eq!(&format!("{:?}", calculator7::parse_Exprs("22 * + 3").unwrap()),
               "[((22 * error) + 3)]");
    assert_eq!(&format!("{:?}", calculator7::parse_Exprs("22 * 44 + 66, *3").unwrap()),
               "[((22 * 44) + 66), (error * 3)]");
    assert_eq!(&format!("{:?}", calculator7::parse_Exprs("*").unwrap()),
               "[error]");
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
