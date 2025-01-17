use std::path::PathBuf;

use lexer::AtlasLexer;
use parser::{arena::AstArena, ast::AstProgram, error::ParseResult};

pub mod lexer;
pub mod parser;

pub(crate) fn parse<'ast>(
    path: &'ast str,
    arena: &'ast AstArena<'ast>,
    source: String,
) -> ParseResult<AstProgram<'ast>> {
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    let tokens = lex.set_source(source.clone()).tokenize().unwrap();
    let mut parser = parser::Parser::new(arena, tokens, PathBuf::from(path), source);
    parser.parse()
}
