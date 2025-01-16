use crate::declare_error_type;
use miette::{Diagnostic, SourceSpan as Span};
use thiserror::Error;

declare_error_type! {
    #[error("semantic error: {0}")]
    pub enum HirError {
        UnknownType(UnknownTypeError),
        BreakOutsideLoop(BreakOutsideLoopError),
        ContinueOutsideLoop(ContinueOutsideLoopError),
        TypeMismatch(TypeMismatchError),
        FunctionTypeMismatch(FunctionTypeMismatchError),
        UnsupportedStatement(UnsupportedStatement),
        UnsupportedExpr(UnsupportedExpr),
    }
}

/// Handy type alias for all HIR-related errors.
pub type HirResult<T> = Result<T, HirError>;

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::unknown_type))]
#[error("{expr} isn't supported yet")]
pub struct UnsupportedExpr {
    #[label = "unsupported expr"]
    pub span: Span,
    pub expr: String,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::unknown_type))]
#[error("{stmt} isn't supported yet")]
pub struct UnsupportedStatement {
    #[label = "unsupported statement"]
    pub span: Span,
    pub stmt: String,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::unknown_type))]
#[error("{name} does not name a known type")]
pub struct UnknownTypeError {
    pub name: String,
    #[label = "could not find type {name}"]
    pub span: Span,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::break_outside_loop))]
#[error("break statement outside of loop")]
pub struct BreakOutsideLoopError {
    #[label = "there is no enclosing loop"]
    pub span: Span,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::continue_outside_loop))]
#[error("continue statement outside of loop")]
pub struct ContinueOutsideLoopError {
    #[label = "there is no enclosing loop"]
    pub span: Span,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::type_mismatch))]
#[error("type mismatch")]
pub struct TypeMismatchError {
    pub actual_type: String,
    pub expected_type: String,
    #[label = "the expression has type {actual_type}"]
    pub actual_loc: Span,
    #[label = "expected type {expected_type}"]
    pub expected_loc: Span,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(sema::function_type_mismatch))]
#[error("function types do not take the same number of arguments")]
pub struct FunctionTypeMismatchError {
    pub expected_ty: String,
    #[label = "the function has type {expected_ty}"]
    pub span: Span,
}
