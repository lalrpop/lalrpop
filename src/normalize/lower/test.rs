use parser;
use normalize::normalize;
use normalize::test_util::expect_debug;

#[test]
fn test_comma() {
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };

    Comma<E>: Vec<E> =
       ~v:(~E \",\")* ~e:E? =>
           v.into_iter().chain(e.into_iter()).collect();

    Ids = Comma<\"Id\">;
}
").unwrap();
    let actual = normalize(grammar).unwrap();
    expect_debug(actual,
                 r#"Grammar {
    action_fn_defns: [
        fn _(__0: Vec<Tok>) -> Vec<Tok> { (__0) },
        fn _(v: std::vec::Vec<Tok>, e: std::option::Option<Tok>) -> Vec<Tok> { v.into_iter().chain(e.into_iter()).collect() },
        fn _(__0: Tok) -> std::option::Option<Tok> { Some(__0) },
        fn _() -> std::option::Option<Tok> { None },
        fn _() -> std::vec::Vec<Tok> { vec![] },
        fn _(v: std::vec::Vec<Tok>, e: Tok) -> std::vec::Vec<Tok> { { let mut v = v; v.push(e); v } },
        fn _(__0: Tok, _: Tok) -> Tok { (__0) }
    ],
    productions: [
        Ids = Comma<"Id"> => ActionFn(0);,
        Comma<"Id"> = (~"Id" ",")*, "Id"? => ActionFn(1);,
        "Id"? = "Id" => ActionFn(2);,
        "Id"? =  => ActionFn(3);,
        (~"Id" ",")* =  => ActionFn(4);,
        (~"Id" ",")* = (~"Id" ",")*, (~"Id" ",") => ActionFn(5);,
        (~"Id" ",") = "Id", "," => ActionFn(6);
    ],
    conversions: {},
    types: DummyTypes {
        terminal_type: Tok,
        nonterminal_types: [
            ("\"Id\"?", std::option::Option<Tok>),
            ("(~\"Id\" \",\")", Tok),
            ("(~\"Id\" \",\")*", std::vec::Vec<Tok>),
            ("Comma<\"Id\">", Vec<Tok>),
            ("Ids", Vec<Tok>)
        ]
    }
}"#)

}
