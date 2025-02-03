//NB: This is a dumb down version of the instruction set.
//A more powerful version will be done for the v0.5.2 & v0.5.3

use std::collections::BTreeMap;
use std::ops::Index;

use crate::atlas_c::atlas_hir::signature::ConstantValue;
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
    /// - **Bottom** `[ListPointer, Index,]` **Top**
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
    /// Stack state:
    ///
    /// - **Bottom** `[StrPointer, Index,]` **Top**
    ///
    /// Load a value from a str to the top of the stack
    StringLoad,
    /// Stack state:
    ///
    /// - **Bottom** `[StrPointer, Index, Value,]` **Top**
    ///
    /// Store a value in a given str
    StringStore,

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
        nb_args: u8,
    },

    FunctionCall {
        function_name: &'run str,
        nb_args: u8,
    },
    ExternCall {
        function_name: &'run str,
        nb_args: u8,
    },
    Return,

    /// Delete the object from memory (the object pointer is at the top of the stack)
    DeleteObj,
    /// Stack:
    /// - **[ClassPtr,] -> [FieldValue,]**
    GetField {
        field_name: &'run str,
    },
    /// Stack:
    /// - [ClassPtr, Value] -> []
    SetField {
        field_name: &'run str,
    },
    /// Create a new object
    /// The information about the object is in the constant pool
    NewObj {
        class_name: &'run str,
    },
    /// This jumps to the correct position in the program to execute the method
    ///
    /// And creates a `self` variable in the var_map
    MethodCall {
        method_name: &'run str,
        nb_args: u8,
    },
    StaticCall {
        method_name: &'run str,
        nb_args: u8,
    },
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
    pub global: ConstantPool<'run>,
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
                string_pool: &[],
                list_pool: &[],
                function_pool: &[],
                class_pool: &[],
            },
            libraries: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ConstantPool<'run> {
    //todo: Vec<T> -> &'run [T]
    pub string_pool: &'run [&'run str],
    pub list_pool: &'run [ConstantValue],
    pub function_pool: &'run [usize],
    pub class_pool: &'run [ConstantClass<'run>],
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ConstantClass<'run> {
    pub name: &'run str,
    pub fields: Vec<&'run str>,
    pub constructor_nb_args: usize,
    pub constants: BTreeMap<&'run str, ConstantValue>,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Label<'run> {
    pub name: &'run str,
    pub position: usize,
    pub body: &'run [Instruction<'run>],
}
