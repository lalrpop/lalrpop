use super::*;

#[test]
fn type_name() {
    let x = parse_type_name("parser::Enum<'l,T>");
    assert_eq!(x.reference(), "::parser::Enum<'l, T>");
}

#[test]
fn empty_grammar() {
    let x = parse_grammar(
        r#"grammar Foo { } "#);
    assert_eq!(
        format!("{:?}", x),
        "Grammar { type_name: TypeName { module: [], type_name: \"Foo\", parameters: [] }, items: [] }");
}

