use grammar::parse_tree::NonterminalString;
use grammar::repr::Grammar;
use intern::intern;
use normalize::{self, NormResult};
use parser;
use test_util::compare;

use super::inline;

fn inlined_grammar(text: &str) -> NormResult<Grammar> {
    let g = parser::parse_grammar(text).unwrap();
    let g = normalize::lower_helper(g, true).unwrap();
    inline(g)
}

#[test]
fn sri() {
    // This grammar gets a shift-reduce conflict because if the input
    // is "&" (*) "L", then we see two possibilities, and we must decide
    // between them:
    //
    // "&" (*) "L" E
    //  |       |  |
    //  +-------+--|
    //          |
    //          E
    //
    // or
    //
    // "&"      (*) "L"
    //  |            |
    //  |  OPT_L     E
    //  |   |        |
    //  +---+---+----+
    //          |
    //          E
    //
    // to some extent this may be a false conflict, in that inlined
    // rules would address it, but it's an interesting one for
    // producing a useful error message.

    let grammar = inlined_grammar(r#"
        grammar;

        E: () = {
            "L",
            "&" OPT_L E
        };

        #[inline] OPT_L: () = {
            (),
            "L"
        };
    "#).unwrap();

    let nt = NonterminalString(intern("E"));

    // After inlining, we expect:
    // E = "L"
    // E = "&" () E
    // E = "&" "L" E
    let e_productions = grammar.productions_for(nt);
    assert_eq!(e_productions.len(), 3);
    assert_eq!(format!("{:?}", e_productions[0].symbols), r#"["L"]"#);
    assert_eq!(format!("{:?}", e_productions[1].symbols), r#"["&", (), E]"#);
    assert_eq!(format!("{:?}", e_productions[2].symbols), r#"["&", "L", E]"#);
}
