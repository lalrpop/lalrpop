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

    graph.add_edge(
        nt!(X),
        LR0Item { production: &productions[0], index: 1 },
        &productions[0].symbols[..1]);

    graph.add_edge(
        LR0Item { production: &productions[1], index: 1 },
        nt!(X),
        &[]);

    graph.add_edge(
        LR0Item { production: &productions[2], index: 1 },
        nt!(X),
        &[]);

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
