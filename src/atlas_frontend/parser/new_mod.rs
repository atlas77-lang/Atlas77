//todo: Add Parser::sync() to recover from errors

use std::path::PathBuf;

use atlas_core::prelude::{Span, Spanned};
use miette::{SourceOffset, SourceSpan};

use super::error::{ParseError, ParseResult, UnexpectedTokenError};
use super::new_ast::{
    AstBinaryOp, AstBinaryOpExpr, AstBooleanLiteral, AstBooleanType, AstCallExpr, AstCompTimeExpr,
    AstDoExpr, AstEnum, AstEnumVariant, AstExpr, AstExternFunction, AstFieldAccessExpr,
    AstFieldInit, AstFloatLiteral, AstFloatType, AstFunction, AstFunctionType, AstIdentifier,
    AstIfElseExpr, AstInclude, AstIndexingExpr, AstIntegerLiteral, AstIntegerType, AstItem,
    AstLambdaExpr, AstLetExpr, AstLiteral, AstMatchArm, AstNamedType, AstNewObjExpr, AstObjField,
    AstPattern, AstPatternKind, AstPointerType, AstProgram, AstStringLiteral, AstStringType,
    AstStruct, AstType, AstUnaryOp, AstUnaryOpExpr, AstUnion, AstUnionVariant, AstUnitType,
    AstUnsignedIntegerType,
};

use super::arena::AstArena;
use crate::atlas_frontend::lexer::{Literal, Token, TokenKind, TokenVec};
use crate::atlas_frontend::parser::new_ast::AstUnsignedIntegerLiteral;

pub struct Parser<'ast> {
    arena: AstArena<'ast>,
    tokens: Vec<Token>,
    //for error reporting
    file_path: PathBuf,
    pos: usize,
    src: String,
}

impl<'ast> Parser<'ast> {
    pub fn new(
        arena: AstArena<'ast>,
        tokens: Vec<Token>,
        file_path: PathBuf,
        src: String,
    ) -> Parser<'ast> {
        Parser {
            arena,
            tokens,
            file_path,
            pos: 0,
            src,
        }
    }

    #[must_use]
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    #[must_use]
    fn peek(&self) -> Option<TokenKind> {
        self.tokens.get(self.pos + 1).map(|t| t.kind())
    }

    #[must_use]
    /// This should maybe return a ParseResult::UnexpectedEndOfFileError
    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned();
        if let Some(t) = tok {
            self.pos += 1;
            t
        } else {
            Token::new(Span::default(), TokenKind::EoI)
        }
    }

    #[must_use]
    fn expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        let tok = self.advance();
        if tok.kind() == kind {
            Ok(tok)
        } else {
            Err(ParseError::UnexpectedToken(UnexpectedTokenError {
                token: tok.clone(),
                expected: TokenVec(vec![kind]),
                span: SourceSpan::new(SourceOffset::from(tok.start()), tok.end() - tok.start()),
                src: self.src.clone(),
            }))
        }
    }

    #[must_use]
    pub fn parse(&mut self) -> ParseResult<AstProgram<'ast>> {
        let mut items: Vec<AstItem> = Vec::new();
        let _ = self.advance(); // Skip the first token (SoI)
        while self.current().kind() != TokenKind::EoI {
            items.push(self.parse_item()?);
        }
        let node = AstProgram {
            items: self.arena.alloc_vec(items),
        };
        Ok(node)
    }

    #[must_use]
    fn parse_item(&mut self) -> ParseResult<AstItem<'ast>> {
        let start = self.current().start();
        match self.current().kind() {
            TokenKind::KwStruct => Ok(AstItem::Struct(self.parse_struct()?)),
            TokenKind::KwInclude => Ok(AstItem::Include(self.parse_include()?)),
            TokenKind::KwExtern => Ok(AstItem::ExternFunction(self.parse_extern_function()?)),
            TokenKind::KwFunc => Ok(AstItem::Func(self.parse_func()?)),
            _ => Err(ParseError::UnexpectedToken(UnexpectedTokenError {
                token: self.current().clone(),
                expected: TokenVec(vec![TokenKind::Literal(Literal::Identifier(
                    "Item".to_string(),
                ))]),
                span: SourceSpan::new(SourceOffset::from(start), self.current().end() - start),
                src: self.src.clone(),
            })),
        }
    }

    #[must_use]
    fn parse_func(&mut self) -> ParseResult<AstFunction<'ast>> {
        todo!("parse_func")
    }

    #[must_use]
    fn parse_let(&mut self) -> ParseResult<AstLetExpr<'ast>> {
        let _ = self.advance();
        let name = self.parse_identifier()?;

        self.expect(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        self.expect(TokenKind::OpAssign)?;

        let value = self.parse_binary()?;
        let node = AstLetExpr {
            span: Span::union_span(name.span, value.span()),
            name: self.arena.alloc(name),
            ty: Some(self.arena.alloc(ty)),
            value: self.arena.alloc(value),
        };
        Ok(node)
    }

    #[must_use]
    fn parse_binary(&mut self) -> ParseResult<AstExpr<'ast>> {
        let left = self.parse_factor()?;
        match self.current().kind() {
            TokenKind::OpAdd | TokenKind::OpSub => {
                let op = match self.current().kind() {
                    TokenKind::OpAdd => AstBinaryOp::Add,
                    TokenKind::OpSub => AstBinaryOp::Sub,
                    _ => unreachable!(),
                };
                let _ = self.advance();
                let right = self.parse_binary()?;
                let node = AstExpr::BinaryOp(AstBinaryOpExpr {
                    span: Span::union_span(left.span(), right.span()),
                    op,
                    lhs: self.arena.alloc(left),
                    rhs: self.arena.alloc(right),
                });
                return Ok(node);
            }
            _ => return Ok(left),
        }
    }

    #[must_use]
    fn parse_factor(&mut self) -> ParseResult<AstExpr<'ast>> {
        let left = self.parse_condition()?;
        match self.current().kind() {
            TokenKind::OpMul | TokenKind::OpDiv | TokenKind::OpMod => {
                let op = match self.current().kind() {
                    TokenKind::OpMul => AstBinaryOp::Mul,
                    TokenKind::OpDiv => AstBinaryOp::Div,
                    TokenKind::OpMod => AstBinaryOp::Mod,
                    _ => unreachable!(),
                };
                let _ = self.advance();
                let right = self.parse_factor()?;
                let node = AstExpr::BinaryOp(AstBinaryOpExpr {
                    span: Span::union_span(left.span(), right.span()),
                    op,
                    lhs: self.arena.alloc(left),
                    rhs: self.arena.alloc(right),
                });
                return Ok(node);
            }
            _ => return Ok(left),
        }
    }
    #[must_use]
    fn parse_condition(&mut self) -> ParseResult<AstExpr<'ast>> {
        let left = AstExpr::UnaryOp(self.parse_unary()?);

        match self.current().kind() {
            TokenKind::OpEq
            | TokenKind::OpNEq
            | TokenKind::OpGreaterThan
            | TokenKind::OpGreaterThanEq
            | TokenKind::OpLessThan
            | TokenKind::OpLessThanEq => {
                let op = match self.current().kind() {
                    TokenKind::OpEq => AstBinaryOp::Eq,
                    TokenKind::OpNEq => AstBinaryOp::NEq,
                    TokenKind::OpGreaterThan => AstBinaryOp::Gt,
                    TokenKind::OpGreaterThanEq => AstBinaryOp::Gte,
                    TokenKind::OpLessThan => AstBinaryOp::Lt,
                    TokenKind::OpLessThanEq => AstBinaryOp::Lte,
                    _ => unreachable!(),
                };
                let _ = self.advance();
                let right = self.parse_condition()?;
                let node = AstExpr::BinaryOp(AstBinaryOpExpr {
                    span: Span::union_span(left.span(), right.span()),
                    op,
                    lhs: self.arena.alloc(left),
                    rhs: self.arena.alloc(right),
                });
                return Ok(node);
            }
            _ => return Ok(left),
        }
    }

    #[must_use]
    fn parse_unary(&mut self) -> ParseResult<AstUnaryOpExpr<'ast>> {
        let start_pos = self.current().span();
        let op = match self.current().kind() {
            TokenKind::OpSub => {
                let _ = self.advance();
                Some(AstUnaryOp::Neg)
            }
            TokenKind::Bang => {
                let _ = self.advance();
                Some(AstUnaryOp::Not)
            }
            _ => None,
        };

        let expr = self.parse_primary()?;
        let node = AstUnaryOpExpr {
            span: Span::union_span(start_pos, self.current().span()),
            op,
            expr: self.arena.alloc(expr),
        };
        Ok(node)
    }
    #[must_use]
    fn parse_primary(&mut self) -> ParseResult<AstExpr<'ast>> {
        let tok = self.current();

        let node = match tok.kind() {
            TokenKind::Literal(Literal::Bool(b)) => {
                AstExpr::Literal(AstLiteral::Boolean(AstBooleanLiteral {
                    span: tok.span(),
                    value: b,
                }))
            }
            TokenKind::Literal(Literal::Float(f)) => {
                AstExpr::Literal(AstLiteral::Float(AstFloatLiteral {
                    span: tok.span(),
                    value: f,
                }))
            }
            TokenKind::Literal(Literal::F64(f)) => {
                AstExpr::Literal(AstLiteral::Float(AstFloatLiteral {
                    span: tok.span(),
                    value: f,
                }))
            }
            TokenKind::Literal(Literal::U64(u)) => {
                AstExpr::Literal(AstLiteral::UnsignedIntegerer(AstUnsignedIntegerLiteral {
                    span: tok.span(),
                    value: u,
                }))
            }
            TokenKind::Literal(Literal::Int(i)) => {
                AstExpr::Literal(AstLiteral::Integer(AstIntegerLiteral {
                    span: tok.span(),
                    value: i,
                }))
            }
            TokenKind::Literal(Literal::I64(i)) => {
                AstExpr::Literal(AstLiteral::Integer(AstIntegerLiteral {
                    span: tok.span(),
                    value: i,
                }))
            }
            TokenKind::Literal(Literal::StringLiteral(s)) => {
                AstExpr::Literal(AstLiteral::String(AstStringLiteral {
                    span: tok.span(),
                    value: self.arena.alloc(s),
                }))
            }
            //temporary, until the lexer is fixed
            TokenKind::KwTrue | TokenKind::KwFalse => {
                AstExpr::Literal(AstLiteral::Boolean(AstBooleanLiteral {
                    span: tok.span(),
                    value: tok.kind() == TokenKind::KwTrue,
                }))
            }
            _ => {
                return Err(ParseError::UnexpectedToken(UnexpectedTokenError {
                    token: tok.clone(),
                    expected: TokenVec(vec![TokenKind::Literal(Literal::Identifier(
                        "Primary expression".to_string(),
                    ))]),
                    span: SourceSpan::new(SourceOffset::from(tok.start()), tok.end() - tok.start()),
                    src: self.src.clone(),
                }))
            }
        };
        let _ = self.advance();
        Ok(node)
    }

    #[must_use]
    fn parse_extern_function(&mut self) -> ParseResult<AstExternFunction<'ast>> {
        let _ = self.advance();
        let name = self.parse_identifier()?;
        self.expect(TokenKind::LParen)?;
        let mut params = vec![];
        while self.current().kind() != TokenKind::RParen {
            params.push(self.parse_type()?);
            if self.current().kind() == TokenKind::Comma {
                let _ = self.advance();
            }
        }
        self.expect(TokenKind::RParen)?;
        let _ = self.expect(TokenKind::RArrow)?;
        let ret_ty = self.parse_type()?;
        let node = AstExternFunction {
            span: Span::union_span(name.span, ret_ty.span()),
            name: self.arena.alloc(name),
            args: self.arena.alloc_vec(params),
            ret: self.arena.alloc(ret_ty),
        };
        Ok(node)
    }

    #[must_use]
    fn parse_include(&mut self) -> ParseResult<AstInclude<'ast>> {
        let start = self.advance();

        let path = match self.current().kind() {
            TokenKind::Literal(Literal::StringLiteral(s)) => s,
            _ => {
                return Err(ParseError::UnexpectedToken(UnexpectedTokenError {
                    token: self.current().clone(),
                    expected: TokenVec(vec![TokenKind::Literal(Literal::StringLiteral(
                        "String Literal".to_string(),
                    ))]),
                    span: SourceSpan::new(
                        SourceOffset::from(self.current().start()),
                        self.current().end() - self.current().start(),
                    ),
                    src: self.src.clone(),
                }));
            }
        };
        let _ = self.advance();

        if let TokenKind::KwAs = self.current().kind() {
            let _ = self.advance();
            let alias = self.parse_identifier()?;
            let node = AstInclude {
                span: Span::union_span(start.span(), alias.span),
                path: self.arena.alloc(path),
                alias: Some(self.arena.alloc(alias)),
            };
            Ok(node)
        } else {
            let node = AstInclude {
                span: Span::union_span(start.span(), start.span()),
                path: self.arena.alloc(path),
                alias: None,
            };
            Ok(node)
        }
    }

    #[must_use]
    fn parse_struct(&mut self) -> ParseResult<AstStruct<'ast>> {
        let _ = self.advance();

        let ident = self.parse_identifier()?;

        self.expect(TokenKind::LParen)?;

        let mut fields = vec![];
        while self.current().kind() != TokenKind::RParen {
            fields.push(self.parse_obj_field()?);
            if self.current().kind() == TokenKind::Comma {
                let _ = self.advance();
            }
        }
        self.expect(TokenKind::RParen)?;
        let node = AstStruct {
            span: Span::union_span(ident.span, self.current().span()),
            name: self.arena.alloc(ident),
            fields: self.arena.alloc_vec(fields),
        };
        Ok(node)
    }

    #[must_use]
    fn parse_obj_field(&mut self) -> ParseResult<AstObjField<'ast>> {
        let name = self.parse_identifier()?;

        self.expect(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        let node = AstObjField {
            span: Span::union_span(name.span, ty.span()),
            name: self.arena.alloc(name),
            ty: self.arena.alloc(ty),
        };

        Ok(node)
    }

    #[must_use]
    fn parse_identifier(&mut self) -> ParseResult<AstIdentifier<'ast>> {
        let token = self.current();

        let node = match token.kind() {
            TokenKind::Literal(Literal::Identifier(s)) => AstIdentifier {
                span: Span::union_span(self.current().span(), self.current().span()),
                name: self.arena.alloc(s),
            },
            _ => {
                return Err(ParseError::UnexpectedToken(UnexpectedTokenError {
                    token: self.current().clone(),
                    expected: TokenVec(vec![TokenKind::Literal(Literal::Identifier(
                        "Identifier".to_string(),
                    ))]),
                    span: SourceSpan::new(
                        SourceOffset::from(self.current().start()),
                        self.current().end() - self.current().start(),
                    ),
                    src: self.src.clone(),
                }));
            }
        };
        let _ = self.advance();
        Ok(node)
    }

    #[must_use]
    //Todo: add List<T>/Map<K, V>/() & function types
    fn parse_type(&mut self) -> ParseResult<AstType<'ast>> {
        let token = self.current();
        let start = self.current().span();
        match token.kind() {
            TokenKind::I64Ty => {
                let _ = self.advance();
                let node = AstType::Integer(AstIntegerType {
                    span: Span::union_span(start, self.current().span()),
                });
                Ok(node)
            }
            TokenKind::F64Ty => {
                let _ = self.advance();
                let node = AstType::Float(AstFloatType {
                    span: Span::union_span(start, self.current().span()),
                });
                Ok(node)
            }
            TokenKind::U64Ty => {
                let _ = self.advance();
                let node = AstType::UnsignedIntegerer(AstUnsignedIntegerType {
                    span: Span::union_span(start, self.current().span()),
                });
                Ok(node)
            }
            TokenKind::BoolTy => {
                let _ = self.advance();
                let node = AstType::Boolean(AstBooleanType {
                    span: Span::union_span(start, self.current().span()),
                });
                Ok(node)
            }
            TokenKind::StrTy => {
                let _ = self.advance();
                let node = AstType::String(AstStringType {
                    span: Span::union_span(start, self.current().span()),
                });
                Ok(node)
            }
            TokenKind::UnitTy => {
                let _ = self.advance();
                let node = AstType::Unit(AstUnitType {
                    span: Span::union_span(start, self.current().span()),
                });
                Ok(node)
            }
            TokenKind::Ampersand => {
                let _ = self.advance();
                let ty = self.parse_type()?;
                let node = AstType::Pointer(AstPointerType {
                    span: Span::union_span(start, ty.span()),
                    inner: self.arena.alloc(ty),
                });
                Ok(node)
            }
            TokenKind::Literal(Literal::Identifier(_)) => {
                let name = self.parse_identifier()?;
                let node = AstType::Named(AstNamedType {
                    span: Span::union_span(start, self.current().span()),
                    name: self.arena.alloc(name),
                });
                Ok(node)
            }
            TokenKind::LParen => {
                let _ = self.advance();
                let mut types = vec![];
                while self.current().kind() != TokenKind::RParen {
                    types.push(self.parse_type()?);
                    if self.current().kind() == TokenKind::Comma {
                        let _ = self.advance();
                    }
                }
                self.expect(TokenKind::RParen)?;

                self.expect(TokenKind::RArrow)?;

                let ret = self.parse_type()?;

                let node = AstType::Function(AstFunctionType {
                    span: Span::union_span(start, self.current().span()),
                    args: self.arena.alloc_vec(types),
                    ret: self.arena.alloc(ret),
                });
                Ok(node)
            }
            _ => Err(ParseError::UnexpectedToken(UnexpectedTokenError {
                token: self.current().clone(),
                expected: TokenVec(vec![TokenKind::Literal(Literal::Identifier(
                    "h".to_string(),
                ))]),
                span: SourceSpan::new(
                    SourceOffset::from(start.start()),
                    self.current().end() - start.start(),
                ),
                src: self.src.clone(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use bumpalo::Bump;
    use miette::Result;

    use super::*;
    use crate::atlas_frontend::lexer::AtlasLexer;

    #[test]
    fn test_parse_struct() -> Result<()> {
        let input = r#"
include "foo.atlas" as foo
struct Foo(bar: &str, baz: f64)
extern print(&str) -> (i64) -> (i64, &u64) -> () -> &unit
func main() -> i64 {}
        "#
        .to_string();
        let mut lexer = AtlasLexer::default();
        lexer.set_source(input.to_string());
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => panic!("{:?}", e),
        };
        let bump = Bump::new();
        let mut parser = Parser::new(AstArena::new(&bump), tokens, PathBuf::from("test"), input);
        let result = parser.parse();
        match result {
            Ok(program) => {
                for item in program.items.iter() {
                    match item {
                        AstItem::Struct(s) => {
                            println!(
                                "struct {:?} ({:?})\n",
                                s.name.name,
                                s.fields
                                    .iter()
                                    .map(|f| format!("{}: {:?}", f.name.name, f.ty))
                                    .collect::<String>()
                            );
                        }
                        AstItem::Include(i) => {
                            println!("include {:?} as {:?}\n", i.path, i.alias);
                        }
                        AstItem::ExternFunction(e) => {
                            println!(
                                "extern {:?} ({:?}) -> {:?}\n",
                                e.name.name,
                                e.args
                                    .iter()
                                    .map(|a| format!("{:?}", a))
                                    .collect::<String>(),
                                e.ret
                            );
                        }
                        AstItem::Func(f) => {
                            println!(
                                "func {:?} ({:?}) -> {:?}\n",
                                f.name.name,
                                f.args
                                    .iter()
                                    .map(|a| format!("{:?}", a))
                                    .collect::<String>(),
                                f.ret
                            );
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
