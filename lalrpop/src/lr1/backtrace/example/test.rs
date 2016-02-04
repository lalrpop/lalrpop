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

macro_rules! syms {
    ($($t:ident),*) => {
        vec![$(Symbol::Nonterminal(nt(stringify!($t)))),*]
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
