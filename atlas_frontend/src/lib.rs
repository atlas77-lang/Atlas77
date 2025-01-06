use lexer::AtlasLexer;
use parser::{ast::AbstractSyntaxTree, ParseError};

pub mod lexer;
pub mod parser;

pub fn parse(path: &str) -> Result<AbstractSyntaxTree, ParseError> {
    //"default()" setup all the systems you asked for, tho you could use "new()" to add them manually
    let source = std::fs::read_to_string(path).unwrap();
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    let tokens = lex
        .set_path("src/main.atlas")
        .set_source(source)
        .tokenize()
        .unwrap();
    let mut parser = parser::SimpleParserV1::new();
    parser.with_tokens(tokens);
    parser.parse()
}
