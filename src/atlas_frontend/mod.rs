use std::path::PathBuf;

use lexer::AtlasLexer;
use parser::{arena::AstArena, ast::AstProgram, error::ParseResult};

pub mod lexer;
pub mod parser;

pub fn parse<'ast>(path: &'ast str, arena: &'ast bumpalo::Bump) -> ParseResult<AstProgram<'ast>> {
    let source = std::fs::read_to_string(path).unwrap();
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    let tokens = lex.set_source(source.clone()).tokenize().unwrap();
    let mut parser = parser::Parser::new(AstArena::new(arena), tokens, PathBuf::from(path), source);
    parser.parse()
}
