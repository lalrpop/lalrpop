use token::dfa::{build_dfa, DFA, NFAIndex};
use token::dfa::interpret::interpret;
use token::re;

fn dfa(strs: &[&str]) -> DFA {
    let regexs: Result<Vec<_>, _> = strs.iter().map(|s| re::parse_regex(s)).collect();
    let regexs = regexs.unwrap();
    build_dfa(&regexs)
}

#[test]
fn tokenizer() {
    let dfa = dfa(&[
        /* 0 */ r#"class"#, // this takes precedence over the code below
        /* 1 */ r#"[a-zA-Z_][a-zA-Z0-9_]*"#,
        /* 2 */ r#"[0-9]+"#,
        /* 3 */ r#" +"#,
        /* 4 */ r#">>"#,
        /* 5 */ r#">"#,
        ]);

    assert_eq!(interpret(&dfa, "class Foo"), Some((NFAIndex(0), "class")));
    assert_eq!(interpret(&dfa, "classz Foo"), Some((NFAIndex(1), "classz")));
    assert_eq!(interpret(&dfa, "123"), Some((NFAIndex(2), "123")));
    assert_eq!(interpret(&dfa, "  classz Foo"), Some((NFAIndex(3), "  ")));
    assert_eq!(interpret(&dfa, ">"), Some((NFAIndex(4), ">")));
    assert_eq!(interpret(&dfa, ">>"), Some((NFAIndex(5), ">>")));
}
