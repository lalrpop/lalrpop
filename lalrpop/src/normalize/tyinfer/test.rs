use intern::intern;
use parser;
use normalize::macro_expand::expand_macros;
use normalize::tyinfer::infer_types;
use grammar::parse_tree::NonterminalString;
use grammar::repr::TypeRepr;

fn type_repr(s: &str) -> TypeRepr {
    let type_ref = parser::parse_type_ref(s).unwrap();
    return type_ref.type_repr();
}

fn compare(g1: &str, expected: Vec<(&'static str, &'static str)>) {
    let grammar = parser::parse_grammar(g1).unwrap();
    let grammar = expand_macros(grammar).unwrap();
    let types = infer_types(&grammar).unwrap();

    println!("types table: {:?}", types);

    for (nt_id, nt_type) in expected {
        let id = NonterminalString(intern(nt_id));
        let ty = type_repr(nt_type);
        println!("expected type of {:?} is {:?}", id, ty);
        assert_eq!(types.nonterminal_type(id), &ty);
    }
}

#[test]
fn test_pairs_and_tokens() {
    compare("
grammar {
    extern token { enum Tok { } }
    X = Y Z;
    Y: Foo = \"Hi\";
    Z = \"Ho\";
}
", vec![
    ("X", "(Foo, Tok)"),
    ("Y", "Foo"),
    ("Z", "Tok")
        ])
}

#[test]
fn test_cycle_direct() {
    let grammar = parser::parse_grammar("
grammar {
    extern token { enum Tok { } }
    X = {
        X Y;
        <Y> => vec![<>];
    };
    Y = \"Hi\";
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    assert!(infer_types(&actual).is_err());
}

#[test]
fn test_cycle_indirect() {
    let grammar = parser::parse_grammar("
grammar {
    extern token { enum Tok { } }
    A = B;
    B = C;
    C = D;
    D = A;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    assert!(infer_types(&actual).is_err());
}

#[test]
fn test_macro_expansion() {
    compare("
grammar {
    extern token { enum Tok { } }
    Two<X>: (X, X) = X X;
    Ids = Two<\"Id\">;
}
", vec![
    ("Ids", "(Tok, Tok)"),
    (r#"Two<"Id">"#, "(Tok, Tok)"),
        ])
}

#[test]
fn test_macro_expansion_infer() {
    compare("
grammar {
    extern token { enum Tok { } }
    Two<X> = X X;
    Ids = Two<\"Id\">;
}
", vec![
    ("Ids", "(Tok, Tok)"),
    (r#"Two<"Id">"#, "(Tok, Tok)"),
        ])
}

#[test]
fn test_type_question() {
    compare("
grammar {
    extern token { enum Tok { } }
    X = Y?;
    Y = \"Hi\";
}
",vec![
    ("X", "std::option::Option<Tok>"),
    ("Y", "Tok")
        ])
}

#[test]
fn test_star_plus_question() {
    compare("
grammar {
    extern token { enum Tok { } }
    A = Z*;
    X = \"Hi\"*;
    Y = \"Hi\"+;
    Z = \"Hi\"?;
}
", vec![
    ("A", "std::vec::Vec<std::option::Option<Tok>>"),
    ("X", "std::vec::Vec<Tok>"),
    ("Y", "std::vec::Vec<Tok>"),
    ("Z", "std::option::Option<Tok>")
        ])
}

#[test]
fn test_action() {
    compare(r#"
grammar {
    extern token { enum Tok { } }

    X = {
        Y;
        <l:X> "+" <r:Y> => l + r;
    };

    Y: i32 = "foo" => 22;
}
"#,vec![
    ("X", "i32"),
    ("Y", "i32"),
        ])
}

#[test]
fn test_inconsistent_action() {
    let grammar = parser::parse_grammar("
grammar {
    extern token { enum Tok { } }

    X = {
        Y;
        Z;
        <l:X> \"+\" <r:Y> => l + r;
    };

    Y: i32 = \"foo\" => 22;

    Z: u32 = \"bar\" => 22;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    assert!(infer_types(&actual).is_err());
}
