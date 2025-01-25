//NB: This is a dumb down version of the instruction set.
//A more powerful version will be done for the v0.5.2 & v0.5.3

use std::ops::Index;

use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Type {
    Integer,
    Float,
    UnsignedInteger,
    Boolean,
    String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Instruction {
    PushInt(i64),
    PushFloat(f64),
    PushUnsignedInt(u64),
    PushBool(bool),
    /// Push a string from the constant pool
    /// The string is directly put in the memory and its pointer is pushed to the stack
    PushStr(usize),
    /// Push a list from the constant pool
    /// The list is directly put in the memory and its pointer is pushed to the stack
    PushList(usize),
    PushUnit,

    ///Store will replace StoreInteger, StoreFloat, StoreUnsignedInteger, StoreBool and will stop using the var_map and favor the stack instead
    //Store(usize),
    ///Load will replace LoadInteger, LoadFloat, LoadUnsignedInteger, LoadBool and will stop using the var_map and favor the stack instead
    Get(usize),

    Pop,

    Store {
        var_name: String,
    },

    Load {
        var_name: String,
    },

    ListIndex(usize),

    CastTo(Type),
    //Math
    IAdd,
    FAdd,
    UIAdd,

    ISub,
    FSub,
    UISub,

    IMul,
    FMul,
    UIMul,

    IDiv,
    FDiv,
    UIDiv,

    IMod,
    FMod,
    UIMod,

    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,

    /// Relative unconditional jump
    Jmp {
        pos: isize,
    },
    /// Relative Jump if the top of the stack value is eq to 0
    JmpZ {
        pos: isize,
    },

    /// Call a function by taking the value at `pos` in the stack as the fn_ptr
    DirectCall {
        pos: usize,
        args: u8,
    },
    /// Call a function by taking the top of the stack value as the fn_ptr
    Call {
        args: u8,
    },

    CallFunction {
        name: String,
        args: u8,
    },
    ExternCall {
        name: String,
        args: u8,
    },
    Return,

    Halt,
}

/// Read by the VM before execution to import the related functions
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ImportedLibrary {
    pub name: String,
    pub is_std: bool,
}

///todo: Make the program serializable and deserializable
/// This will allow the program to be saved and loaded from a file
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Program {
    pub labels: Vec<Label>,
    pub entry_point: String,
    pub libraries: Vec<ImportedLibrary>,
    pub global: ConstantPool,
}

impl Index<usize> for Program {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current_index = 0;
        for label in &self.labels {
            if current_index + label.body.len() > index {
                return &label.body[index - current_index];
            }
            current_index += label.body.len();
        }
        panic!("Index out of bounds");
    }
}
impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
impl Program {
    pub fn len(&self) -> usize {
        self.labels.iter().map(|label| label.body.len()).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn new() -> Self {
        Self {
            labels: vec![],
            entry_point: String::new(),
            global: ConstantPool {
                string_pool: vec![],
                function_pool: vec![],
            },
            libraries: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ConstantPool {
    pub string_pool: Vec<String>,
    pub function_pool: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub position: usize,
    pub body: Vec<Instruction>,
}
