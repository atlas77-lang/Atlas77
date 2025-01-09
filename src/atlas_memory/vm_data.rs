use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use super::object_map::ObjectIndex;

/// A way to represent ``&fn`` should be better implemented honestly, as they can have all sort of the types,
/// and you should, technically, be able to store `&extern fn` and used them as normal `&fn`
#[derive(Clone, Copy)]
pub union RawVMData {
    as_unit: (),
    as_i64: i64,
    as_u64: u64,
    as_f64: f64,
    as_bool: bool,
    as_char: char,
    as_object: ObjectIndex,
    //as_fn_ptr: fn(&[VMData]) -> VMData, //This will be added later for the FFI
    as_fn_ptr: usize,
}

#[derive(Clone, Copy)]
pub struct VMData {
    pub tag: u64,
    data: RawVMData,
}

macro_rules! def_new_vmdata_func {
    ($ident: ident, $field: ident, $ty: ty, $const: ident) => {
        #[inline(always)]
        pub fn $ident(val: $ty) -> Self {
            Self::new(Self::$const, RawVMData { $field: val })
        }
    };
}

impl VMData {
    pub const TAG_UNIT: u64 = 0;
    pub const TAG_U64: u64 = 4;
    pub const TAG_I64: u64 = 8;
    pub const TAG_FLOAT: u64 = 9;
    pub const TAG_BOOL: u64 = 10;
    pub const TAG_STR: u64 = 11;
    pub const TAG_CHAR: u64 = 12;
    pub const TAG_FN_PTR: u64 = 13;

    pub fn new(tag: u64, data: RawVMData) -> Self {
        Self { tag, data }
    }

    pub fn new_unit() -> Self {
        Self {
            tag: Self::TAG_UNIT,
            data: RawVMData { as_unit: () },
        }
    }

    pub fn new_object(tag: u64, val: ObjectIndex) -> Self {
        assert!(tag > 256, "object typeid is within the reserved area");
        Self {
            tag,
            data: RawVMData { as_object: val },
        }
    }

    pub fn new_string(val: ObjectIndex) -> Self {
        Self::new(Self::TAG_STR, RawVMData { as_object: val })
    }

    pub fn new_list(tag: u64, val: ObjectIndex) -> Self {
        assert!(tag > 256, "object typeid is within the reserved area");
        Self {
            tag,
            data: RawVMData { as_object: val },
        }
    }

    def_new_vmdata_func!(new_i64, as_i64, i64, TAG_I64);
    def_new_vmdata_func!(new_u64, as_u64, u64, TAG_U64);
    def_new_vmdata_func!(new_f64, as_f64, f64, TAG_FLOAT);
    def_new_vmdata_func!(new_bool, as_bool, bool, TAG_BOOL);
    def_new_vmdata_func!(new_char, as_char, char, TAG_CHAR);
    def_new_vmdata_func!(new_fn_ptr, as_fn_ptr, usize, TAG_FN_PTR);
}

impl PartialEq for VMData {
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
            Self::TAG_UNIT => true,
            _ if self.tag > 256 => self.as_object() == other.as_object(),
            _ => panic!("Illegal comparison"),
        }
    }
}

impl PartialOrd for VMData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.tag != other.tag {
            return None;
        }
        match self.tag {
            Self::TAG_FLOAT => self.as_f64().partial_cmp(&other.as_f64()),
            Self::TAG_U64 => self.as_u64().partial_cmp(&other.as_u64()),
            Self::TAG_I64 => self.as_i64().partial_cmp(&other.as_i64()),
            Self::TAG_CHAR => self.as_char().partial_cmp(&other.as_char()),
            _ => panic!("Illegal comparison"),
        }
    }
}

impl Add for VMData {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() + other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() + other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() + other.as_f64()),
            _ => panic!("Illegal addition"),
        }
    }
}

impl Sub for VMData {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() - other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() - other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() - other.as_f64()),
            _ => panic!("Illegal subtraction"),
        }
    }
}

impl Mul for VMData {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() * other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() * other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() * other.as_f64()),
            _ => panic!("Illegal multiplication"),
        }
    }
}

impl Div for VMData {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() / other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() / other.as_u64()),
            (Self::TAG_FLOAT, Self::TAG_FLOAT) => Self::new_f64(self.as_f64() / other.as_f64()),
            _ => panic!("Illegal division"),
        }
    }
}

impl Rem for VMData {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self.tag, other.tag) {
            (Self::TAG_I64, Self::TAG_I64) => Self::new_i64(self.as_i64() % other.as_i64()),
            (Self::TAG_U64, Self::TAG_U64) => Self::new_u64(self.as_u64() % other.as_u64()),
            _ => panic!("Illegal remainder"),
        }
    }
}

impl Display for VMData {
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
                Self::TAG_CHAR => self.as_char().to_string(),
                Self::TAG_FN_PTR => format!("fn_ptr: {}", self.as_fn_ptr()),
                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for VMData {
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
                Self::TAG_FN_PTR => "fn_ptr",
                _ if self.is_object() => "obj",
                _ => "res",
            },
            match self.tag {
                Self::TAG_UNIT => "()".to_string(),
                Self::TAG_I64 => self.as_i64().to_string(),
                Self::TAG_U64 => self.as_u64().to_string(),
                Self::TAG_FLOAT => self.as_f64().to_string(),
                Self::TAG_BOOL => self.as_bool().to_string(),
                Self::TAG_CHAR => self.as_char().to_string(),
                Self::TAG_FN_PTR => format!("fn_ptr: {}", self.as_fn_ptr()),
                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
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

impl VMData {
    enum_variant_function!(as_i64, is_i64, TAG_I64, i64);
    enum_variant_function!(as_f64, is_f64, TAG_FLOAT, f64);
    enum_variant_function!(as_u64, is_u64, TAG_U64, u64);
    enum_variant_function!(as_bool, is_bool, TAG_BOOL, bool);
    enum_variant_function!(as_char, is_char, TAG_CHAR, char);
    enum_variant_function!(as_unit, is_unit, TAG_UNIT, ());
    enum_variant_function!(as_fn_ptr, is_fn_ptr, TAG_FN_PTR, usize);

    #[inline(always)]
    #[must_use]
    pub fn is_object(self) -> bool {
        self.tag > 256 || self.tag == Self::TAG_STR
    }

    #[inline(always)]
    #[must_use]
    pub fn as_object(self) -> ObjectIndex {
        if !self.is_object() {
            unreachable!()
        }

        unsafe { self.data.as_object }
    }
}
