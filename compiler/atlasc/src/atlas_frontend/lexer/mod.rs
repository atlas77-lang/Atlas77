use crate::atlas_frontend::lexer::token::{LexingError, Token, TokenKind};
use logos::{Logos, Span};

pub mod token;

#[derive(Debug)]
pub struct AtlasLexer<'lex> {
    path: &'lex str,
    pub source: String,
}

impl<'lex> AtlasLexer<'lex> {
    pub fn new(path: &'lex str, source: String) -> Self {
        AtlasLexer {
            path,
            source,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexingError> {
        let lex = TokenKind::lexer(&self.source);
        let mut res: Vec<Result<Token, LexingError>> = lex.spanned().map(|(kind, span)| {
            match kind {
                Ok(kind) => Ok(Token::new(span, kind)),
                Err(e) => Err(e),
            }
        }).collect::<Vec<_>>();
        res.push(Ok(Token::new(Span::default(), TokenKind::EoI)));
        res.into_iter().collect::<Result<Vec<_>, LexingError>>()
    }
}

pub trait Spanned {
    fn union_span(&self, other: &Self) -> Self;
}

impl Spanned for Span {
    fn union_span(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            TokenKind::KwElse => {
                write!(f, "else")
            }
            TokenKind::KwEnum => {
                write!(f, "enum")
            }
            TokenKind::KwExtern => {
                write!(f, "extern")
            }
            _ => {
                write!(f, "{:?}", self.kind())
            }
        }
    }
}

#[derive(Debug)]
pub struct TokenVec(pub Vec<TokenKind>);

impl std::fmt::Display for TokenVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(f, "{:?} ", token)?;
        }
        Ok(())
    }
}
