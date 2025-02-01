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
    let mut lex = AtlasLexer::new(path, source.clone());
    let token_res = lex.tokenize();
    let tokens = match token_res {
        Ok(tokens) => tokens,
        Err(e) => panic!("Error while lexing: {:?}", e),
    };
    let mut parser = parser::Parser::new(arena, tokens, PathBuf::from(path), source);
    let res = parser.parse()?;
    //print classes in res
    for item in res.items.iter() {
        if let parser::ast::AstItem::Class(class) = item {
            println!("{:?}", class);
        }
    }

    Ok(res)
}
