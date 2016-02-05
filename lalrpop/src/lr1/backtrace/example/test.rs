use intern::intern;
use grammar::parse_tree::TerminalLiteral;
use grammar::repr::*;

use super::super::{Example, Reduction};

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn term(t: &str) -> TerminalString {
    TerminalString::Literal(TerminalLiteral::Quoted(intern(t)))
}

macro_rules! sym {
    (ε) => { None };
    ($t:ident) => { Some(Symbol::Nonterminal(nt(stringify!($t)))) }
}

macro_rules! syms {
    ($($t:tt),*) => {
        vec![$(sym!($t)),*]
    }
}

//  012345678901234567890
//  A1   B2  C3  D4 E5 F6
//  |             |     |
//  +-LongLabel22-+     |
//  |                   |
//  +-Label-------------+
fn long_label_1_example() -> Example {
    Example {
        symbols: syms!(A1,B2,C3,D4,E5,F6),
        reductions: vec![
            Reduction { start: 0, end: 4, nonterminal: nt("LongLabel22") },
            Reduction { start: 0, end: 6, nonterminal: nt("Label") }],
    }
}

#[test]
fn long_label_1_positions() {
    let example = long_label_1_example();
    let lengths = example.lengths();
    let positions = example.positions(&lengths);
    assert_eq!(positions, vec![0, 5, 9, 13, 16, 19, 21]);
}

// Note that `positions` can handle this,
// but maybe not `paint`.
//
//  01234567890123456789012345678901234
//         A1  B2  C3 D4 E5       F6
//  |   |           |       |   | |   |
//  +-X-+           |       +-Y-+ +-Z-+
//  |               |
//  +-MegaLongLabel-+
fn empty_labels_example() -> Example {
    Example {
        //             0 1  2  3  4  5  6 7
        symbols: syms!(ε,A1,B2,C3,D4,E5,ε,F6),
        reductions: vec![
            Reduction { start: 0, end: 1, nonterminal: nt("X") },
            Reduction { start: 0, end: 4, nonterminal: nt("MegaLongLabel") },
            Reduction { start: 6, end: 7, nonterminal: nt("Y") },
            Reduction { start: 7, end: 8, nonterminal: nt("Z") }],
    }
}

#[test]
fn empty_labels_positions() {
    let example = empty_labels_example();
    let lengths = example.lengths();
    let positions = example.positions(&lengths);
    //                            A1 B2  C3  D4  E5      F6
    assert_eq!(positions, vec![0, 7, 11, 15, 18, 21, 24, 30, 35]);
}
