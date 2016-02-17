use intern::intern;
use grammar::repr::*;
use lr1::build_states;
use lr1::core::Item;
use lr1::interpret::interpret_partial;
use lr1::lookahead::Lookahead;
use test_util::{expect_debug, normalized_grammar};
use tls::Tls;

use super::super::Tracer;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn term(t: &str) -> TerminalString {
    TerminalString::Literal(TerminalLiteral::Quoted(intern(t)))
}

macro_rules! terms {
    ($($t:expr),*) => {
        vec![$(term($t)),*]
    }
}

fn test_grammar1() -> Grammar {
    normalized_grammar(r#"
    grammar;

    pub Start: () = Stmt;

    pub Stmt: () = {
        Exprs ";",
        Exprs
    };

    Exprs: () = {
        Expr,
        Exprs "," Expr
    };

    Expr: () = {
        "Int",
        Expr "+" "Int",
    };
"#)
}

#[test]
fn backtrace1() {
    let _tls = Tls::test();
    let grammar = test_grammar1();
    let states = build_states(&grammar, nt("Start")).unwrap();
    let tracer = Tracer::new(&grammar, &states);
    let state_stack = interpret_partial(&states, terms!["Int"]).unwrap();
    let top_state = *state_stack.last().unwrap();

    // Top state will have items like:
    //
    // Expr = "Int" (*) [EOF],
    // Expr = "Int" (*) ["+"],
    // Expr = "Int" (*) [","],
    // Expr = "Int" (*) [";"]
    //
    // Select the last one.
    let semi = Lookahead::Terminal(term(";"));
    let semi_item = states[top_state.0].items.vec.iter()
                                                 .filter(|item| item.lookahead == semi)
                                                 .next()
                                                 .unwrap();

    let backtrace = tracer.backtrace_reduce(top_state, *semi_item);

    println!("{:#?}", backtrace);
    expect_debug(&backtrace, r#"
[
    (Nonterminal(Expr) -(["Int"], None, [])-> Item(Expr = "Int" (*))),
    (Nonterminal(Exprs) -([Exprs, ","], Some(Expr), [])-> Item(Exprs = Exprs "," (*) Expr)),
    (Nonterminal(Exprs) -([Exprs, ","], Some(Expr), [])-> Nonterminal(Expr)),
    (Nonterminal(Exprs) -([], Some(Expr), [])-> Item(Exprs = (*) Expr)),
    (Nonterminal(Exprs) -([], Some(Expr), [])-> Nonterminal(Expr)),
    (Item(Stmt = (*) Exprs ";") -([], Some(Exprs), [";"])-> Nonterminal(Exprs))
]
"#.trim());
}

#[test]
fn backtrace2() {
    let _tls = Tls::test();
    let grammar = test_grammar1();
    let states = build_states(&grammar, nt("Start")).unwrap();
    let tracer = Tracer::new(&grammar, &states);
    let state_stack = interpret_partial(&states, terms!["Int"]).unwrap();
    let top_state = *state_stack.last().unwrap();

    // Top state will have items like:
    //
    // Expr = "Int" (*) [EOF],
    // Expr = "Int" (*) ["+"],
    // Expr = "Int" (*) [","],
    // Expr = "Int" (*) [";"]
    //
    // Select the last one.
    let plus = Lookahead::Terminal(term("+"));
    let plus_item = states[top_state.0].items.vec.iter()
                                                 .filter(|item| item.lookahead == plus)
                                                 .next()
                                                 .unwrap();

    let backtrace = tracer.backtrace_reduce(top_state, *plus_item);

    println!("{:#?}", backtrace);
    expect_debug(&backtrace, r#"
[
    (Nonterminal(Expr) -(["Int"], None, [])-> Item(Expr = "Int" (*))),
    (Item(Expr = (*) Expr "+" "Int") -([], Some(Expr), ["+", "Int"])-> Nonterminal(Expr))
]
"#.trim());
}

#[test]
fn backtrace3() {
    let _tls = Tls::test();
    // This grammar yields a S/R conflict. Is it (int -> int) -> int
    // or int -> (int -> int)?
    let grammar = normalized_grammar(r#"
grammar;
pub Ty: () = {
    "int" => (),
    "bool" => (),
    <t1:Ty> "->" <t2:Ty> => (),
};
"#);
    let states = build_states(&grammar, nt("Ty")).unwrap_err().states;
    let tracer = Tracer::new(&grammar, &states);
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|s| &s.conflicts)
              .flat_map(|(l, cs)| cs.iter().map(move |c| (l, c)))
              .next()
              .unwrap();
    println!("conflict={:?}", conflict);
    let item = Item { production: conflict.production,
                      index: conflict.production.symbols.len(),
                      lookahead: lookahead };
    println!("item={:?}", item);
    let backtrace = tracer.backtrace_reduce(conflict.state, item);
    println!("{:#?}", backtrace);
    expect_debug(&backtrace, r#"
[
    (Nonterminal(Ty) -([Ty, "->"], Some(Ty), [])-> Item(Ty = Ty "->" (*) Ty)),
    (Nonterminal(Ty) -([Ty, "->"], Some(Ty), [])-> Nonterminal(Ty)),
    (Nonterminal(Ty) -([Ty, "->", Ty], None, [])-> Item(Ty = Ty "->" Ty (*))),
    (Item(Ty = (*) Ty "->" Ty) -([], Some(Ty), ["->", Ty])-> Nonterminal(Ty))
]
"#.trim());

    // Check that we can successfully enumerate and paint the examples
    // here.
    let pictures: Vec<_> = backtrace.examples(item.to_lr0())
                                    .map(|e| e.paint_unstyled())
                                    .collect();
    expect_debug(&pictures, r#"
[
    [
        "Ty \"->\" Ty \"->\" Ty",
        "|        |       |",
        "+-Ty-----+       |",
        "|                |",
        "+-Ty-------------+"
    ]
]
"#.trim());
}

#[test]
fn reduce_backtrace_3_graph() {
    // This grammar yields a S/R conflict. Is it `(int -> int) -> int`
    // or `int -> (int -> int)`?
    let _tls = Tls::test();
    let grammar = normalized_grammar(r#"
grammar;
pub Ty: () = {
    "int" => (),
    "bool" => (),
    <t1:Ty> "->" <t2:Ty> => (),
};
"#);
    let states = build_states(&grammar, nt("Ty")).unwrap_err().states;
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|s| &s.conflicts)
              .flat_map(|(l, cs)| cs.iter().map(move |c| (l, c)))
              .next()
              .unwrap();
    println!("conflict={:?}", conflict);
    let item = Item { production: conflict.production,
                      index: conflict.production.symbols.len(),
                      lookahead: lookahead };
    println!("item={:?}", item);
    let tracer = Tracer::new(&grammar, &states);
    let graph = tracer.backtrace_reduce(conflict.state, item);
    expect_debug(&graph, r#"
[
    (Nonterminal(Ty) -([Ty, "->"], Some(Ty), [])-> Item(Ty = Ty "->" (*) Ty)),
    (Nonterminal(Ty) -([Ty, "->"], Some(Ty), [])-> Nonterminal(Ty)),
    (Nonterminal(Ty) -([Ty, "->", Ty], None, [])-> Item(Ty = Ty "->" Ty (*))),
    (Item(Ty = (*) Ty "->" Ty) -([], Some(Ty), ["->", Ty])-> Nonterminal(Ty))
]
"#.trim());

    let list: Vec<_> =
        graph.examples(item.to_lr0())
             .map(|example| example.paint_unstyled())
             .collect();
    expect_debug(&list, r#"
[
    [
        "Ty \"->\" Ty \"->\" Ty",
        "|        |       |",
        "+-Ty-----+       |",
        "|                |",
        "+-Ty-------------+"
    ]
]
"#.trim());
}
