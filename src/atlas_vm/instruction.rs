//NB: This is a dumb down version of the instruction set.
//A more powerful version will be done for the v0.5.2 & v0.5.3

use std::ops::Index;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum Instruction<'vm> {
    PushInt(i64),
    PushFloat(f64),
    PushUnsignedInt(u64),
    PushString(String),
    PushUnit,

    Pop,

    /// Store an i64 value in a variable from the stack
    StoreI64 {
        var_name: String,
    },
    /// Store an f64 value in a variable from the stack
    StoreF64 {
        var_name: String,
    },
    /// Store an u64 value in a variable from the stack
    StoreU64 {
        var_name: String,
    },

    /// Load an i64 value from a variable to the stack
    LoadI64 {
        var_name: String,
    },
    /// Load an f64 value from a variable to the stack
    LoadF64 {
        var_name: String,
    },
    /// Load an u64 value from a variable to the stack
    LoadU64 {
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

    /// Unconditional jump
    Jmp {
        pos: usize,
    },
    /// Jump if 0
    JmpZ {
        pos: usize,
    },

    CallFunction {
        name: String,
        args: u8,
    },
    ExternCall {
        name: &'vm str,
        args: u8,
    },
    Return,

    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Program<'vm> {
    pub labels: &'vm [&'vm Label<'vm>],
    pub entry_point: &'vm str,
}

impl<'vm> Index<usize> for Program<'vm> {
    type Output = Instruction<'vm>;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current_index = 0;
        for label in self.labels {
            if current_index + label.body.len() > index {
                return &label.body[index - current_index];
            }
            current_index += label.body.len();
        }
        panic!("Index out of bounds");
    }
}

impl Program<'_> {
    pub fn len(&self) -> usize {
        self.labels.iter().map(|label| label.body.len()).sum()
    }
    pub fn new() -> Self {
        Self {
            labels: &[],
            entry_point: "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct ConstantPool<'vm> {
    pub string_pool: &'vm [&'vm str],
    pub function_pool: &'vm [&'vm str],
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Label<'vm> {
    pub name: String,
    pub position: usize,
    pub body: &'vm [&'vm Instruction<'vm>],
}
