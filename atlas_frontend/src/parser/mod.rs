pub mod ast;

use std::path::PathBuf;

use ast::{AbstractSyntaxTree, BinaryOperator, MatchArm, Type, UnaryOperator};
use ast::{
    BinaryExpression, DoExpression, Expression, FunctionCall, FunctionExpression, IdentifierNode,
    IfElseNode, IndexExpression, MatchExpression, UnaryExpression, VariableDeclaration,
};
use atlas_core::prelude::Span;
use internment::Intern;
use crate::lexer::{TokenKind, TokenKind::*, Token};


#[derive(Debug, Clone)]
pub struct ParseError;

#[derive(Debug, Clone)]
pub struct SimpleParserV1 {
    tokens: Vec<Token>,
    file_path: PathBuf,
    pos: usize,
    eof_token: Token,
}

impl SimpleParserV1 {
    pub fn with_file_path(&mut self, file_path: PathBuf) -> Result<(), std::io::Error> {
        self.file_path = file_path;
        return Ok(());
    }

    pub fn with_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }
    
    pub fn parse(&mut self) -> Result<AbstractSyntaxTree, ParseError> {
        let mut ast: AbstractSyntaxTree = Vec::new();
        self.advance(); //Start of Input (SoI)
        while self.current().kind() != EoI {
            ast.push(self.parse_expression().expect("Failed to parse expression"));
        }
        Ok(ast)
    }
    /// Creates a new empty `SimpleParserV1`
    pub fn new() -> Self {
        SimpleParserV1 { 
            tokens: Vec::default(), 
            file_path: PathBuf::default(), 
            pos: usize::default(),
            eof_token: Token::new(Span::default(), TokenKind::EoI),
        }
    }

    fn current(&self) -> &Token {
        match self.tokens.get(self.pos) {
            Some(t) => t,
            None => &self.eof_token,
        }
    }

    fn advance(&mut self) -> &Token {
        let tok = self.tokens.get(self.pos);
        if let Some(t) = tok {
            self.pos += 1;
            t
        } else {
            &self.eof_token
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<&Token, ParseError> {
        let tok = self.advance();
        if tok.kind() == expected {
            Ok(tok)
        } else {
            eprintln!("Expected {:?}, got {:?}", expected, tok.kind());
            Err(ParseError)
        }
    }
    
    fn parse_type(&mut self) -> Result<Box<Type>, ParseError> {
        let tok = self.advance();
        match tok.kind() {
            LParen => {
                let mut args = vec![];
                while self.current().kind() != RParen {
                    let mut arg: (Intern<String>, Type) = (Intern::new(String::default()), Type::Void);
                    match self.advance().kind().clone() {
                        TokenKind::Literal(crate::lexer::Literal::Identifier(s)) => {
                            arg.0 = s;
                        }
                        _ => {
                            eprintln!("Unexpected token: {:?}", self.current().kind());
                            unimplemented!()
                        }
                    }
                    self.expect(Colon)?;
                    arg.1 = *self.parse_type()?;
                    args.push(arg);
                    if self.current().kind() == Comma {
                        self.advance();
                    }
                }
                self.expect(RParen)?;

                self.expect(RArrow)?;
                let ret = self.parse_type()?;

                return Ok(Box::new(Type::Function(args, ret)))
            },
            TokenKind::Keyword(kw) => match kw.as_str() {
                "List" => {
                    self.expect(LBracket)?;
                    let t = self.parse_type()?;
                    self.expect(RBracket)?;
                    return Ok(Box::new(Type::List(t)))
                },
                "Map" => {
                    self.expect(LBracket)?;
                    let k = self.parse_type()?;
                    self.expect(Colon)?;
                    let v = self.parse_type()?;
                    self.expect(RBracket)?;
                    Ok(Box::new(Type::Map(k, v)))
                },
                "int" => return Ok(Box::new(Type::Integer)),
                "f64" => return Ok(Box::new(Type::Float)),
                "string" => return Ok(Box::new(Type::String)),
                "bool" => return Ok(Box::new(Type::Bool)),
                _ => unreachable!("Unexpected token: {:?}", tok)
            }
            _ => unreachable!("Unexpected token: {:?}", tok)
        }
    }

    fn parse_expression(&mut self) -> Result<Box<Expression>, ParseError> {
        match self.current().kind() {
            TokenKind::Keyword(kw) => match kw.as_str() {
                "let" => {
                    self.advance();
                    let var = self.parse_variable_declaration()?;
                    Ok(Box::new(Expression::VariableDeclaration(var)))
                },
                _ => {
                    //self.advance();
                    let expr = self.parse_expr()?;
                    Ok(Box::new(*expr))
                }
            }
            _ => {
                //self.advance();
                let expr = self.parse_expr()?;
                Ok(Box::new(*expr))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, ParseError> {
        let name = match self.advance().kind().clone() {
            TokenKind::Literal(crate::lexer::Literal::Identifier(s)) => {
                s
            }
            _ => unreachable!("Unexpected token: {:?}", self.current().kind())
        };
        self.expect(Colon)?;
        let t = *self.parse_type()?;
        if self.current().kind() == OpAssign {
            self.advance();
            match t.clone() {
                Type::Function(args, _) => {
                    // TODO: Typechecking required
                    let body = self.parse_expr()?;
                    Ok(VariableDeclaration { name, t, mutable: false, value: Some(Box::new(Expression::FunctionExpression(FunctionExpression { args, body }))) })
                },
                _ => {
                    let value = Some(self.parse_expr()?);
                    Ok(VariableDeclaration { name, t, mutable: true, value })
                }
            }
        } else {
            let value = self.parse_expr()?;
            Ok(VariableDeclaration { name, t, mutable: false, value: Some(value) })
        }
        
    }

    pub fn parse_expr(&mut self) -> Result<Box<Expression>, ParseError> {
        let expr = self.parse_binary()?;        
        Ok(expr)
    }

    fn parse_binary(&mut self) -> Result<Box<Expression>, ParseError> {
        let left = self.parse_term()?;
        let op = Option::<BinaryOperator>::from(&self.current().kind());
        
        match self.current().kind() {
            OpAdd | OpSub => {
                self.advance();
                let right = self.parse_binary()?;
                Ok(Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: op.unwrap(), right })))
            }
            _ => Ok(left)
        }
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, ParseError> {
        let left = self.parse_factor()?;
        let op = Option::<BinaryOperator>::from(&self.current().kind());

        match self.current().kind() {
            OpMul | OpDiv | OpMod => {
                self.advance();
                let right = self.parse_term()?;
                Ok(Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: op.unwrap(), right })))
            }
            _ => Ok(left)
        }
    }

    fn parse_factor(&mut self) -> Result<Box<Expression>, ParseError> {
        let left = self.parse_condition()?;
        let op = Option::<BinaryOperator>::from(&self.current().kind());

        match self.current().kind() {
            OpPow => {
                self.advance();
                let right = self.parse_binary()?; //Check this line with tests
                Ok(Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: op.unwrap(), right })))
            }
            _ => Ok(left)
        }
    }

    fn parse_condition(&mut self) -> Result<Box<Expression>, ParseError> {
        let left = self.parse_power()?;
        let operator = Option::<BinaryOperator>::from(&self.current().kind());

        match self.current().kind() {
            OpEq | OpNEq | OpLessThan | OpLessThanEq | OpGreaterThan | OpGreaterThanEq => {
                self.advance();
                let right = self.parse_expr()?;
                Ok(Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: operator.unwrap(), right })))
            }
            _ => Ok(left)
        }
    }

    fn parse_power(&mut self) -> Result<Box<Expression>, ParseError> {

        let operator = match &self.current().kind() {
            TokenKind::OpSub => {
                self.advance();
                Some(UnaryOperator::OpSub)
            },
            TokenKind::Bang => {
                self.advance();
                Some(UnaryOperator::OpNot)
            },
            _ => None,
        };
        
        Ok(Box::new(Expression::UnaryExpression(UnaryExpression { operator, expression: self.parse_primary()? })))
    }

    fn parse_primary(&mut self) -> Result<Box<Expression>, ParseError> {
        match self.current().kind().clone() {
            TokenKind::Literal(crate::lexer::Literal::Float(f)) => {
                self.advance();
                Ok(Box::new(Expression::Literal(ast::Literal::Float(f))))
            }
            TokenKind::Literal(crate::lexer::Literal::Int(i)) => {
                self.advance();
                Ok(Box::new(Expression::Literal(ast::Literal::Integer(i))))
            }
            TokenKind::Literal(crate::lexer::Literal::StringLiteral(s)) => {
                self.advance();
                Ok(Box::new(Expression::Literal(ast::Literal::String(s))))
            }
            LBracket => {
                self.expect(LBracket)?;
                let mut exprs = vec![];
                while self.current().kind() != RBracket {
                    exprs.push(self.parse_expr()?.as_ref().clone());
                    if self.current().kind() == Comma {
                        self.advance();
                    }
                }
                self.expect(RBracket)?;
                Ok(Box::new(Expression::Literal(ast::Literal::List(exprs))))
            }
            TokenKind::Keyword(kw) => match kw.as_str() {
                "true" => {
                    self.advance();
                    Ok(Box::new(Expression::Literal(ast::Literal::Bool(true))))
                },
                "false" => {
                    self.advance();
                    Ok(Box::new(Expression::Literal(ast::Literal::Bool(false))))
                },
                "do" => {
                    self.expect(TokenKind::Keyword(Intern::new("do".to_string())))?;
                    let mut expressions = vec![];
                    while self.current().kind() != TokenKind::Keyword(Intern::new("end".to_string())) {
                        expressions.push(self.parse_expression()?);
                        self.expect(Semicolon)?;
                    }
                    self.expect(TokenKind::Keyword(Intern::new("end".to_string())))?;
                    Ok(Box::new(Expression::DoExpression(DoExpression { body: expressions })))
                },
                "let" => {
                    self.parse_expression()
                },
                "if" => {
                    self.expect(TokenKind::Keyword(Intern::new("if".to_string())))?;
                    let condition = self.parse_expr()?;
                    self.expect(TokenKind::Keyword(Intern::new("then".to_string())))?;
                    let if_body = self.parse_expr()?;
                    if self.current().kind() == TokenKind::Keyword(Intern::new("else".to_string())) {
                        self.expect(TokenKind::Keyword(Intern::new("else".to_string())))?;
                        let else_body = self.parse_expr()?;
                        Ok(Box::new(Expression::IfElseNode(IfElseNode { condition, if_body, else_body: Some(else_body) })))
                    } else {
                        Ok(Box::new(Expression::IfElseNode(IfElseNode { condition, if_body, else_body: None })))
                    }
                },
                "match" => {
                    println!("Parsing match expression");
                    self.expect(TokenKind::Keyword(Intern::new("match".to_string())))?;
                    let expr = self.parse_expr()?;
                    let mut match_arm = vec![];
                    while self.current().kind() != BackSlash {
                        self.expect(Pipe)?;
                        match_arm.push(self.parse_match_arm()?);
                        self.expect(Comma)?;
                    }
                    self.expect(BackSlash)?;
                    let mut default_arm = None;
                    if self.current().kind() == Underscore {
                        self.expect(Underscore)?;
                        self.expect(FatArrow)?;
                        default_arm = Some(self.parse_expr()?);
                    } else {
                        match_arm.push(self.parse_match_arm()?);
                    }
                    Ok(Box::new(Expression::MatchExpression(MatchExpression { expr, arms: match_arm, default: default_arm })))
                },
                _ => {
                    eprintln!("Unexpected token: {:?}", self.current().kind());
                    unimplemented!()
                }
            }
            LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(RParen)?;
                Ok(expr)
            }
            TokenKind::Literal(crate::lexer::Literal::Identifier(s)) => {
                self.advance();
                if self.current().kind() == LParen {
                    self.expect(LParen)?;
                    let args = self.parse_arguments()?;
                    self.expect(RParen)?;
                    Ok(Box::new(Expression::FunctionCall(FunctionCall { name: s, args })))
                } else if self.current().kind() == LBracket  {
                    self.expect(LBracket)?;
                    let index = self.parse_expr()?;
                    self.expect(RBracket)?;
                    Ok(Box::new(Expression::IndexExpression(IndexExpression { name: s, index })))
                }
                else{
                    Ok(Box::new(Expression::Identifier(IdentifierNode { name: s })))
                }
            }

            _ => {
                eprintln!("Unexpected token: {:?}", self.current().kind());
                unimplemented!()
            }
        }
    }

    fn parse_match_arm(&mut self) -> Result<MatchArm, ParseError> {
        println!("Parsing match arm");
        let pattern = self.parse_expr()?;
        self.expect(FatArrow)?;
        let body = self.parse_expr()?;
        Ok(MatchArm { pattern, body })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Box<Expression>>, ParseError> {
        let mut args = vec![];
        while self.current().kind() != RParen {
            let expr = self.parse_expr()?;
            args.push(expr);
            if self.current().kind() == Comma {
                self.advance();
            }
        }
        Ok(args)
    }

}