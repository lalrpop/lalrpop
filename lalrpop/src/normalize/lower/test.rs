use grammar::repr::{Grammar, Production};
use normalize::normalize_without_validating;
use parser;
use test_util::expect_debug;

fn flat_productions(grammar: &Grammar) -> Vec<Production> {
    let mut productions: Vec<_> =
        grammar.productions.values()
                           .flat_map(|prods| prods.iter().cloned())
                           .collect();

    // sort by the action fn index just to get a consistent ordering
    productions.sort_by(|k1, k2| {
        Ord::cmp(&k1.action_fn.index(), &k2.action_fn.index())
    });

    productions
}

#[test]
fn test_comma() {
    let grammar = parser::parse_grammar("
grammar {
    extern token { enum Tok { } }

    Comma<E>: Vec<E> =
       <v:(<E> \",\")*> <e:E?> =>
           v.into_iter().chain(e.into_iter()).collect();

    Ids = Comma<\"Id\">;
}
").unwrap();
    let actual = normalize_without_validating(grammar).unwrap();

    expect_debug(flat_productions(&actual),
                 r#"[
    Ids = Comma<"Id"> => ActionFn(0);,
    Comma<"Id"> = (<"Id"> ",")*, "Id"? => ActionFn(1);,
    "Id"? = "Id" => ActionFn(2);,
    "Id"? =  => ActionFn(3);,
    (<"Id"> ",")* =  => ActionFn(4);,
    (<"Id"> ",")* = (<"Id"> ",")*, (<"Id"> ",") => ActionFn(5);,
    (<"Id"> ",") = "Id", "," => ActionFn(6);
]"#);

    expect_debug(&actual.action_fn_defns,
                 r#"[
    fn _(__0: Vec<Tok>) -> Vec<Tok> { (__0) },
    fn _(v: std::vec::Vec<Tok>, e: std::option::Option<Tok>) -> Vec<Tok> { v.into_iter().chain(e.into_iter()).collect() },
    fn _(__0: Tok) -> std::option::Option<Tok> { Some(__0) },
    fn _() -> std::option::Option<Tok> { None },
    fn _() -> std::vec::Vec<Tok> { vec![] },
    fn _(v: std::vec::Vec<Tok>, e: Tok) -> std::vec::Vec<Tok> { { let mut v = v; v.push(e); v } },
    fn _(__0: Tok, _: Tok) -> Tok { (__0) }
]"#);
}
