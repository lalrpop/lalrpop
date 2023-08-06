use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod eval;
pub mod lexer;

lalrpop_mod!(pub parser);

pub fn compile(input: &str) -> Result<ast::Program, String> {
    match parser::ProgramParser::new().parse(lexer::Lexer::new(input)) {
        Ok(s) => Ok(ast::Program::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[test]
fn parse_simple() {
    let input = lexer::Lexer::new("\n\n\n");
    let program = parser::ProgramParser::new().parse(input).expect("Oh no");
    match (program.len(), program.first()) {
        (1, Some(&ast::Stmt::Exit)) => (),
        other => panic!("Well that didn't work: {:?}", other),
    }
}
