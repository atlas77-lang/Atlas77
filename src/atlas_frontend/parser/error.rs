use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{atlas_frontend::lexer::Token, declare_error_type};

declare_error_type! {
    #[error("Parse error: {0}")]
    pub(crate) enum ParseError {
        UnexpectedEndOfFile(UnexpectedEndOfFileError),
        UnexpectedToken(UnexpectedTokenError),
    }
}

pub(crate) type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(syntax::unexpected_end_of_file),
    help("Add more input to form a valid program")
)]
#[error("expected more characters after this")]
pub(crate) struct UnexpectedEndOfFileError {
    #[label = "required more input to parse"]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(syntax::unexpected_token))]
#[error("Found unexpected token during parsing")]
pub(crate) struct UnexpectedTokenError {
    pub token: Token,
    pub expected: crate::atlas_frontend::lexer::TokenVec,
    #[label("was not expecting to find '{token}' in this position, expected one of: {expected}")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}
