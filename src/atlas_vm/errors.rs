pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    OutOfMemory,
    StackOverflow,
    StackUnderflow,
    NullReference,
    DivisionByZero,
    IndexOutOfBounds,
    InvalidOperation,
    TypeMismatchError,
    EntryPointNotFound(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeError::*;
        match self {
            OutOfMemory => writeln!(f, "No more memory bozo"),
            StackOverflow => writeln!(f, "No more stack bozo"),
            StackUnderflow => writeln!(f, "Too little stack bozo"),
            NullReference => writeln!(f, "Null Reference error"),
            DivisionByZero => writeln!(f, "There are no infinity, you can't divide by zero"),
            IndexOutOfBounds => writeln!(f, "Index out of bounds"),
            InvalidOperation => writeln!(f, "Invalid Operation (default error)"),
            TypeMismatchError => writeln!(f, "Incorrect types bozo"),
            EntryPointNotFound(entry_point) => {
                writeln!(f, "Entry point {} not found", entry_point)
            }
        }
    }
}
