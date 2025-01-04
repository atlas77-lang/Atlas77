use u64 as HlirDataType;

pub enum HlirResult {
    Success,
    VariableID(usize),
}
pub enum HlirError {
    NoMainFunction,
    VariableAlreadyExists(String, usize), //usize is the line number
    FunctionAlreadyExists(String, usize),
    VariableNotFound(String, usize),
    FunctionNotFound(String, usize),
    TypeMismatch(HlirDataType, HlirDataType, usize),
    ReturnTypeMismatch(String, String, usize),
    BinaryOpTypeMismatch(String, String, usize), //for example, in if construct, you need to have a comparison between two data types (==, !=, <, >, <=, >=) not a mathematical operation
    FunctionArgsMismatch(String, usize), //Where there are too much or too less arguments in a function call.
}