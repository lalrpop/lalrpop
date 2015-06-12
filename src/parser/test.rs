use super::*;

#[test]
fn type_name() {
    let x = parse_type_name("parser::Enum<'l,T>");
    assert_eq!(x.reference(), "::parser::Enum<'l, T>");
}
