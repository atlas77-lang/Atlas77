//The codegen outputs a `Vec<Instruction>` and this crate will try to take it and output a `Vec<u8>` or a `Vec<u16>`

use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum OpCode {
    /// No operation
    Nop = 0x00,

    // Constants

    /// Push an immediate integer to the stack
    ///
    /// The immediate integer is stored in the next 8 bytes
    PushInteger,
    /// Push an immediate float to the stack
    ///
    /// The immediate float is stored in the next 8 bytes
    PushFloat,
    /// Push an immediate unsigned integer to the stack
    ///
    /// The immediate unsigned integer is stored in the next 8 bytes
    PushUnsignedInteger,
    /// Push an immediate boolean to the stack
    ///
    /// The immediate boolean is stored in the next 1 byte
    PushBoolean,
    /// Push an immediate character to the stack
    ///
    /// The immediate character is stored in the next 4 byte
    /// The character is stored as an utf-8 character
    PushChar,
    /// Push a string from the constant pool
    ///
    /// The string is directly put in the memory and its pointer is pushed to the stack
    /// The string index is stored in the next 4 bytes
    PushStr,
    /// Push a function pointer from the constant pool to the stack
    ///
    /// The function pointer is stored in the next 4 bytes
    PushFnPtr,
    /// Push a list from the constant pool
    ///
    /// The list is directly put in the memory and its pointer is pushed to the stack
    /// The list index is stored in the next 8 bytes
    PushList,
    /// Push a unit to the stack
    PushUnit,
    /// Push a null to the stack
    ///
    /// Equivalent to pushing a unit
    PushNull,

    // Stack operations

    /// Pop the top value from the stack
    Pop,
    /// Swap the top two values in the stack
    Swap,
    /// Duplicate the top value in the stack
    Dup,

    //Variables

    /// Load the value of a variable to the top of the stack
    ///
    /// As the variable is already in the stack,
    /// the variable offset is stored in the next 4 bytes
    Get,
    /// Store the value of a variable to the stack
    /// The variable index is stored in the next 4 bytes
    Store,

    // Arithmetics

    /// Multiply two signed integers 64 bits
    ///
    /// The two integers are popped from the stack and the result is pushed back
    IMul,
    /// Multiply two floats 64 bits
    ///
    /// The two floats are popped from the stack and the result is pushed back
    FMul,
    /// Multiply two unsigned integers 64 bits
    ///
    /// The two unsigned integers are popped from the stack and the result is pushed back
    UIMul,

    /// Divide two signed integers 64 bits
    ///
    /// The two integers are popped from the stack and the result is pushed back
    IDiv,
    /// Divide two floats 64 bits
    ///
    /// The two floats are popped from the stack and the result is pushed back
    FDiv,
    /// Divide two unsigned integers 64 bits
    ///
    /// The two unsigned integers are popped from the stack and the result is pushed back
    UIDiv,

    /// Subtract two signed integers 64 bits
    ///
    /// The two integers are popped from the stack and the result is pushed back
    ISub,
    /// Subtract two floats 64 bits
    ///
    /// The two floats are popped from the stack and the result is pushed back
    FSub,
    /// Subtract two unsigned integers 64 bits
    ///
    /// The two unsigned integers are popped from the stack and the result is pushed back
    UISub,

    /// Modulo two signed integers 64 bits
    ///
    /// The two integers are popped from the stack and the result is pushed back
    IMod,
    /// Modulo two floats 64 bits
    ///
    /// The two floats are popped from the stack and the result is pushed back
    FMod,
    /// Modulo two unsigned integers 64 bits
    ///
    /// The two unsigned integers are popped from the stack and the result is pushed back
    UIMod,

    /// Add two signed integers 64 bits
    ///
    /// The two integers are popped from the stack and the result is pushed back
    IAdd,
    /// Add two floats 64 bits
    ///
    /// The two floats are popped from the stack and the result is pushed back
    FAdd,
    /// Add two unsigned integers 64 bits
    ///
    /// The two unsigned integers are popped from the stack and the result is pushed back
    UIAdd,

    // Comparisons
    /// Compare two values for equality
    ///
    /// The two values are popped from the stack and the result is pushed back
    Eq,
    /// Compare two values for inequality
    ///
    /// The two values are popped from the stack and the result is pushed back
    Neq,
    /// Compare two values for greater than
    ///
    /// The two values are popped from the stack and the result is pushed back
    Gt,
    /// Compare two values for greater than or equal
    ///
    /// The two values are popped from the stack and the result is pushed back
    Gte,
    /// Compare two values for less than
    ///
    /// The two values are popped from the stack and the result is pushed back
    Lt,
    /// Compare two values for less than or equal
    ///
    /// The two values are popped from the stack and the result is pushed back
    Lte,

    // Jumps
    /// Jump to an absolute position
    ///
    /// The position is stored in the next 4 bytes
    DJmp,
    /// Jump to a relative position
    ///
    /// The relative position is stored in the next 4 bytes
    RJmp,
    /// Jump to a relative position if the top of the stack is 0
    /// - The top of the stack is equal to 0 if the result of a comparison is false
    ///
    /// The relative position is stored in the next 4 bytes
    JmpZ,
    // Functions
    /// Call a function by taking the top of the stack value as the fn_ptr
    ///
    /// The number of arguments is stored in the next 1 byte
    Call,
    /// Call a function
    ///
    /// The function pointer is stored in the next 4 bytes
    /// The number of arguments is stored in the next 1 byte
    DirectCall,
    /// Call an external function and return the result to the top of the stack
    ///
    /// The function name is a string stored in the constant pool. The function name index is stored in the next 4 bytes
    /// The number of arguments is stored in the next 1 byte
    ExternCall,
    Return,

    // List operations

    /// Create a new empty list and push its pointer to the stack
    ///
    /// The number of elements is stored in the next 4 bytes
    NewList,
    /// Copy a list to another one 
    ListCopy,
    /// Stack state:
    ///
    /// - **Bottom** `[Index, ListPointer]` **Top**
    ///
    /// Load a value from a list to the top of the stack
    ListLoad,
    /// Stack state:
    ///
    /// - **Bottom** `[Index, ListPointer, Value]` **Top**
    ///
    /// Store a value in a given list
    ListStore,

    Halt,
}
