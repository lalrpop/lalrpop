use intern::intern;
use parser;
use normalize::macro_expand::expand_macros;
use normalize::tyinfer::infer_types;
use grammar::parse_tree::TypeRef;
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
        let id = intern(nt_id);
        let ty = type_repr(nt_type);
        println!("expected type of {:?} is {:?}", id, ty);
        assert_eq!(types.nt_type(id), Some(&ty));
    }
}

#[test]
fn test_pairs_and_tokens() {
    compare("
grammar Foo {
         token Tok where { };
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
grammar Foo {
    token Tok where { };
    X = {
        X Y;
        Y;
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
grammar Foo {
    token Tok where { };
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
grammar Foo {
    token Tok where { };
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
grammar Foo {
    token Tok where { };
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
grammar Foo {
    token Tok where { };
    X = Y?;
    Y = \"Hi\";
}
",vec![
    ("X", "std::option::Option<Tok>"),
    ("Y", "Tok")
        ])
}
