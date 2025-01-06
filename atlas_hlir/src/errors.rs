use atlas_core::prelude::Span;
use internment::Intern;
use u64 as HlirDataType;

#[derive(Debug, Clone)]
pub enum HlirResult {
    Success,
    VariableID(usize),
}

#[derive(Debug, Clone)]
pub enum HlirError {
    NoMainFunction,
    VariableAlreadyExists {
        name: Intern<String>,
        span: Span,
    },
    VariableNotFound {
        name: Intern<String>,
        span: Span,
    },
    TypeMismatch {
        type1: HlirDataType,
        type2: HlirDataType,
        span: Span,
    },
}
