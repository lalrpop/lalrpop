use intern::intern;
use grammar::repr::*;
use lr1::build_states;
use lr1::core::*;
use session::Session;
use test_util::{expect_debug, normalized_grammar};

use super::super::Tracer;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

#[test]
fn shift_backtrace_1() {
    // This grammar yields a S/R conflict. Is it `(int -> int) -> int`
    // or `int -> (int -> int)`?

    let grammar = normalized_grammar(r#"
grammar;
pub Ty: () = {
    "int" => (),
    "bool" => (),
    <t1:Ty> "->" <t2:Ty> => (),
};
"#);
    let session = Session::test();
    let states = build_states(&session, &grammar, nt("Ty")).unwrap_err().states;
    let conflict =
        states.iter()
              .flat_map(|s| s.conflicts.values())
              .flat_map(|cs| cs.iter())
              .next()
              .unwrap();
    println!("conflict={:?}", conflict);

    // Gin up the LR0 item involved in the shift/reduce conflict:
    //
    //     Ty = Ty (*) -> Ty (shift)
    //
    // from the item that we can reduce:
    //
    //     Ty = Ty -> Ty (*) (reduce)

    assert!(conflict.production.symbols.len() == 3);
    let item = LR0Item { production: conflict.production, index: 1 };
    println!("item={:?}", item);
    let tracer = Tracer::new(&session, &grammar, &states);
    let graph = tracer.backtrace_shift_graph(conflict.state, item);
    expect_debug(&graph, r#"
[
    (Nonterminal(Ty) -([], Some(Ty), ["->", Ty])-> Nonterminal(Ty)),
    (Nonterminal(Ty) -([Ty], Some("->"), [Ty])-> Item(Ty = Ty (*) "->" Ty)),
    (Item(Ty = Ty "->" (*) Ty) -([Ty, "->"], Some(Ty), [])-> Nonterminal(Ty))
]
"#.trim());

    let list: Vec<_> = graph.enumerate_paths_from(item).collect();
    expect_debug(&list, r#"
[
    (
        [
            Ty,
            "->",
            Ty,
            "->",
            Ty
        ],
        3
    )
]
"#.trim());
}
