macro_rules! lalrpop_mod_doc {
    ($vis:vis $name:ident) => {
        lalrpop_util::lalrpop_mod!(
            #[allow(clippy::ptr_arg)]
            #[allow(clippy::vec_box)]
            $vis $name);
    }
}

lalrpop_mod_doc!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

lalrpop_mod_doc!(pub calculator2);

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator2::TermParser::new().parse("((22)").is_err());
}

lalrpop_mod_doc!(pub calculator2b);

#[test]
fn calculator2b() {
    let result = calculator2b::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator2b::TermParser::new().parse("foo33").unwrap();
    assert_eq!(result, "Id(foo33)");

    let result = calculator2b::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");

    let result = calculator2b::TermParser::new().parse("(222)").unwrap();
    assert_eq!(result, "222");
}

lalrpop_mod_doc!(pub calculator3);

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(
            calculator3::ExprParser::new()
                .parse(stringify!($expr))
                .unwrap(),
            $expr
        );
    };
}

#[test]
fn calculator3() {
    test3!(22 + 44);
    test3!(22 - 44 - 66);
    test3!(22 * 44 + 66);
    test3!(22 * 44 + 66 / 3);
    test3!(22 * (44 + 66) / 3);
}

lalrpop_mod_doc!(pub calculator4);
mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "((22 * 44) + 66)");
}

lalrpop_mod_doc!(pub calculator5);

#[test]
fn calculator5() {
    let expr = calculator5::ExprsParser::new().parse("").unwrap();
    assert_eq!(&format!("{expr:?}"), "[]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66,")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[((22 * 44) + 66), (13 * 3)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3,")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[((22 * 44) + 66), (13 * 3)]");
}

lalrpop_mod_doc!(pub calculator6);

#[test]
fn calculator6() {
    // Number is one bigger than std::i32::MAX
    let expr = calculator6::ExprsParser::new().parse("2147483648");
    assert!(expr.is_err());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Calculator6Error {
    InputTooBig,
    OddNumber,
}

lalrpop_mod_doc!(pub calculator6b);

#[test]
fn calculator6b() {
    use lalrpop_util::ParseError;

    let expr = calculator6b::ExprsParser::new().parse("2147483648");
    assert!(expr.is_err());
    assert_eq!(
        expr.unwrap_err(),
        ParseError::User {
            error: Calculator6Error::InputTooBig
        }
    );

    let expr = calculator6b::ExprsParser::new().parse("3");
    assert!(expr.is_err());
    assert_eq!(
        expr.unwrap_err(),
        ParseError::User {
            error: Calculator6Error::OddNumber
        }
    );
}

lalrpop_mod_doc!(pub calculator7);

#[test]
fn calculator7() {
    let mut errors = Vec::new();

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[((22 * error) + 3)]");

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "22 * 44 + 66, *3")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[((22 * 44) + 66), (error * 3)]");

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "*")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

lalrpop_mod_doc!(pub calculator8);

#[test]
fn calculator8() {
    let scale = 2;
    let expr = calculator8::ExprParser::new()
        .parse(scale, "11 * 22 + 33")
        .unwrap();
    assert_eq!(&format!("{expr:?}"), "((22 * 44) + 66)");
}

lalrpop_mod_doc!(pub calculator9);
pub mod tok9;

#[test]
fn calculator9() {
    let input = "22 * pi + 66";
    let lexer = crate::tok9::Lexer::new(input);
    let expr = calculator9::ExprParser::new().parse(input, lexer).unwrap();
    assert_eq!(&format!("{expr:?}"), "((\"22\" * \"pi\") + \"66\")");
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
