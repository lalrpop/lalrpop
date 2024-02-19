use lexer::grammar::ScriptParser;
use lexer::lexer::Lexer;

fn main() {
    let source_code = "var a = 42;
var b = 23;

# a comment
print (a - b);";

    let lexer = Lexer::new(&source_code[..]);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);
}
