pub mod lexer;
pub mod parser;

use std::path::PathBuf;

use lexer::AtlasLexer;
use parser::{arena::AstArena, ast::AstProgram, error::ParseResult};


pub fn parse<'ast>(
    path: &'ast str,
    arena: &'ast AstArena<'ast>,
    source: String,
) -> ParseResult<AstProgram<'ast>> {
    let mut lex: AtlasLexer = lexer::AtlasLexer::new(path, source.clone());
    let tokens = lex.tokenize().unwrap();
    let mut parser = parser::Parser::new(arena, tokens, PathBuf::from(path), source);
    parser.parse()
}
