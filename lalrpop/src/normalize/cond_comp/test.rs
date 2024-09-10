use crate::normalize::NormResult;
use crate::parser;
use crate::session::Session;
use crate::test_util::compare;

use super::remove_disabled_decls;

#[test]
fn cfg_attr() {
    let grammar = parser::parse_grammar(
        r#"grammar;
A = ();
#[cfg(feature = "feat1")]
B = ();
#[cfg(not(feature = "feat3"))]
C = ();
#[cfg(all(feature = "feat1", feature = "feat2"))]
D = ();
#[cfg(any(feature = "feat1", feature = "feat3"))]
E = ();
#[cfg(all(
    feature = "feat1",
    not(feature = "feat3"),
    any(feature = "feat1"),
))]
F = ();

#[cfg(not(feature = "feat1"))]
G = ();
#[cfg(all(feature = "feat1", feature = "feat2", feature = "feat3"))]
H = ();
#[cfg(any(feature = "feat3", feature = "feat4"))]
I = ();
#[cfg(any(
    feature = "feat3",
    not(feature = "feat1"),
    any(feature = "feat3"),
))]
J = ();
"#,
    )
    .unwrap();

    let expected = parser::parse_grammar(
        r#"grammar;
A = ();
B = ();
C = ();
D = ();
E = ();
F = ();
"#,
    )
    .unwrap();

    let features = Some(["feat1", "feat2"].into_iter().map(str::to_string).collect());
    let session = Session {
        features,
        ..Default::default()
    };

    let mut grammar = remove_disabled_decls(&session, grammar);

    // remove attributes to compare with expected
    match &mut grammar {
        Ok(grammar) => grammar.items.iter_mut().for_each(|item| {
            if let super::GrammarItem::Nonterminal(nt) = item {
                nt.attributes.clear()
            }
        }),
        Err(_) => (),
    };

    compare(grammar, NormResult::Ok(expected));
}
