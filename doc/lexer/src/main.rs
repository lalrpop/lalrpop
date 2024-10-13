use lexer::grammar::ScriptParser;
use lexer::lexer::Lexer;

fn main() {
    let source_code = "var a = 42;
var b = 23;

# a comment
print (a - b);";

    let lexer = Lexer::new(source_code);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);

    assert!(matches!(ast[0], lexer::ast::Statement::Variable { .. }));
    assert!(matches!(ast[1], lexer::ast::Statement::Variable { .. }));
    assert!(matches!(ast[2], lexer::ast::Statement::Print { .. }));

    #[cfg(feature = "bit")]
    {
        let source_code = "var a = 4;
var b = 2;

# a comment
print (a << b);";

        let lexer = Lexer::new(source_code);
        let parser = ScriptParser::new();
        let ast = parser.parse(lexer).unwrap();

        println!("{:?}", ast);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use lexer::grammar::ScriptParser;
        use lexer::lexer::Lexer;

        #[cfg(test)]
        {
            let source_code = "var a = 42;
var b = 23;

# a comment
print (a - b);";

            let lexer = Lexer::new(source_code);
            let parser = ScriptParser::new();
            let ast = parser.parse(lexer).unwrap();

            println!("{:?}", ast);

            assert!(matches!(ast[0], lexer::ast::Statement::Variable { .. }));
            assert!(matches!(ast[1], lexer::ast::Statement::Variable { .. }));
            assert!(matches!(ast[2], lexer::ast::Statement::EPrint { .. }));

            let _ = lexer::grammar::TESTParser::new();
        }
    }
}
