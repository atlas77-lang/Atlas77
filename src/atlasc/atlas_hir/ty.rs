use logos::Span;
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Copy)]
pub struct HirTyId(u64);

// All the "magic" numbers should be replaced with constants.
// e.g. 0x00 -> const INTEGER64_MAGIC: u8 = 0x00;
impl HirTyId {
    pub fn compute_integer64_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x00.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_float64_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x01.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_uint64_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x02.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_boolean_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x03.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_unit_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x04.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_char_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x05.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_str_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x10.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_function_ty_id(ret_ty: &HirTyId, params: &[HirTyId]) -> Self {
        let mut hasher = DefaultHasher::new();

        (0x20, ret_ty, params).hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_list_ty_id(ty: &HirTyId) -> Self {
        let mut hasher = DefaultHasher::new();
        (0x30, ty).hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_uninitialized_ty_id() -> Self {
        let mut hasher = DefaultHasher::new();
        0x50.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn compute_name_ty_id(name: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        (0x10, name).hash(&mut hasher);
        Self(hasher.finish())
    }
}

impl<'hir> From<&'hir HirTy<'hir>> for HirTyId {
    fn from(value: &'hir HirTy<'hir>) -> Self {
        match value {
            HirTy::Int64(_) => Self::compute_integer64_ty_id(),
            HirTy::Float64(_) => Self::compute_float64_ty_id(),
            HirTy::UInt64(_) => Self::compute_uint64_ty_id(),
            HirTy::Char(_) => Self::compute_char_ty_id(),
            HirTy::Boolean(_) => Self::compute_boolean_ty_id(),
            HirTy::Unit(_) => Self::compute_unit_ty_id(),
            HirTy::String(_) => Self::compute_str_ty_id(),
            HirTy::List(ty) => HirTyId::compute_list_ty_id(&HirTyId::from(ty.inner)),
            HirTy::Named(ty) => HirTyId::compute_name_ty_id(ty.name),
            HirTy::Uninitialized(_) => Self::compute_uninitialized_ty_id(),
            HirTy::_Function(f) => {
                let parameters = f.params.iter().map(HirTyId::from).collect::<Vec<_>>();
                let ret_ty = HirTyId::from(f.ret_ty);
                HirTyId::compute_function_ty_id(&ret_ty, &parameters)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub enum HirTy<'hir> {
    Int64(HirIntegerTy),
    Float64(HirFloatTy),
    UInt64(HirUnsignedIntTy),
    Char(HirCharTy),
    Unit(HirUnitTy),
    Boolean(HirBooleanTy),
    String(HirStringTy),
    List(HirListTy<'hir>),
    Named(HirNamedTy<'hir>),
    Uninitialized(HirUninitializedTy),

    _Function(HirFunctionTy<'hir>),
}

impl fmt::Display for HirTy<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HirTy::Int64(_) => write!(f, "int64"),
            HirTy::Float64(_) => write!(f, "float64"),
            HirTy::UInt64(_) => write!(f, "uint64"),
            HirTy::Char(_) => write!(f, "char"),
            HirTy::Unit(_) => write!(f, "unit"),
            HirTy::Boolean(_) => write!(f, "bool"),
            HirTy::String(_) => write!(f, "str"),
            HirTy::List(ty) => write!(f, "[{}]", ty),
            HirTy::Named(ty) => write!(f, "{}", ty.name),
            HirTy::Uninitialized(_) => write!(f, "uninitialized"),
            HirTy::_Function(func) => {
                let params = func
                    .params
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({}) -> {}", params, func.ret_ty)
            }
        }
    }
}

/// The char type is a 32-bit Unicode code point.
///
/// It can be considered as a 4-byte integer.
#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirCharTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirListTy<'hir> {
    pub inner: &'hir HirTy<'hir>,
}
impl fmt::Display for HirListTy<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// all the types should hold a span
#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirUninitializedTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirIntegerTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirFloatTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirUnsignedIntTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirUnitTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirBooleanTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirStringTy {}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirFunctionTy<'hir> {
    pub ret_ty: &'hir HirTy<'hir>,
    pub params: Vec<HirTy<'hir>>,
}

#[derive(Debug, Clone, Serialize, Eq, Hash, PartialEq)]
pub struct HirNamedTy<'hir> {
    pub name: &'hir str,
    /// Span of the name declaration.
    pub span: Span,
}
