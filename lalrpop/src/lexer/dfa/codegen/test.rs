use lexer::dfa::Precedence;
use lexer::dfa::test::dfa;
use lexer::dfa::codegen;
use rust::RustWrite;

const P1: Precedence = Precedence(1);
const P0: Precedence = Precedence(0);

#[test]
fn codegen() {
    let dfa = dfa(&[
        /* 0 */ (r#"class"#, P1),
        /* 1 */ (r#"[a-zA-Z_][a-zA-Z0-9_]*"#, P0),
        /* 2 */ (r#"[0-9]+"#, P0),
        /* 3 */ (r#" +"#, P0),
        /* 4 */ (r#">>"#, P0),
        /* 5 */ (r#">"#, P0),
        ]).unwrap();

    let mut buffer = vec![];
    codegen::compile("_", &dfa, &mut RustWrite::new(&mut buffer)).unwrap();
    let string = String::from_utf8(buffer).unwrap();
    println!("{}", string);
    assert!(false);
}
