//The codegen outputs a `Vec<Instruction>` and this crate will try to take it and output a `Vec<u8>` or a `Vec<u16>`

#[repr(u8)]
pub enum OpCode {
    Nop,

    /// Push an immediate integer to the stack
    /// The immediate integer is stored in the next 8 bytes
    PushInteger,
    /// Push an immediate float to the stack
    /// The immediate float is stored in the next 8 bytes
    PushFloat,
    /// Push an immediate unsigned integer to the stack
    /// The immediate unsigned integer is stored in the next 8 bytes
    PushUnsignedInteger,
    /// Push an immediate boolean to the stack
    /// The immediate boolean is stored in the next 1 byte
    PushBoolean,
    /// Push a string from the constant pool
    /// The string is directly put in the memory and its pointer is pushed to the stack
    /// The string index is stored in the next 8 bytes
    PushStr,
    //Todo
    PushList,
    /// Push a unit to the stack
    PushUnit,

    //Variables

    /// Get the value of a variable from the stack
    /// The variable index is stored in the next 4 bytes
    Get,
    /// Store the value of a variable to the stack
    /// The variable index is stored in the next 4 bytes
    Store,
    //arrays

}
