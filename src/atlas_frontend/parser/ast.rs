use core::fmt;

use atlas_core::prelude::{Span, Spanned};
use internment::Intern;

use crate::atlas_frontend::lexer::TokenKind;

pub type AbstractSyntaxTree = Vec<Expression>;

/// Literal
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Unit,
    String(Intern<String>),
    Bool(bool),
    List(Vec<Expression>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
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
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Return(Expression),
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Self::VariableDeclaration(v) => v.span(),
            Self::Expression(e) => e.span(),
            Self::Return(e) => e.span(),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableDeclaration(v) => write!(f, "{};", v),
            Self::Expression(e) => write!(f, "{};", e),
            Self::Return(e) => write!(f, "return {};", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Integer,
    Float,
    String,
    Bool,
    Unit,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function(Vec<(Intern<String>, Type)>, Box<Type>),
    NonPrimitive(Intern<String>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "i64"),
            Self::Float => write!(f, "f64"),
            Self::String => write!(f, "string"),
            Self::Bool => write!(f, "bool"),
            Self::Unit => write!(f, "()"),
            Self::List(t) => write!(f, "List[{}]", t),
            Self::Map(k, v) => write!(f, "Map[{}, {}]", k, v),
            Self::Function(args, ret) => write!(
                f,
                "({}) -> {}",
                args.iter()
                    .map(|(s, t)| format!("{}: {}", s, t))
                    .collect::<Vec<String>>()
                    .join(", "),
                ret
            ),
            Self::NonPrimitive(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub name: Intern<String>,
    pub fields: Vec<Type>,
    pub span: Span,
}

impl Spanned for StructDeclaration {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for StructDeclaration {
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
pub struct VariableDeclaration {
    pub name: Intern<String>,
    pub t: Type,
    pub mutable: bool,
    pub value: Option<Box<Expression>>,
    pub span: Span,
}
impl Spanned for VariableDeclaration {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_some() {
            writeln!(
                f,
                "let {}{}: {} = {}",
                if self.mutable { "mut " } else { "" },
                self.name,
                self.t,
                self.clone().value.unwrap()
            )
        } else {
            writeln!(
                f,
                "let {}{}: {}",
                if self.mutable { "mut " } else { "" },
                self.name,
                self.t
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression {
    pub args: Vec<(Intern<String>, Type)>,
    pub body: Box<Expression>,
    pub span: Span,
}

impl Spanned for FunctionExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for FunctionExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

/// Identifier
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    /// Name of the identifier
    pub name: Intern<String>,
    pub span: Span,
}

impl Spanned for IdentifierNode {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Binary expression
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
    pub span: Span,
}

impl Spanned for BinaryExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
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

impl fmt::Display for BinaryOperator {
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

impl From<&TokenKind> for Option<BinaryOperator> {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::OpAdd => Some(BinaryOperator::OpAdd),
            TokenKind::OpSub => Some(BinaryOperator::OpSub),
            TokenKind::OpMul => Some(BinaryOperator::OpMul),
            TokenKind::OpDiv => Some(BinaryOperator::OpDiv),
            TokenKind::OpMod => Some(BinaryOperator::OpMod),
            TokenKind::OpEq => Some(BinaryOperator::OpEq),
            TokenKind::OpNEq => Some(BinaryOperator::OpNe),
            TokenKind::OpLessThan => Some(BinaryOperator::OpLt),
            TokenKind::OpLessThanEq => Some(BinaryOperator::OpLe),
            TokenKind::OpGreaterThan => Some(BinaryOperator::OpGt),
            TokenKind::OpGreaterThanEq => Some(BinaryOperator::OpGe),
            TokenKind::OpAnd => Some(BinaryOperator::OpAnd),
            TokenKind::OpOr => Some(BinaryOperator::OpOr),
            _ => None,
        }
    }
}

/// Unary expression
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: Option<UnaryOperator>,
    pub expression: Box<Expression>,
    pub span: Span,
}

impl Spanned for UnaryExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.operator.is_some() {
            write!(f, "{} {}", self.operator.clone().unwrap(), self.expression)
        } else {
            write!(f, "{}", self.expression)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    OpSub,
    OpNot,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpSub => write!(f, "-"),
            Self::OpNot => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseNode {
    pub condition: Box<Expression>,
    pub if_body: Box<Expression>,
    pub else_body: Option<Box<Expression>>,
    pub span: Span,
}

impl Spanned for IfElseNode {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for IfElseNode {
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
pub struct FunctionCall {
    pub name: Intern<String>,
    pub args: Vec<Box<Expression>>,
    pub span: Span,
}

impl Spanned for FunctionCall {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for FunctionCall {
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
pub struct IndexExpression {
    pub name: Intern<String>,
    pub index: Box<Expression>,
    pub span: Span,
}

impl Spanned for IndexExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.name, self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoExpression {
    pub body: Vec<Box<Expression>>,
    pub span: Span,
}

impl Spanned for DoExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for DoExpression {
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
pub struct MatchArm {
    pub pattern: Box<Expression>,
    pub body: Box<Expression>,
    pub span: Span,
}

impl Spanned for MatchArm {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpression {
    pub expr: Box<Expression>,
    pub arms: Vec<MatchArm>,
    pub default: Option<Box<Expression>>,
    pub span: Span,
}

impl Spanned for MatchExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for MatchExpression {
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
pub struct NewObjectExpression {
    pub name: Intern<String>,
    pub fields: Vec<Expression>,
    pub span: Span,
}

impl Spanned for NewObjectExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for NewObjectExpression {
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
pub struct FieldAccessExpression {
    pub name: Intern<String>,
    pub field: usize, //currently accessing field is through index (like a tuple)
    pub span: Span,
}

impl Spanned for FieldAccessExpression {
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for FieldAccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.field)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(IdentifierNode),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    IfElseNode(IfElseNode),
    FunctionExpression(FunctionExpression),
    VariableDeclaration(VariableDeclaration),
    StructDeclaration(StructDeclaration),
    FunctionCall(FunctionCall),
    DoExpression(DoExpression),
    MatchExpression(MatchExpression),
    IndexExpression(IndexExpression),
    FieldAccessExpression(FieldAccessExpression),
    NewObjectExpression(NewObjectExpression),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(i) => i.span(),
            Self::BinaryExpression(b) => b.span(),
            Self::UnaryExpression(u) => u.span(),
            Self::IfElseNode(i) => i.span(),
            Self::FunctionExpression(fun) => fun.span(),
            Self::VariableDeclaration(v) => v.span(),
            Self::FunctionCall(fun) => fun.span(),
            Self::DoExpression(d) => d.span(),
            Self::MatchExpression(m) => m.span(),
            Self::IndexExpression(l) => l.span(),
            Self::NewObjectExpression(n) => n.span(),
            Self::FieldAccessExpression(f) => f.span(),
            _ => unreachable!("Literal has no span"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::Identifier(i) => write!(f, "{}", i),
            Self::BinaryExpression(b) => write!(f, "{}", b),
            Self::UnaryExpression(u) => write!(f, "{}", u),
            Self::IfElseNode(i) => write!(f, "{}", i),
            Self::FunctionExpression(fun) => write!(f, "{}", fun),
            Self::VariableDeclaration(v) => write!(f, "{}", v),
            Self::FunctionCall(fun) => write!(f, "{}", fun),
            Self::DoExpression(d) => write!(f, "{}", d),
            Self::MatchExpression(m) => write!(f, "{}", m),
            Self::IndexExpression(l) => write!(f, "{}", l),
            Self::StructDeclaration(s) => write!(f, "{}", s),
            Self::NewObjectExpression(n) => write!(f, "{}", n),
            Self::FieldAccessExpression(fa) => write!(f, "{}", fa),
        }
    }
}
