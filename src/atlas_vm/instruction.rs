#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
        var_name: &'vm str,
    },
    /// Store an u64 value in a variable from the stack
    StoreU64 {
        var_name: &'vm str,
    },

    /// Load an i64 value from a variable to the stack
    LoadI64 {
        var_name: String,
    },
    /// Load an f64 value from a variable to the stack
    LoadF64 {
        var_name: &'vm str,
    },
    /// Load an u64 value from a variable to the stack
    LoadU64 {
        var_name: &'vm str,
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

    CallFunction {
        name: &'vm str,
        args: u8,
    },
    ExternCall {
        name: &'vm str,
        args: u8,
    },
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Program<'vm> {
    pub labels: &'vm [Label<'vm>],
    pub entry_point: &'vm str,
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstantPool<'vm> {
    pub string_pool: &'vm [&'vm str],
    pub function_pool: &'vm [&'vm str],
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Label<'vm> {
    pub name: &'vm str,
    pub position: usize,
    pub body: &'vm [Instruction<'vm>],
}
