use core::fmt;

use crate::atlas_frontend::parser::ast::{BinaryOperator, UnaryOperator};
use atlas_core::prelude::{Span, Spanned};
use internment::Intern;

pub type HlirTree = Vec<HlirExpr>;

pub type TypeID = u64;
pub type IdentID = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum HlirLiteral {
    Unit,
    Int(i64),
    Float(f64),
    UInt(u64),
    String(Intern<String>),
    Bool(bool),
    List(Vec<HlirExpr>),
}

impl fmt::Display for HlirLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::UInt(u) => write!(f, "{}", u),
            Self::Unit => write!(f, "()"),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::List(l) => write!(
                f,
                "[{}]",
                l.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum HlirStatement {
    HlirVarDecl(HlirVarDecl),
    HlirExpr(HlirExpr),
    Return(HlirExpr),
}

impl Spanned for HlirStatement {
    fn span(&self) -> Span {
        match self {
            Self::HlirVarDecl(v) => v.span(),
            Self::HlirExpr(e) => e.span(),
            Self::Return(e) => e.span(),
        }
    }
}

impl fmt::Display for HlirStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HlirVarDecl(v) => write!(f, "{};", v),
            Self::HlirExpr(e) => write!(f, "{};", e),
            Self::Return(e) => write!(f, "return {};", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirStructDecl {
    pub name: IdentID,
    pub fields: Vec<TypeID>,
    pub span: Span,
}

impl Spanned for HlirStructDecl {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirStructDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "struct {} ({})",
            self.name,
            self.fields
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(",\n\t")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirVarDecl {
    pub name: IdentID,
    pub t: TypeID,
    pub value: Box<HlirExpr>,
    pub span: Span,
}
impl Spanned for HlirVarDecl {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "let {}: {} = {}", self.name, self.t, self.clone().value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirFunExpr {
    pub args: Vec<(IdentID, TypeID)>,
    pub body: Box<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirFunExpr {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirFunExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirIdentifier {
    pub name: IdentID,
    pub span: Span,
}

impl Spanned for HlirIdentifier {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirBinaryExp {
    pub left: Box<HlirExpr>,
    pub operator: HlirBinOp,
    pub right: Box<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirBinaryExp {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirBinaryExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HlirBinOp {
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpEq,
    OpNe,
    OpLt,
    OpLe,
    OpGt,
    OpGe,
    OpAnd,
    OpOr,
}

impl fmt::Display for HlirBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpMul => write!(f, "*"),
            Self::OpDiv => write!(f, "/"),
            Self::OpMod => write!(f, "%"),
            Self::OpEq => write!(f, "=="),
            Self::OpNe => write!(f, "!="),
            Self::OpLt => write!(f, "<"),
            Self::OpLe => write!(f, "<="),
            Self::OpGt => write!(f, ">"),
            Self::OpGe => write!(f, ">="),
            Self::OpAnd => write!(f, "&&"),
            Self::OpOr => write!(f, "||"),
        }
    }
}

impl From<&BinaryOperator> for HlirBinOp {
    fn from(value: &BinaryOperator) -> Self {
        match value {
            BinaryOperator::OpAdd => HlirBinOp::OpAdd,
            BinaryOperator::OpSub => HlirBinOp::OpSub,
            BinaryOperator::OpMul => HlirBinOp::OpMul,
            BinaryOperator::OpDiv => HlirBinOp::OpDiv,
            BinaryOperator::OpMod => HlirBinOp::OpMod,
            BinaryOperator::OpEq => HlirBinOp::OpEq,
            BinaryOperator::OpNe => HlirBinOp::OpNe,
            BinaryOperator::OpLt => HlirBinOp::OpLt,
            BinaryOperator::OpLe => HlirBinOp::OpLe,
            BinaryOperator::OpGt => HlirBinOp::OpGt,
            BinaryOperator::OpGe => HlirBinOp::OpGe,
            BinaryOperator::OpAnd => HlirBinOp::OpAnd,
            BinaryOperator::OpOr => HlirBinOp::OpOr,
        }
    }
}

/// Unary expression
#[derive(Debug, Clone, PartialEq)]
pub struct HlirUnary {
    pub operator: Option<HlirUnaryOp>,
    pub expression: Box<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirUnary {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirUnary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.operator.is_some() {
            write!(f, "{} {}", self.operator.clone().unwrap(), self.expression)
        } else {
            write!(f, "{}", self.expression)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HlirUnaryOp {
    OpSub,
    OpNot,
}

impl fmt::Display for HlirUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpSub => write!(f, "-"),
            Self::OpNot => write!(f, "!"),
        }
    }
}

impl From<&UnaryOperator> for HlirUnaryOp {
    fn from(value: &UnaryOperator) -> Self {
        match value {
            UnaryOperator::OpSub => HlirUnaryOp::OpSub,
            UnaryOperator::OpNot => HlirUnaryOp::OpNot,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirIfElseExpr {
    pub condition: Box<HlirExpr>,
    pub if_body: Box<HlirExpr>,
    pub else_body: Option<Box<HlirExpr>>,
    pub span: Span,
}

impl Spanned for HlirIfElseExpr {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirIfElseExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(e) = &self.else_body {
            write!(
                f,
                "if {}then\n\t{}else\n\t{}",
                self.condition, self.if_body, e
            )
        } else {
            write!(f, "if {} then\n\t{}", self.condition, self.if_body)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirFunCall {
    pub name: IdentID,
    pub args: Vec<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirFunCall {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirFunCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirIndexExpr {
    pub name: IdentID,
    pub index: Box<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirIndexExpr {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirIndexExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.name, self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirDoExpr {
    pub body: Vec<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirDoExpr {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirDoExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "do\n\t{}",
            self.body
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirMatchArm {
    pub pattern: Box<HlirExpr>,
    pub body: Box<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirMatchArm {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirMatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirMatchExpr {
    pub expr: Box<HlirExpr>,
    pub arms: Vec<HlirMatchArm>,
    pub default: Option<Box<HlirExpr>>,
    pub span: Span,
}

impl Spanned for HlirMatchExpr {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirMatchExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.default.is_some() {
            write!(
                f,
                "match {} \n\t{}default\n\t{}",
                self.expr,
                self.arms
                    .iter()
                    .map(|a| a.pattern.to_string() + "=>" + &a.body.to_string())
                    .collect::<Vec<String>>()
                    .join("\n\t"),
                self.default.clone().unwrap()
            )
        } else {
            write!(
                f,
                "match {} \n\t{}",
                self.expr,
                self.arms
                    .iter()
                    .map(|a| a.pattern.to_string() + "=>" + &a.body.to_string())
                    .collect::<Vec<String>>()
                    .join("\n\t")
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirNewObjExpr {
    pub name: IdentID,
    pub fields: Vec<HlirExpr>,
    pub span: Span,
}

impl Spanned for HlirNewObjExpr {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirNewObjExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "new {} {{\n\t{}\n}}",
            self.name,
            self.fields
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(",\n\t")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HlirFieldAccess {
    pub name: IdentID,
    pub field: usize, //currently accessing field is through index (like a tuple)
    pub span: Span,
}

impl Spanned for HlirFieldAccess {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for HlirFieldAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.field)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HlirExpr {
    HlirLiteral(HlirLiteral),
    HlirIdent(HlirIdentifier),
    HlirBinaryExp(HlirBinaryExp),
    HlirUnary(HlirUnary),
    HlirIfElseExpr(HlirIfElseExpr),
    HlirFunExpr(HlirFunExpr),
    HlirVarDecl(HlirVarDecl),
    HlirStructDecl(HlirStructDecl),
    HlirFunCall(HlirFunCall),
    HlirDoExpr(HlirDoExpr),
    HlirMatchExpr(HlirMatchExpr),
    HlirIndexExpr(HlirIndexExpr),
    HlirFieldAccess(HlirFieldAccess),
    HlirNewObjExpr(HlirNewObjExpr),
}

impl Spanned for HlirExpr {
    fn span(&self) -> Span {
        match self {
            Self::HlirIdent(i) => i.span(),
            Self::HlirBinaryExp(b) => b.span(),
            Self::HlirUnary(u) => u.span(),
            Self::HlirIfElseExpr(i) => i.span(),
            Self::HlirFunExpr(fun) => fun.span(),
            Self::HlirVarDecl(v) => v.span(),
            Self::HlirFunCall(fun) => fun.span(),
            Self::HlirDoExpr(d) => d.span(),
            Self::HlirMatchExpr(m) => m.span(),
            Self::HlirIndexExpr(l) => l.span(),
            Self::HlirNewObjExpr(n) => n.span(),
            Self::HlirFieldAccess(f) => f.span(),
            _ => unreachable!("HlirLiteral has no span"),
        }
    }
}

impl fmt::Display for HlirExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HlirLiteral(l) => write!(f, "{}", l),
            Self::HlirIdent(i) => write!(f, "{}", i),
            Self::HlirBinaryExp(b) => write!(f, "{}", b),
            Self::HlirUnary(u) => write!(f, "{}", u),
            Self::HlirIfElseExpr(i) => write!(f, "{}", i),
            Self::HlirFunExpr(fun) => write!(f, "{}", fun),
            Self::HlirVarDecl(v) => write!(f, "{}", v),
            Self::HlirFunCall(fun) => write!(f, "{}", fun),
            Self::HlirDoExpr(d) => write!(f, "{}", d),
            Self::HlirMatchExpr(m) => write!(f, "{}", m),
            Self::HlirIndexExpr(l) => write!(f, "{}", l),
            Self::HlirStructDecl(s) => write!(f, "{}", s),
            Self::HlirNewObjExpr(n) => write!(f, "{}", n),
            Self::HlirFieldAccess(fa) => write!(f, "{}", fa),
        }
    }
}
