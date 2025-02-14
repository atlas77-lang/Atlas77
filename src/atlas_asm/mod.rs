//The codegen outputs a `Vec<Instruction>` and this crate will try to take it and output a `Vec<u8>` or a `Vec<u16>`

use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
/// # The opcodes for the VM
///
/// ## Instruction format
/// An instruction in the atlas VM is a 1 byte opcode followed by the immediate data
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

    /// Multiply the top 2 values in the stack
    ///
    /// The two values are popped from the stack and the result is pushed back
    Mul,

    /// Divide the top 2 values in the stack
    ///
    /// The two values are popped from the stack and the result is pushed back
    Div,

    /// Subtract the top 2 values in the stack
    ///
    /// The two values are popped from the stack and the result is pushed back
    Sub,

    /// Modulo the top two values in the stack
    ///
    /// The two values are popped from the stack and the result is pushed back
    Mod,

    /// Add the top two values in the stack
    ///
    /// The two integers are popped from the stack and the result is pushed back
    Add,

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
    /// The function descriptor is stored in the next 4 bytes
    DirectCall,
    /// Call an external function and return the result to the top of the stack
    ///
    /// The function name is a string stored in the constant pool. The function name index is stored in the next 4 bytes
    /// The number of arguments is stored in the next 1 byte
    ExternCall,
    /// Returns from the current function, pushing a return value from the callee's stack onto the caller's stack
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub instructions: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstantPool {
    pub strings: Vec<String>,
    pub functions: Vec<FunctionDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionPool {
    pub functions: Vec<FunctionDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDescriptor {
    pub name: String,
    pub nb_args: u8,
}
