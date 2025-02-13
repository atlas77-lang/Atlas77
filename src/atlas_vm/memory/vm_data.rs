use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use super::object_map::{Object, ObjectIndex};

#[derive(Clone, Copy)]
pub union RawVMData<'vm> {
    as_unit: (),
    as_i64: i64,
    as_u64: u64,
    as_f64: f64,
    as_bool: bool,
    as_char: char,
    /// Null value
    as_none: (),
    /// Pointer to a value in the stack
    as_stack_ptr: usize,
    /// Pointer to a function
    as_fn_ptr: usize,
    /// Pointer to an object in the object map
    as_object: ObjectIndex,
    as_raw_ptr: &'vm mut Object<'vm>,
}


pub struct VMData<'vm> {
    pub tag: u8,
    data: RawVMData<'vm>,
}
impl<'vm> Copy for VMData<'vm> {}
impl<'vm> Clone for VMData<'vm> {
    fn clone(&self) -> Self {
        if self.tag == Self::TAG_RAW_OBJECT {
            unreachable!("You can't copy/clone a mutable pointer.")
        }
        *self
    }
}

macro_rules! def_new_vm_data_func {
    ($ident: ident, $field: ident, $ty: ty, $const: ident) => {
        #[inline(always)]
        pub fn $ident(val: $ty) -> Self {
            Self::new(Self::$const, RawVMData { $field: val })
        }
    };
}

impl<'vm> VMData<'vm> {
    pub const TAG_UNIT: u8 = 0;
    pub const TAG_NONE: u8 = 1;
    pub const TAG_U64: u8 = 4;
    pub const TAG_I64: u8 = 8;
    pub const TAG_FLOAT: u8 = 9;
    pub const TAG_BOOL: u8 = 10;
    pub const TAG_STR: u8 = 11;
    pub const TAG_CHAR: u8 = 12;
    pub const TAG_STACK_PTR: u8 = 13;
    pub const TAG_FN_PTR: u8 = 14;
    pub const TAG_LIST: u8 = 15;
    pub const TAG_OBJECT: u8 = 16;
    pub const TAG_RAW_OBJECT: u8 = 17;

    pub fn new(tag: u8, data: RawVMData<'vm>) -> Self {
        Self { tag, data }
    }

    pub fn new_unit() -> Self {
        Self {
            tag: Self::TAG_UNIT,
            data: RawVMData { as_unit: () },
        }
    }
    pub fn new_none() -> Self {
        Self {
            tag: Self::TAG_NONE,
            data: RawVMData { as_none: () },
        }
    }

    pub fn new_object(val: ObjectIndex) -> Self {
        Self {
            tag: Self::TAG_OBJECT,
            data: RawVMData { as_object: val },
        }
    }

    pub fn new_string(val: ObjectIndex) -> Self {
        Self {
            tag: Self::TAG_STR,
            data: RawVMData { as_object: val },
        }
    }

    pub fn new_list(val: ObjectIndex) -> Self {
        Self {
            tag: Self::TAG_LIST,
            data: RawVMData { as_object: val },
        }
    }

    def_new_vm_data_func!(new_i64, as_i64, i64, TAG_I64);
    def_new_vm_data_func!(new_u64, as_u64, u64, TAG_U64);
    def_new_vm_data_func!(new_f64, as_f64, f64, TAG_FLOAT);
    def_new_vm_data_func!(new_bool, as_bool, bool, TAG_BOOL);
    def_new_vm_data_func!(new_char, as_char, char, TAG_CHAR);
    def_new_vm_data_func!(new_stack_ptr, as_stack_ptr, usize, TAG_STACK_PTR);
    def_new_vm_data_func!(new_fn_ptr, as_fn_ptr, usize, TAG_FN_PTR);
    def_new_vm_data_func!(new_raw_object, as_raw_ptr, &'vm mut Object<'vm>, TAG_RAW_OBJECT);
}

impl PartialEq for VMData<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.tag != other.tag {
            return false;
        }
        match self.tag {
            Self::TAG_BOOL => self.as_bool() == other.as_bool(),
            Self::TAG_FLOAT => self.as_f64() == other.as_f64(),
            Self::TAG_I64 => self.as_i64() == other.as_i64(),
            Self::TAG_U64 => self.as_u64() == other.as_u64(),
            Self::TAG_CHAR => self.as_char() == other.as_char(),
            Self::TAG_UNIT | Self::TAG_NONE => true,
            // comparison based on pointer and not inner data
            _ if self.is_object() => self.as_object() == other.as_object(),
            _ => panic!("Illegal comparison between {:?} and {:?}", self, other),
        }
    }
}

impl PartialOrd for VMData<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.tag != other.tag {
            return None;
        }
        match self.tag {
            Self::TAG_FLOAT => self.as_f64().partial_cmp(&other.as_f64()),
            Self::TAG_U64 => self.as_u64().partial_cmp(&other.as_u64()),
            Self::TAG_I64 => self.as_i64().partial_cmp(&other.as_i64()),
            Self::TAG_CHAR => self.as_char().partial_cmp(&other.as_char()),
            _ => panic!("Illegal comparison between {:?} and {:?}", self, other),
        }
    }
}

impl Add for VMData<'_> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() + other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() + other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() + other.as_f64()),
            _ => panic!("Illegal addition between {:?} and {:?}", self, other),
        }
    }
}

impl Sub for VMData<'_> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() - other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() - other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() - other.as_f64()),
            _ => panic!("Illegal subtraction between {:?} and {:?}", self, other),
        }
    }
}

impl Mul for VMData<'_> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() * other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() * other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() * other.as_f64()),
            _ => panic!("Illegal multiplication between {:?} and {:?}", self, other),
        }
    }
}

impl Div for VMData<'_> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() / other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() / other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() / other.as_f64()),
            _ => panic!("Illegal division between {:?} and {:?}", self, other),
        }
    }
}

impl Rem for VMData<'_> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() % other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() % other.as_u64()),
            _ => panic!("Illegal remainder between {:?} and {:?}", self, other),
        }
    }
}

impl Display for VMData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.tag {
                Self::TAG_UNIT => "()".to_string(),
                Self::TAG_I64 => self.as_i64().to_string(),
                Self::TAG_U64 => self.as_u64().to_string(),
                Self::TAG_FLOAT => self.as_f64().to_string(),
                Self::TAG_BOOL => self.as_bool().to_string(),
                Self::TAG_CHAR => format!("'{}'", self.as_char()),
                Self::TAG_STACK_PTR => self.as_stack_ptr().to_string(),
                Self::TAG_FN_PTR => self.as_fn_ptr().to_string(),
                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for VMData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VMData {{ tag: {}({}), data: {}}}",
            self.tag,
            match self.tag {
                Self::TAG_BOOL => "bool",
                Self::TAG_UNIT => "unit",
                Self::TAG_FLOAT => "f64",
                Self::TAG_I64 => "i64",
                Self::TAG_U64 => "u64",
                Self::TAG_CHAR => "char",
                Self::TAG_STACK_PTR => "&",
                Self::TAG_FN_PTR => "fn",
                _ if self.is_object() => "obj",
                _ => "res",
            },
            format!("{}", self)
        )
    }
}

macro_rules! enum_variant_function {
    ($getter: ident, $is: ident, $variant: ident, $ty: ty) => {
        #[inline(always)]
        #[must_use]
        pub fn $getter(self) -> $ty {
            unsafe { self.data.$getter }
        }

        #[inline(always)]
        #[must_use]
        pub fn $is(self) -> bool {
            self.tag == Self::$variant
        }
    };
}

impl<'vm> VMData<'vm> {
    enum_variant_function!(as_i64, is_i64, TAG_I64, i64);
    enum_variant_function!(as_f64, is_f64, TAG_FLOAT, f64);
    enum_variant_function!(as_u64, is_u64, TAG_U64, u64);
    enum_variant_function!(as_bool, is_bool, TAG_BOOL, bool);
    enum_variant_function!(as_char, is_char, TAG_CHAR, char);
    enum_variant_function!(as_stack_ptr, is_stack_ptr, TAG_STACK_PTR, usize);
    enum_variant_function!(as_fn_ptr, is_fn_ptr, TAG_FN_PTR, usize);
    //Clippy doesn't like #[must_use] on () return types
    #[inline(always)]
    pub fn as_unit(self) {
        unsafe {
            self.data.as_unit
        }
    }
    #[inline(always)]
    #[must_use]
    pub fn is_unit(self) -> bool {
        self.tag == Self::TAG_UNIT
    }

    #[inline(always)]
    pub fn as_none(self) {
        unsafe {
            self.data.as_none
        }
    }
    #[inline(always)]
    #[must_use]
    pub fn is_none(self) -> bool {
        self.tag == Self::TAG_NONE
    }

    #[inline(always)]
    #[must_use]
    pub fn is_object(self) -> bool {
        self.tag == Self::TAG_OBJECT || self.tag == Self::TAG_LIST || self.tag == Self::TAG_STR
    }

    #[inline(always)]
    #[must_use]
    pub fn as_object(self) -> ObjectIndex {
        if !self.is_object() {
            unreachable!("Attempted to get an object from a non-object VMData {:?}", self);
        }

        unsafe { self.data.as_object }
    }

    pub fn is_raw_object(self) -> bool {
        self.tag == Self::TAG_RAW_OBJECT
    }

    pub fn as_raw_object(self) -> &'vm mut Object<'vm> {
        if !self.is_raw_object() {
            unreachable!("Attempted to get an object from a non-raw pointer VMData {:?}", self);
        }
        unsafe { self.data.as_raw_ptr }
    }
}
