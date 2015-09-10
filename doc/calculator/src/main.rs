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

fn main() {
    println!("Hello, world!");
}
