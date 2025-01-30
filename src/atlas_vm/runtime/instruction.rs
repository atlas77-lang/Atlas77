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
    Char,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Instruction<'run> {
    PushInt(i64),
    PushFloat(f64),
    PushUnsignedInt(u64),
    PushBool(bool),
    PushChar(char),
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
    Swap,
    Dup,

    Store {
        var_name: &'run str,
    },

    Load {
        var_name: &'run str,
    },

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
    /// Create a new list, the size is the top of the stack
    NewList,
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
        name: &'run str,
        args: u8,
    },
    ExternCall {
        name: &'run str,
        args: u8,
    },
    Return,

    Halt,
}

/// Read by the VM before execution to import the related functions
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ImportedLibrary {
    pub name: String,
    pub is_std: bool,
}

///todo: Make the program serializable and deserializable
/// This will allow the program to be saved and loaded from a file
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Program<'run> {
    pub labels: Vec<Label<'run>>,
    pub entry_point: String,
    pub libraries: Vec<ImportedLibrary>,
    pub global: ConstantPool,
}

impl<'run> Index<usize> for Program<'run> {
    type Output = Instruction<'run>;

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
impl Default for Program<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl Program<'_> {
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
                list_pool: vec![],
                function_pool: vec![],
            },
            libraries: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ConstantPool {
    pub string_pool: Vec<String>,
    pub list_pool: Vec<Constant>,
    pub function_pool: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
//Temporary solution to the constant pool
pub enum Constant {
    String(String),
    List(Vec<Constant>),
    Integer(i64),
    Float(f64),
    UnsignedInteger(u64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Label<'run> {
    pub name: String,
    pub position: usize,
    pub body: Vec<Instruction<'run>>,
}
