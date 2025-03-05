use crate::parser;
use crate::test_util;

fn check_err(expected_err: &str, grammar: &str, span: &str) {
    let parsed_grammar = parser::parse_grammar(grammar).unwrap();
    let err = super::validate(&parsed_grammar).unwrap_err();
    test_util::check_norm_err(expected_err, span, err);
}

#[test]
fn named_symbols() {
    check_err(
        r#"named symbols \(like `"Num"`\) require a custom action"#,
        r#"grammar; Term = { <n:"Num"> };"#,
        r#"                     ~~~~~    "#,
    );
}

#[test]
fn bad_assoc_type() {
    check_err(
        r#"associated type `Foo` not recognized"#,
        r#"grammar; extern { type Foo = i32; enum Tok { } }"#,
        r#"                       ~~~                      "#,
    );
}

#[test]
fn dup_assoc_type() {
    check_err(
        r#"associated type `Location` already specified"#,
        r#"grammar; extern { type Location = i32; type Location = u32; enum Tok { } }"#,
        r#"                                            ~~~~~~~~                      "#,
    );
}

#[test]
fn lookahead_without_loc_type() {
    check_err(
        r#"lookahead/lookbehind require you to declare the type of a location"#,
        r#"grammar; extern { enum Tok { } } Foo = @L;"#,
        r#"                                       ~~ "#,
    );
}

#[test]
fn multiple_extern_token() {
    check_err(
        r#"multiple extern definitions are not permitted"#,
        r#"grammar; extern { enum Tok { } } extern { enum Tok { } }"#,
        r#"                                 ~~~~~~                 "#,
    );
}

#[test]
fn unrecognized_attribute() {
    check_err(
        r#"unrecognized attribute `foo`"#,
        r#"grammar; #[foo] Term = ();"#,
        r#"           ~~~            "#,
    );
}

#[test]
fn duplicate_attribute() {
    check_err(
        r#"duplicate attribute `inline`"#,
        r#"grammar; #[inline] #[inline] Term = ();"#,
        r#"                     ~~~~~~            "#,
    );
}

#[test]
fn pub_inline_attribute() {
    check_err(
        r"public items cannot be marked #\[inline\]",
        r#"grammar; #[inline] pub Term = ();"#,
        r#"           ~~~~~~            "#,
    );
}

#[test]
fn missing_cfg_attribute_arg() {
    check_err(
        r#"`cfg` attributes take one argument"#,
        r#"grammar; #[cfg] pub Term = ();"#,
        r#"           ~~~                "#,
    )
}

#[test]
fn multiple_match_token() {
    check_err(
        r#"multiple match definitions are not permitted"#,
        r#"grammar; match { _ } match { _ }"#,
        r#"                     ~~~~~      "#,
    );
}

#[test]
fn match_after_extern_token() {
    check_err(
        r"match and extern \(with custom tokens\) definitions are mutually exclusive",
        r#"grammar; extern { enum Tok { } } match { _ }"#,
        r#"                                 ~~~~~      "#,
    );
}

#[test]
fn extern_after_match_token() {
    check_err(
        r"extern \(with custom tokens\) and match definitions are mutually exclusive",
        r#"grammar; match { _ } extern { enum Tok { } }"#,
        r#"                     ~~~~~~                 "#,
    );
}

#[test]
fn expandable_expression_requires_named_variables() {
    check_err(
        r"Using `<>` between curly braces \(e.g., `\{<>\}`\) only works when your parsed values have been given names \(e.g., `<x:Foo>`, not just `<Foo>`\)",
        r#"grammar; Term = { <A> => Foo {<>} };"#,
        r#"                  ~~~~~~~~~~~~~~~~  "#,
    );
}

#[test]
fn mixing_names_and_anonymous_values() {
    check_err(
        r#"anonymous symbols like this one cannot be combined with named symbols like `b:B`"#,
        r#"grammar; Term = { <A> <b:B> => Alien: Eighth passanger of Nostromo};"#,
        r#"                  ~~~                                               "#,
    );
}

#[test]
fn public_macros() {
    check_err(
        r#"macros cannot be marked public"#,
        r#"grammar; pub Comma<T> = (T ",")* T?;"#,
        r#"             ~~~~~~~~               "#,
    );
}

#[test]
fn alternative_unrecognized_attribute() {
    check_err(
        r#"unrecognized attribute `foo`"#,
        r#"grammar; Term = { #[foo(bar="baz")] "a" => () };"#,
        r#"                    ~~~~~~~~~~~~~~              "#,
    );
}

#[test]
fn missing_precedence() {
    check_err(
        r#"missing precedence attribute on the first alternative"#,
        r#"grammar; Term = { "a" => (), #[precedence(level="1")] "b" => () };"#,
        r#"                  ~~~~~~~~~                                       "#,
    );
}

#[test]
fn cannot_parse_precedence() {
    check_err(
        r#"could not parse the precedence level `a`, expected integer"#,
        r#"grammar; Term = { #[precedence(level="a")] "a" => ()};"#,
        r#"                    ~~~~~~~~~~~~~~~~~~~~~             "#,
    );
}

#[test]
fn invalid_lvl_precedence() {
    check_err(
        r#"invalid argument `foo` for precedence attribute, expected `level`"#,
        r#"grammar; Term = { #[precedence(foo="1")] "a" => ()};"#,
        r#"                    ~~~~~~~~~~~~~~~~~~~             "#,
    );
}

#[test]
fn missing_arg_precedence() {
    check_err(
        r#"missing argument for precedence attribute, expected `level`"#,
        r#"grammar; Term = { #[precedence] "a" => ()};"#,
        r#"                    ~~~~~~~~~~             "#,
    );
}

#[test]
fn cannot_parse_assoc() {
    check_err(
        r#"could not parse the associativity `foo`, expected `left`, `right`, `none` or `all`"#,
        r#"grammar; Term = { #[precedence(level="1")] #[assoc(side="foo")] "a" => ()};"#,
        r#"                                             ~~~~~~~~~~~~~~~~~             "#,
    );
}

#[test]
fn invalid_assoc() {
    check_err(
        r#"invalid argument `foo` for associativity attribute, expected `side`"#,
        r#"grammar; Term = { #[precedence(level="1")] #[assoc(foo="left")] "a" => ()};"#,
        r#"                                             ~~~~~~~~~~~~~~~~~             "#,
    );
}

#[test]
fn missing_arg_assoc() {
    check_err(
        r#"missing argument for associativity attribute, expected `side`"#,
        r#"grammar; Term = { #[precedence(level="1")] #[assoc] "a" => ()};"#,
        r#"                                             ~~~~~             "#,
    );
}

#[test]
fn first_level_assoc() {
    check_err(
        r#"cannot set associativity on the first precedence level 3"#,
        r#"grammar; Term = { #[precedence(level="3")] #[assoc(side="left")] "a" => ()};"#,
        r#"                                             ~~~~~~~~~~~~~~~~~~             "#,
    );
}

#[test]
fn missing_macro_arg() {
    check_err(
        r#"macros must have at least one argument"#,
        r#"grammar; Macro<Smth>: String = { Smth => <>.to_string() } pub Root: String = { Macro<>}"#,
        r#"                                                                               ~~~~~~~"#,
    )
}
