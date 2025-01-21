//NB: This is a dumb down version of the instruction set.
//A more powerful version will be done for the v0.5.2 & v0.5.3

use std::ops::Index;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Instruction {
    PushInt(i64),
    PushFloat(f64),
    PushUnsignedInt(u64),
    PushBool(bool),
    PushStr(String),
    PushUnit,

    ///Store will replace StoreI64, StoreF64, StoreU64, StoreBool and will stop using the var_map and favor the stack instead
    Store(usize),
    ///Load will replace LoadI64, LoadF64, LoadU64, LoadBool and will stop using the var_map and favor the stack instead
    Get(usize),

    Pop,

    /// Store an i64 value in a variable from the stack
    StoreI64 {
        var_name: String,
    },
    /// Store a f64 value in a variable from the stack
    StoreF64 {
        var_name: String,
    },
    /// Store an u64 value in a variable from the stack
    StoreU64 {
        var_name: String,
    },
    StoreBool {
        var_name: String,
    },

    /// Load an i64 value from a variable to the stack
    LoadI64 {
        var_name: String,
    },
    /// Load a f64 value from a variable to the stack
    LoadF64 {
        var_name: String,
    },
    /// Load an u64 value from a variable to the stack
    LoadU64 {
        var_name: String,
    },
    LoadBool {
        var_name: String,
    },

    //Math
    AddI64,
    AddF64,
    AddU64,

    SubI64,
    SubF64,
    SubU64,

    MulI64,
    MulF64,
    MulU64,

    DivI64,
    DivF64,
    DivU64,

    ModI64,
    ModF64,
    ModU64,

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
