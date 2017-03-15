use super::{Tok, Tokenizer};
use super::Tok::*;

fn test(input: &str,
        expected: Vec<(&str, Tok)>)
{
    // use $ to signal EOL because it can be replaced with a single space
    // for spans, and because it applies also to r#XXX# style strings:
    let input = input.replace("$", "\n");

    let tokenizer = Tokenizer::new(&input, 0);
    let len = expected.len();
    for (token, (expected_span, expected_tok)) in tokenizer.zip(expected.into_iter()) {
        println!("token: {:?}", token);
        let expected_start = expected_span.find("~").unwrap();
        let expected_end = expected_span.rfind("~").unwrap() + 1;
        assert_eq!(Ok((expected_start, expected_tok, expected_end)), token);
    }

    let tokenizer = Tokenizer::new(&input, 0);
    assert_eq!(None, tokenizer.skip(len).next());
}

#[test]
fn basic() {
    test("extern foo", vec![
        ("~~~~~~    ", Extern),
        ("       ~~~", Id("foo")),
    ]);
}

#[test]
fn eol_comment() {
    test("extern // This is a comment$ foo", vec![
        ("~~~~~~                          ", Extern),
        ("                             ~~~", Id("foo")),
    ]);
}

#[test]
fn code1() {
    test("=> a(b, c),", vec![
        ("~~~~~~~~~~ ", EqualsGreaterThanCode(" a(b, c)")),
        ("          ~", Comma),
    ]);
}

#[test]
fn rule_id_then_equalsgreaterthancode_functioncall() {
    test("id => a(b, c),", vec![
        ("~~            ", Id("id")),
        ("   ~~~~~~~~~~ ", EqualsGreaterThanCode(" a(b, c)")),
        ("             ~", Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_dot_then_equalsgreaterthancode_functioncall() {
    test(r#" "\." => a(b, c),"#, vec![
        (r#" ~~~~            "#, StringLiteral(r#"\."#)),
        (r#"      ~~~~~~~~~~ "#, EqualsGreaterThanCode(" a(b, c)")),
        (r#"                ~"#, Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_dot_then_equalsgreaterthancode_many_characters_in_stringliteral() {
    test(r#" "\." => "Planet Earth" ,"#, vec![
        (r#" ~~~~                    "#, StringLiteral(r#"\."#)),
        (r#"      ~~~~~~~~~~~~~~~~~~ "#, EqualsGreaterThanCode(r#" "Planet Earth" "#)),
        (r#"                        ~"#, Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_dot_then_equalsgreaterthancode_one_character_dot_in_stringliteral() {
    test(r#" "\." => "." ,"#, vec![
        (r#" ~~~~         "#, StringLiteral(r#"\."#)),
        (r#"      ~~~~~~~ "#, EqualsGreaterThanCode(r#" "." "#)),
        (r#"             ~"#, Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_openningbracket_then_equalsgreaterthancode_one_character_openningbracket_in_stringliteral() {
    test(r#" "\(" => "(" ,"#, vec![
        (r#" ~~~~         "#, StringLiteral(r#"\("#)),
        (r#"      ~~~~~~~ "#, EqualsGreaterThanCode(r#" "(" "#)),
        (r#"             ~"#, Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_openningbracket_then_equalsgreaterthancode_empty_stringliteral() {
    test(r#" "\(" => "" ,"#, vec![
        (r#" ~~~~        "#, StringLiteral(r#"\("#)),
        (r#"      ~~~~~~ "#, EqualsGreaterThanCode(r#" "" "#)),
        (r#"            ~"#, Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_dot_then_equalsgreaterthancode_one_character_dot() {
    test(r#" "\." => '.' ,"#, vec![
        (r#" ~~~~         "#, StringLiteral(r#"\."#)),
        (r#"      ~~~~~~~ "#, EqualsGreaterThanCode(r#" '.' "#)),
        (r#"             ~"#, Comma),
    ]);
}

#[test]
fn rule_stringliteral_slash_openningbracket_then_equalsgreaterthancode_one_character_openningbracket() {
    test(r#" "\(" => '(' ,"#, vec![
        (r#" ~~~~         "#, StringLiteral(r#"\("#)),
        (r#"      ~~~~~~~ "#, EqualsGreaterThanCode(r#" '(' "#)),
        (r#"             ~"#, Comma),
    ]);
}

#[test]
fn equalsgreaterthancode_one_character_openningbracket() {
    test(r#"=> '(' ,"#, vec![
        (r#"~~~~~~~ "#, EqualsGreaterThanCode(r#" '(' "#)),
        (r#"       ~"#, Comma),
    ]);
}

#[test]
fn equalsgreaterthancode_one_character_openningcurlybracket() {
    test(r#"=> '{' ,"#, vec![
        (r#"~~~~~~~ "#, EqualsGreaterThanCode(r#" '{' "#)),
        (r#"       ~"#, Comma),
    ]);
}

#[test]
fn equalsgreaterthancode_one_character_openningsquarebracket() {
    test(r#"=> '[' ,"#, vec![
        (r#"~~~~~~~ "#, EqualsGreaterThanCode(r#" '[' "#)),
        (r#"       ~"#, Comma),
    ]);
}

#[test]
fn equalsgreaterthancode_one_character_openningbracket_wrapped_by_brackets() {
    test(r#"=> ('(') ,"#, vec![
        (r#"~~~~~~~~~ "#, EqualsGreaterThanCode(r#" ('(') "#)),
        (r#"         ~"#, Comma),
    ]);
}

#[test]
fn code_paren() { // Issue #25
    test(r#"=> a("(", c),"#, vec![
        (r#"~~~~~~~~~~~~ "#, EqualsGreaterThanCode(r#" a("(", c)"#)),
        (r#"            ~"#, Comma),
    ]);
}

#[test]
fn code_regex_paren() { // Issue #25
    test(r###"=> a(r##"("#""##, c),"###, vec![
        (r###"~~~~~~~~~~~~~~~~~~~~ "###, EqualsGreaterThanCode(r###" a(r##"("#""##, c)"###)),
        (r###"                    ~"###, Comma),
    ]);
}

#[test]
fn code_comment_eol() {
    test("=> a(// (
),", vec![
        ("~~~~~~~~~
~,", EqualsGreaterThanCode(" a(// (\n)")),
        ("=> a(// (
)~", Comma)]);
}

#[test]
fn code2() {
    test("=>? a(b, c),", vec![
        ("~~~~~~~~~~~ ", EqualsGreaterThanQuestionCode(" a(b, c)")),
        ("           ~", Comma),
    ]);
}

#[test]
#[should_panic]
fn code_forgot_comma() {
    test("=> a(b, c),", vec![
        ("~~~~~~~~~~ ", EqualsGreaterThanCode(" a(b, c)")),
        // intentionally forget the comma token; this is more of a test of `test`
    ]);
}

#[test]
fn various_kinds_of_ids() {
    test("foo<T<'a,U,`Z*{}`>>", vec![
        ("~~~                ", MacroId("foo")),
        ("   ~               ", LessThan),
        ("    ~              ", MacroId("T")),
        ("     ~             ", LessThan),
        ("      ~~           ", Lifetime("'a")),
        ("        ~          ", Comma),
        ("         ~         ", Id("U")),
        ("          ~        ", Comma),
        ("           ~~~~~~  ", Escape("Z*{}")),
        ("                 ~ ", GreaterThan),
        ("                  ~", GreaterThan),
    ]);
}

#[test]
fn string_literals() {
    test(r#"foo "bar\"\n" baz"#, vec![
        (r#"~~~              "#, Id("foo")),
        (r#"    ~~~~~~~~~    "#, StringLiteral(r#"bar\"\n"#)),
        (r#"              ~~~"#, Id("baz")),
    ]);
}

#[test]
fn use1() {
    test(r#"use foo::bar; baz"#, vec![
        (r#"~~~~~~~~~~~~     "#, Use(" foo::bar")),
        (r#"            ~    "#, Semi),
        (r#"              ~~~"#, Id("baz")),
    ]);
}

#[test]
fn use2() {
    test(r#"use {foo,bar}; baz"#, vec![
        (r#"~~~~~~~~~~~~~     "#, Use(" {foo,bar}")),
        (r#"             ~    "#, Semi),
        (r#"               ~~~"#, Id("baz")),
    ]);
}

#[test]
fn where1() {
    test(r#"where <foo,bar>,baz;"#, vec![
        (r#"~~~~~~~~~~~~~~~~~~~ "#, Where(vec![" <foo,bar>", "baz"])),
        (r#"                   ~"#, Semi),
    ]);
}

#[test]
fn regex1() {
    test(r#####"raa r##" #"#"" "#"##rrr"#####, vec![
        (r#####"~~~                    "#####, Id("raa")),
        (r#####"    ~~~~~~~~~~~~~~~~   "#####, RegexLiteral(r##" #"#"" "#"##)),
        (r#####"                    ~~~"#####, Id("rrr")),
    ]);
}

#[test]
fn regex2() {
    test(r#"r"(123""#, vec![
        (r#"~~~~~~~"#, RegexLiteral(r"(123")),
    ]);
}

