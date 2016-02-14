use intern::intern;
use grammar::repr::*;
use lr1::core::*;
use test_util::expect_debug;

macro_rules! nt {
    ($x:ident) => {
        NonterminalString(intern(stringify!($x)))
    }
}

macro_rules! syms {
    ($($x:ident),*) => {
        vec![$(Symbol::Nonterminal(nt!($x))),*]
    }
}

macro_rules! production {
    ($x:ident = $($y:ident)*) => {
        Production {
            nonterminal: nt!($x),
            symbols: syms![$($y),*],
            action: ActionFn::new(0),
            span: Span(0, 0)
        }
    }
}

use super::TraceGraph;

#[test]
fn enumerator() {
    // Build this graph:
    //
    //     X = X0 (*) X1
    //     ^
    //     |
    //   {X0}
    //     |
    // +-> X <-- Z = Z0 (*) X Z1
    // |
    // Y = Y0 (*) X Y1
    //
    // which enumerates out to:
    //
    //    [Y0 X0 (*) X1 Y1]
    //    [Z0 X0 (*) X1 Z1]

    let productions = vec![
        production![X = X0 X1],
        production![Y = Y0 X Y1],
        production![Z = Z0 X Z1],
    ];

    let mut graph = TraceGraph::new();

    let item0 = LR0Item { production: &productions[0], index: 1 }; // X = X0 (*) X1
    graph.add_edge(
        nt!(X),
        item0,
        item0.symbol_sets());

    graph.add_edge(
        LR0Item { production: &productions[1], index: 1 }, // Y = Y0 (*) X Y1
        nt!(X),
        SymbolSets::new());

    graph.add_edge(
        LR0Item { production: &productions[2], index: 1 }, // Z = Z0 (*) X Z1
        nt!(X),
        SymbolSets::new());

    let enumerator = graph.enumerate_paths_from(LR0Item {
        production: &productions[0],
        index: 1
    });
    let list: Vec<_> = enumerator.collect();
    expect_debug(&list, r#"
[
    (
        [
            Z0,
            X0,
            X1,
            Z1
        ],
        2
    ),
    (
        [
            Y0,
            X0,
            X1,
            Y1
        ],
        2
    )
]
"#.trim());
}

#[test]
fn enumerator1() {
    // Build this graph:
    //
    //     W = W0 W1 (*)
    //     ^
    //  {W0,W1}
    //     |
    //     W
    //     ^
    //   {X0}
    //     |
    // +-> X <-- Z = Z0 (*) X Z1
    // |
    // Y = Y0 (*) X Y1
    //
    // which enumerates out to:
    //
    //    [Y0 X0 (*) X1 Y1]
    //    [Z0 X0 (*) X1 Z1]

    let productions = vec![
        production![W = W0 W1],
        production![X = X0 W X1], // where X1 may be empty
        production![Y = Y0 X Y1],
        production![Z = Z0 X Z1],
    ];

    let mut graph = TraceGraph::new();

    let item0 = LR0Item { production: &productions[0], index: 2 }; // W = W0 W1 (*)
    graph.add_edge(
        nt!(W),
        item0,
        item0.symbol_sets());

    graph.add_edge(
        nt!(X),
        nt!(W),
        SymbolSets {
            prefix: &productions[1].symbols[..1],
            cursor: Some(&productions[1].symbols[1]),
            suffix: &productions[1].symbols[2..]
        });

    graph.add_edge(
        LR0Item { production: &productions[2], index: 1 },
        nt!(X),
        SymbolSets::new());

    graph.add_edge(
        LR0Item { production: &productions[3], index: 1 },
        nt!(X),
        SymbolSets::new());

    let enumerator = graph.enumerate_paths_from(LR0Item {
        production: &productions[0],
        index: 2,
    });
    let list: Vec<_> = enumerator.collect();
    expect_debug(&list, r#"
[
    (
        [
            Z0,
            X0,
            W0,
            W1,
            X1,
            Z1
        ],
        4
    ),
    (
        [
            Y0,
            X0,
            W0,
            W1,
            X1,
            Y1
        ],
        4
    )
]
"#.trim());
}
