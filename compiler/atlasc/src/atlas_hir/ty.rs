use logos::Span;
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Copy)]
pub struct HirTyId(u64);

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
            HirTy::Boolean(_) => Self::compute_boolean_ty_id(),
            HirTy::Unit(_) => Self::compute_unit_ty_id(),
            HirTy::String(_) => Self::compute_str_ty_id(),
            HirTy::List(ty) => HirTyId::compute_list_ty_id(&HirTyId::from(ty.ty)),
            HirTy::_Named(ty) => HirTyId::compute_name_ty_id(ty.name),
            HirTy::Uninitialized(_) => Self::compute_uninitialized_ty_id(),
            HirTy::_Function(f) => {
                let parameters = f.params.iter().map(HirTyId::from).collect::<Vec<_>>();
                let ret_ty = HirTyId::from(f.ret_ty);
                HirTyId::compute_function_ty_id(&ret_ty, &parameters)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum HirTy<'hir> {
    Int64(HirIntegerTy),
    Float64(HirFloatTy),
    UInt64(HirUnsignedIntTy),
    Unit(HirUnitTy),
    Boolean(HirBooleanTy),
    String(HirStringTy),
    List(HirListTy<'hir>),
    _Named(HirNamedTy<'hir>),
    Uninitialized(HirUninitializedTy),

    _Function(HirFunctionTy<'hir>),
}

impl fmt::Display for HirTy<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HirTy::Int64(_) => write!(f, "int64"),
            HirTy::Float64(_) => write!(f, "float64"),
            HirTy::UInt64(_) => write!(f, "uint64"),
            HirTy::Unit(_) => write!(f, "unit"),
            HirTy::Boolean(_) => write!(f, "bool"),
            HirTy::String(_) => write!(f, "str"),
            HirTy::List(ty) => write!(f, "[{}]", ty),
            HirTy::_Named(ty) => write!(f, "{}", ty.name),
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

#[derive(Debug, Clone, Serialize)]
pub struct HirListTy<'hir> {
    pub ty: &'hir HirTy<'hir>,
}
impl fmt::Display for HirListTy<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)
    }
}

// all the types should hold a span
#[derive(Debug, Clone, Serialize)]
pub struct HirUninitializedTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirIntegerTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirFloatTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirUnsignedIntTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirUnitTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirBooleanTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirStringTy {}

#[derive(Debug, Clone, Serialize)]
pub struct HirFunctionTy<'hir> {
    pub ret_ty: &'hir HirTy<'hir>,
    pub params: Vec<HirTy<'hir>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirNamedTy<'hir> {
    pub name: &'hir str,
    /// Span of the name declaration.
    pub span: Span,
}
