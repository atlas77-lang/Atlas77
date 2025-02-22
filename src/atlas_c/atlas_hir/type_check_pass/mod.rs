//The first view versions of the type checking will be quite simple.
//As there will only be primitive types to check.
//A rework of the type checker will be done when structs, classes, enums and unions are added.

use super::{
    arena::HirArena,
    error::{
        FunctionTypeMismatchError, HirError, HirResult,
        TryingToNegateUnsignedError, TypeMismatchError, UnknownTypeError,
    },
    expr,
    expr::{HirBinaryOp, HirExpr},
    stmt::HirStatement,
    ty::{HirTy, HirTyId},
    HirFunction, HirModule, HirModuleSignature,
};
use crate::atlas_c::atlas_hir::error::{AccessingClassFieldOutsideClassError, AccessingPrivateFieldError, EmptyListLiteralError, FieldKind, UnsupportedExpr};
use crate::atlas_c::atlas_hir::expr::{HirFunctionCallExpr, HirIdentExpr};
use crate::atlas_c::atlas_hir::item::{HirClass, HirClassConstructor, HirClassMethod};
use crate::atlas_c::atlas_hir::signature::{HirClassMethodModifier, HirFunctionParameterSignature, HirFunctionSignature, HirVisibility};
use logos::Span;
use miette::{SourceOffset, SourceSpan};
use std::collections::HashMap;

pub struct TypeChecker<'hir> {
    arena: &'hir HirArena<'hir>,
    ///Keep track of the scopes and variables
    ///
    /// Should be rework in the future, variables should only be represented as (usize, usize)
    ///  (i.e. (scope, var_name) var_name being in the arena)
    context_functions: Vec<HashMap<String, ContextFunction<'hir>>>,
    signature: HirModuleSignature<'hir>,
    current_func_name: Option<&'hir str>,
    current_class_name: Option<&'hir str>,
    // Source code
    src: String,
    extern_monomorphized: HashMap<(&'hir str, Vec<&'hir HirTy<'hir>>), &'hir HirFunctionSignature<'hir>>,
}

pub struct ContextFunction<'hir> {
    pub scopes: Vec<ContextScope<'hir>>,
}

impl Default for ContextFunction<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'hir> ContextFunction<'hir> {
    pub fn new() -> Self {
        Self {
            scopes: vec![ContextScope::new(None)],
        }
    }
    pub fn new_scope(&mut self) -> usize {
        let parent = self.scopes.len() - 1;
        self.scopes.push(ContextScope::new(Some(parent)));
        parent
    }
    pub fn end_scope(&mut self) -> usize {
        self.scopes.pop();
        self.scopes.len() - 1
    }

    pub fn get(&self, name: &str) -> Option<&ContextVariable<'hir>> {
        let scope = self.scopes.last().unwrap();
        match scope.get(name) {
            Some(s) => Some(s),
            None => {
                let mut parent = scope.parent;
                while parent.is_some() {
                    let parent_scope = &self.scopes[parent.unwrap()];
                    match parent_scope.get(name) {
                        Some(s) => return Some(s),
                        None => parent = parent_scope.parent,
                    }
                }
                None
            }
        }
    }

    pub fn insert(&mut self, name: &'hir str, var: ContextVariable<'hir>) {
        self.scopes.last_mut().unwrap().insert(name, var);
    }
}

#[derive(Debug)]
pub struct ContextScope<'hir> {
    ///I should stop using HashMap everywhere. A ContextVariable should be `(depth, &'hir str)`
    /// depth as in the scope depth
    pub variables: HashMap<&'hir str, ContextVariable<'hir>>,
    pub parent: Option<usize>,
}

impl<'hir> ContextScope<'hir> {
    pub fn new(parent: Option<usize>) -> Self {
        Self {
            variables: HashMap::new(),
            parent,
        }
    }
    pub fn get(&self, name: &str) -> Option<&ContextVariable<'hir>> {
        self.variables.get(name)
    }
    pub fn insert(&mut self, name: &'hir str, var: ContextVariable<'hir>) {
        self.variables.insert(name, var);
    }
}

#[derive(Debug)]
pub struct ContextVariable<'hir> {
    pub _name: &'hir str,
    pub name_span: Span,
    pub ty: &'hir HirTy<'hir>,
    pub ty_span: Span,
    pub is_mut: bool,
}

impl<'hir> TypeChecker<'hir> {
    pub fn new(arena: &'hir HirArena<'hir>, src: String) -> Self {
        Self {
            arena,
            context_functions: vec![],
            src,
            signature: HirModuleSignature::default(),
            current_func_name: None,
            current_class_name: None,
            extern_monomorphized: HashMap::new(),
        }
    }

    pub fn check(&mut self, hir: &mut HirModule<'hir>) -> HirResult<()> {
        self.signature = hir.signature.clone();
        for func in &mut hir.body.functions {
            self.current_func_name = Some(func.0);
            self.check_func(func.1)?;
        }
        for class in &mut hir.body.classes {
            self.current_class_name = Some(class.0);
            self.check_class(class.1)?;
        }
        Ok(())
    }

    pub fn check_class(&mut self, class: &mut HirClass<'hir>) -> HirResult<()> {
        for method in &mut class.methods {
            self.current_class_name = Some(class.name);
            self.current_func_name = Some(method.name);
            self.context_functions.push(HashMap::new());
            self.check_method(method)?;
        }
        self.current_func_name = Some("constructor");
        self.check_constructor(&mut class.constructor)?;
        self.current_func_name = Some("destructor");
        self.check_destructor(&mut class.destructor)?;
        Ok(())
    }

    pub fn check_method(&mut self, method: &mut HirClassMethod<'hir>) -> HirResult<()> {
        self.context_functions.push(HashMap::new());
        self.context_functions.last_mut().unwrap().insert(
            self.current_func_name.unwrap().to_string(),
            ContextFunction::new(),
        );
        for param in &method.signature.params {
            self.context_functions
                .last_mut()
                .unwrap()
                .get_mut(self.current_func_name.unwrap())
                .unwrap()
                .insert(
                    param.name,
                    ContextVariable {
                        _name: param.name,
                        name_span: param.span.clone(),
                        ty: param.ty,
                        ty_span: param.ty_span.clone(),
                        is_mut: false,
                    },
                );
        }
        for stmt in &mut method.body.statements {
            self.check_stmt(stmt)?;
        }
        //Because it is a method we don't keep it in the `context_functions`
        self.context_functions.pop();
        Ok(())
    }

    pub fn check_func(&mut self, func: &mut HirFunction<'hir>) -> HirResult<()> {
        self.context_functions.push(HashMap::new());
        self.context_functions.last_mut().unwrap().insert(
            self.current_func_name.unwrap().to_string(),
            ContextFunction::new(),
        );
        for param in &func.signature.params {
            self.context_functions
                .last_mut()
                .unwrap()
                .get_mut(self.current_func_name.unwrap())
                .unwrap()
                .insert(
                    param.name,
                    ContextVariable {
                        _name: param.name,
                        name_span: param.span.clone(),
                        ty: param.ty,
                        ty_span: param.ty_span.clone(),
                        is_mut: false,
                    },
                );
        }
        for stmt in &mut func.body.statements {
            self.check_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn check_constructor(&mut self, constructor: &mut HirClassConstructor<'hir>) -> HirResult<()> {
        self.context_functions.push(HashMap::new());
        self.context_functions.last_mut().unwrap().insert(
            String::from("constructor"),
            ContextFunction::new(),
        );
        for param in &constructor.params {
            self.context_functions
                .last_mut()
                .unwrap()
                .get_mut("constructor")
                .unwrap()
                .insert(
                    param.name,
                    ContextVariable {
                        _name: param.name,
                        name_span: param.span.clone(),
                        ty: param.ty,
                        ty_span: param.ty_span.clone(),
                        is_mut: false,
                    },
                );
        }
        for stmt in &mut constructor.body.statements {
            self.check_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn check_destructor(&mut self, destructor: &mut HirClassConstructor<'hir>) -> HirResult<()> {
        self.context_functions.push(HashMap::new());
        self.context_functions.last_mut().unwrap().insert(
            String::from("destructor"),
            ContextFunction::new(),
        );
        for param in &destructor.params {
            self.context_functions
                .last_mut()
                .unwrap()
                .get_mut("destructor")
                .unwrap()
                .insert(
                    param.name,
                    ContextVariable {
                        _name: param.name,
                        name_span: param.span.clone(),
                        ty: param.ty,
                        ty_span: param.ty_span.clone(),
                        is_mut: false,
                    },
                );
        }
        for stmt in &mut destructor.body.statements {
            self.check_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn check_stmt(&mut self, stmt: &mut HirStatement<'hir>) -> HirResult<()> {
        match stmt {
            HirStatement::Expr(e) => {
                self.check_expr(&mut e.expr)?;
                Ok(())
            }
            HirStatement::Return(r) => {
                let actual_ret_ty = self.check_expr(&mut r.value)?;
                let mut expected_ret_ty = self.arena.types().get_uninitialized_ty();
                let mut span = SourceSpan::new(SourceOffset::from(r.span.start), r.span.end - r.span.start);
                if self.current_class_name.is_some() {
                    //This means we're in a class method
                    let class = self.signature.classes.get(self.current_class_name.unwrap()).unwrap();
                    let method = class.methods.get(self.current_func_name.unwrap()).unwrap();
                    expected_ret_ty = method.return_ty;
                    span = SourceSpan::new(
                        SourceOffset::from(
                            method.return_ty_span.clone().unwrap_or(r.span.clone()).start,
                        ),
                        method.return_ty_span.clone().unwrap_or(r.span.clone()).end
                            - method.return_ty_span.clone().unwrap_or(r.span.clone()).start,
                    )
                } else if self.current_func_name.is_some() {
                    //This means we're in a standalone function
                    let func_ret_from = self
                        .signature
                        .functions
                        .get(self.current_func_name.unwrap())
                        .unwrap();
                    expected_ret_ty = func_ret_from.return_ty;
                    span = SourceSpan::new(
                        SourceOffset::from(
                            func_ret_from.return_ty_span.clone().unwrap_or(r.span.clone()).start,
                        ),
                        func_ret_from.return_ty_span.clone().unwrap_or(r.span.clone()).end
                            - func_ret_from.return_ty_span.clone().unwrap_or(r.span.clone()).start,
                    )
                }
                if HirTyId::from(actual_ret_ty) != HirTyId::from(expected_ret_ty) {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", actual_ret_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(r.value.span().start),
                            r.value.span().end - r.value.span().start,
                        ),
                        expected_type: format!("{}", expected_ret_ty),
                        expected_loc: span,
                        src: self.src.clone(),
                    }));
                }
                Ok(())
            }
            HirStatement::While(w) => {
                let cond_ty = self.check_expr(&mut w.condition)?;
                if HirTyId::from(cond_ty) != HirTyId::compute_boolean_ty_id() {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", cond_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(w.condition.span().start),
                            w.condition.span().end - w.condition.span().start,
                        ),
                        expected_type: format!("{}", self.arena.types().get_boolean_ty()),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(w.condition.span().start),
                            w.condition.span().end - w.condition.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }
                //there should be just "self.context.new_scope()" and "self.context.end_scope()"
                self.context_functions
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .new_scope();
                for stmt in &mut w.body.statements {
                    self.check_stmt(stmt)?;
                }
                self.context_functions
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .end_scope();

                Ok(())
            }
            HirStatement::IfElse(i) => {
                let cond_ty = self.check_expr(&mut i.condition)?;
                if HirTyId::from(cond_ty) != HirTyId::compute_boolean_ty_id() {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", cond_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(i.condition.span().start),
                            i.condition.span().end - i.condition.span().start,
                        ),
                        expected_type: format!("{}", self.arena.types().get_boolean_ty()),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(i.condition.span().start),
                            i.condition.span().end - i.condition.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }

                self.context_functions
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .new_scope();
                for stmt in &mut i.then_branch.statements {
                    self.check_stmt(stmt)?;
                }
                self.context_functions
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .end_scope();
                if let Some(ref mut else_branch) = &mut i.else_branch {
                    self.context_functions
                        .last_mut()
                        .unwrap()
                        .get_mut(self.current_func_name.unwrap())
                        .unwrap()
                        .new_scope();
                    for stmt in &mut else_branch.statements {
                        self.check_stmt(stmt)?;
                    }
                    self.context_functions
                        .last_mut()
                        .unwrap()
                        .get_mut(self.current_func_name.unwrap())
                        .unwrap()
                        .end_scope();
                }
                Ok(())
            }
            HirStatement::Const(c) => {
                let expr_ty = self.check_expr(&mut c.value)?;
                let const_ty = c.ty.unwrap_or(expr_ty);
                c.ty = Some(const_ty);
                let ty = HirTyId::from(const_ty);
                self.context_functions
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .insert(
                        c.name,
                        ContextVariable {
                            _name: c.name,
                            name_span: c.name_span.clone(),
                            ty: const_ty,
                            ty_span: c.ty_span.clone().unwrap_or(c.name_span.clone()),
                            is_mut: false,
                        },
                    );

                if HirTyId::from(expr_ty) != ty {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", expr_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(c.value.span().start),
                            c.value.span().end - c.value.span().start,
                        ),
                        expected_type: format!("{}", const_ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(c.span.start),
                            c.name_span.end - c.span.start,
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(())
            }
            HirStatement::Let(l) => {
                let expr_ty = self.check_expr(&mut l.value)?;
                let var_ty = l.ty.unwrap_or(expr_ty);
                l.ty = Some(var_ty);
                let ty = HirTyId::from(var_ty);
                self.context_functions
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .insert(
                        l.name,
                        ContextVariable {
                            _name: l.name,
                            name_span: l.name_span.clone(),
                            ty: var_ty,
                            ty_span: l.ty_span.clone().unwrap_or(l.name_span.clone()),
                            is_mut: true,
                        },
                    );
                if HirTyId::from(expr_ty) != ty {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", expr_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(l.value.span().start),
                            l.value.span().end - l.value.span().start,
                        ),
                        expected_type: format!("{}", var_ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(l.name_span.start),
                            l.name_span.end - l.name_span.start,
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(())
            }
            _ => {
                todo!("TypeChecker::check_stmt: {:?}", stmt)
            }
        }
    }
    pub fn check_expr(&mut self, expr: &mut HirExpr<'hir>) -> HirResult<&'hir HirTy<'hir>> {
        match expr {
            HirExpr::IntegerLiteral(_) => Ok(self.arena.types().get_integer64_ty()),
            HirExpr::FloatLiteral(_) => Ok(self.arena.types().get_float64_ty()),
            HirExpr::UnsignedIntegerLiteral(_) => Ok(self.arena.types().get_uint64_ty()),
            HirExpr::BooleanLiteral(_) => Ok(self.arena.types().get_boolean_ty()),
            HirExpr::UnitLiteral(_) => Ok(self.arena.types().get_unit_ty()),
            HirExpr::CharLiteral(_) => Ok(self.arena.types().get_char_ty()),
            HirExpr::StringLiteral(_) => Ok(self.arena.types().get_str_ty()),
            HirExpr::SelfLiteral(s) => {
                let class_name = match self.current_class_name {
                    Some(class_name) => class_name,
                    None => return Err(HirError::AccessingClassFieldOutsideClass(AccessingClassFieldOutsideClassError {
                        span: SourceSpan::new(
                            SourceOffset::from(expr.span().start),
                            expr.span().end - expr.span().start,
                        ),
                        src: self.src.clone(),
                    }))
                };
                let class = self.signature.classes.get(class_name).unwrap();
                let self_ty = self.arena.types().get_named_ty(class.name, class.span.clone());
                s.ty = self_ty;
                Ok(self_ty)
            }
            HirExpr::NewArray(a) => {
                let size_ty = self.check_expr(a.size.as_mut())?;
                let size_ty_id = HirTyId::from(size_ty);
                if size_ty_id != HirTyId::compute_uint64_ty_id() && size_ty_id != HirTyId::compute_integer64_ty_id() {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", size_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(a.size.span().start),
                            a.size.span().end - a.size.span().start,
                        ),
                        expected_type: format!("{} or {}", self.arena.types().get_uint64_ty(), self.arena.types().get_integer64_ty()),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(a.size.span().start),
                            a.size.span().end - a.size.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(a.ty)
            }
            HirExpr::ListLiteral(l) => {
                if l.items.is_empty() {
                    return Err(HirError::EmptyListLiteral(EmptyListLiteralError {
                        span: SourceSpan::new(
                            SourceOffset::from(l.span.start),
                            l.span.end - l.span.start,
                        ),
                        src: self.src.clone(),
                    }));
                }
                let ty = self.check_expr(&mut l.items[0])?;
                for e in &mut l.items {
                    let e_ty = self.check_expr(e)?;
                    if HirTyId::from(e_ty) != HirTyId::from(ty) {
                        return Err(HirError::TypeMismatch(TypeMismatchError {
                            actual_type: format!("{}", e_ty),
                            actual_loc: SourceSpan::new(
                                SourceOffset::from(e.span().start),
                                e.span().end - e.span().start,
                            ),
                            expected_type: format!("{}", ty),
                            expected_loc: SourceSpan::new(
                                SourceOffset::from(l.span.start),
                                l.span.end - l.span.start,
                            ),
                            src: self.src.clone(),
                        }));
                    }
                }
                l.ty = self.arena.types().get_list_ty(ty);
                Ok(self.arena.types().get_list_ty(ty))
            }
            HirExpr::Unary(u) => {
                let ty = self.check_expr(&mut u.expr)?;
                match u.op {
                    Some(expr::HirUnaryOp::Neg) => {
                        if HirTyId::from(ty) != HirTyId::compute_integer64_ty_id() {
                            return Err(HirError::TryingToNegateUnsigned(
                                TryingToNegateUnsignedError {
                                    span: SourceSpan::new(
                                        SourceOffset::from(u.expr.span().start),
                                        u.expr.span().end - u.expr.span().start,
                                    ),
                                    src: self.src.clone(),
                                },
                            ));
                        }
                        u.ty = ty;
                        Ok(ty)
                    }
                    _ => {
                        u.ty = ty;
                        Ok(ty)
                    }
                }
            }
            HirExpr::Casting(c) => {
                let expr_ty = self.check_expr(&mut c.expr)?;
                let can_cast = matches!(
                    expr_ty,
                    HirTy::Int64(_) | HirTy::Float64(_) |
                    HirTy::UInt64(_) | HirTy::Boolean(_) |
                    HirTy::Char(_) | HirTy::String(_)
                );
                if !can_cast {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", expr_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(c.expr.span().start),
                            c.expr.span().end - c.expr.span().start,
                        ),
                        expected_type: "int64, float64, uint64, bool, char or str".to_string(),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(c.expr.span().start),
                            c.expr.span().end - c.expr.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }

                Ok(c.ty)
            }
            HirExpr::Indexing(indexing_expr) => {
                let target = self.check_expr(&mut indexing_expr.target)?;
                let index = self.check_expr(&mut indexing_expr.index)?;
                if
                HirTyId::from(index) != HirTyId::compute_uint64_ty_id() &&
                    HirTyId::from(index) != HirTyId::compute_integer64_ty_id() {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", index),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(indexing_expr.index.span().start),
                            indexing_expr.index.span().end - indexing_expr.index.span().start,
                        ),
                        expected_type: format!("{}", self.arena.types().get_uint64_ty()),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(indexing_expr.index.span().start),
                            indexing_expr.index.span().end - indexing_expr.index.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }

                match target {
                    HirTy::List(l) => {
                        indexing_expr.ty = l.inner;
                        Ok(l.inner)
                    }
                    HirTy::String(_) => {
                        indexing_expr.ty = self.arena.types().get_char_ty();
                        Ok(self.arena.types().get_char_ty())
                    }
                    _ => {
                        todo!("TypeChecker::check_expr: {:?}", target)
                    }
                }
            }
            HirExpr::HirBinaryOp(b) => {
                let lhs = self.check_expr(&mut b.lhs)?;
                b.ty = lhs;
                let rhs = self.check_expr(&mut b.rhs)?;
                if HirTyId::from(lhs) != HirTyId::from(rhs) {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", lhs),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(b.lhs.span().start),
                            b.lhs.span().end - b.lhs.span().start,
                        ),
                        expected_type: format!("{}", rhs),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(b.rhs.span().start),
                            b.rhs.span().end - b.rhs.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }

                match b.op {
                    HirBinaryOp::And
                    | HirBinaryOp::Eq
                    | HirBinaryOp::Neq
                    | HirBinaryOp::Or
                    | HirBinaryOp::Gt
                    | HirBinaryOp::Gte
                    | HirBinaryOp::Lt
                    | HirBinaryOp::Lte => Ok(self.arena.types().get_boolean_ty()),
                    _ => Ok(lhs),
                }
            }
            HirExpr::Call(func_expr) => {
                let callee = func_expr.callee.as_mut();
                match callee {
                    HirExpr::Ident(i) => {
                        let name = i.name;
                        let func = match self.signature.functions.get(name) {
                            Some(f) => *f,
                            None => {
                                return Err(HirError::UnknownType(UnknownTypeError {
                                    name: name.to_string(),
                                    span: SourceSpan::new(
                                        SourceOffset::from(func_expr.span.start),
                                        func_expr.span.end - func_expr.span.start,
                                    ),
                                    src: self.src.clone(),
                                }))
                            }
                        };

                        if func.params.len() != func_expr.args.len() {
                            return Err(HirError::FunctionTypeMismatch(FunctionTypeMismatchError {
                                expected_ty: format!("{:?}", func),
                                span: SourceSpan::new(
                                    SourceOffset::from(func.span.start),
                                    func.span.end - func.span.start,
                                ),
                                src: self.src.clone(),
                            }));
                        }

                        //Only check if it's an external function with generics (e.g. `extern foo<T>(a: T) -> T`)
                        if func.is_external && func.generics.is_some() {
                            return self.check_extern_fn(name, func_expr, func);
                        }

                        for (param, arg) in func.params.iter().zip(func_expr.args.iter_mut()) {
                            let arg_ty = self.check_expr(arg)?;

                            if HirTyId::from(arg_ty) != HirTyId::from(param.ty) {
                                return Err(HirError::TypeMismatch(TypeMismatchError {
                                    actual_type: format!("{}", arg_ty),
                                    actual_loc: SourceSpan::new(
                                        SourceOffset::from(arg.span().start),
                                        arg.span().end - arg.span().start,
                                    ),
                                    expected_type: format!("{}", param.ty),
                                    expected_loc: SourceSpan::new(
                                        SourceOffset::from(param.span.start),
                                        param.span.end - param.span.start,
                                    ),
                                    src: self.src.clone(),
                                }));
                            }
                        }

                        Ok(func.return_ty)
                    }
                    //todo: Check if the field access try to access public/private functions
                    HirExpr::FieldAccess(field_access) => {
                        let target_ty = self.check_expr(&mut field_access.target)?;
                        if let HirTy::Named(n) = target_ty {
                            let class = match self.signature.classes.get(n.name) {
                                Some(c) => *c,
                                None => {
                                    return Err(HirError::UnknownType(UnknownTypeError {
                                        name: n.name.to_string(),
                                        span: SourceSpan::new(
                                            SourceOffset::from(field_access.span.start),
                                            field_access.span.end - field_access.span.start,
                                        ),
                                        src: self.src.clone(),
                                    }));
                                }
                            };
                            let method = class.methods.iter().find(|m| *m.0 == field_access.field.name);

                            if let Some((_, method_signature)) = method {
                                //Check if you're currently in the class, if not check is the method public
                                if self.current_class_name != Some(n.name) && method_signature.vis != HirVisibility::Public {
                                    return Err(HirError::AccessingPrivateField(AccessingPrivateFieldError {
                                        span: SourceSpan::new(
                                            SourceOffset::from(expr.span().start),
                                            expr.span().end - expr.span().start,
                                        ),
                                        kind: FieldKind::Function,
                                        src: self.src.clone(),
                                    }));
                                }
                                if method_signature.modifier == HirClassMethodModifier::Static {
                                    return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                        span: SourceSpan::new(
                                            SourceOffset::from(field_access.span.start),
                                            field_access.span.end - field_access.span.start,
                                        ),
                                        expr: "Static method call".to_string(),
                                        src: self.src.clone(),
                                    }));
                                }
                                if method_signature.params.len() != func_expr.args.len() {
                                    return Err(HirError::FunctionTypeMismatch(FunctionTypeMismatchError {
                                        expected_ty: format!("{:?}", method_signature),
                                        span: SourceSpan::new(
                                            SourceOffset::from(field_access.span.start),
                                            field_access.span.end - field_access.span.start,
                                        ),
                                        src: self.src.clone(),
                                    }));
                                }
                                for (param, arg) in method_signature.params.iter().zip(func_expr.args.iter_mut()) {
                                    let arg_ty = self.check_expr(arg)?;
                                    if HirTyId::from(arg_ty) != HirTyId::from(param.ty) {
                                        return Err(HirError::TypeMismatch(TypeMismatchError {
                                            actual_type: format!("{}", arg_ty),
                                            actual_loc: SourceSpan::new(
                                                SourceOffset::from(arg.span().start),
                                                arg.span().end - arg.span().start,
                                            ),
                                            expected_type: format!("{}", param.ty),
                                            expected_loc: SourceSpan::new(
                                                SourceOffset::from(param.span.start),
                                                param.span.end - param.span.start,
                                            ),
                                            src: self.src.clone(),
                                        }));
                                    }
                                }
                                //At one point, field_access.field.ty should be equal to a function returning `return_ty`
                                field_access.ty = method_signature.return_ty;
                                func_expr.ty = method_signature.return_ty;
                                field_access.field.ty = method_signature.return_ty;

                                Ok(method_signature.return_ty)
                            } else {
                                Err(HirError::UnknownType(UnknownTypeError {
                                    name: field_access.field.name.to_string(),
                                    span: SourceSpan::new(
                                        SourceOffset::from(field_access.span.start),
                                        field_access.span.end - field_access.span.start,
                                    ),
                                    src: self.src.clone(),
                                }))
                            }
                        } else {
                            Err(HirError::TypeMismatch(TypeMismatchError {
                                actual_type: format!("{}", target_ty),
                                actual_loc: SourceSpan::new(
                                    SourceOffset::from(field_access.span.start),
                                    field_access.span.end - field_access.span.start,
                                ),
                                expected_type: String::from("Named"),
                                expected_loc: SourceSpan::new(
                                    SourceOffset::from(field_access.span.start),
                                    field_access.span.end - field_access.span.start,
                                ),
                                src: self.src.clone(),
                            }))
                        }
                    }
                    HirExpr::StaticAccess(static_access) => {
                        let class = match self.signature.classes.get(static_access.target.name) {
                            Some(c) => *c,
                            None => {
                                return Err(HirError::UnknownType(UnknownTypeError {
                                    name: static_access.target.name.to_string(),
                                    span: SourceSpan::new(
                                        SourceOffset::from(static_access.span.start),
                                        static_access.span.end - static_access.span.start,
                                    ),
                                    src: self.src.clone(),
                                }));
                            }
                        };
                        let func = class.methods.iter().find(|m| *m.0 == static_access.field.name);
                        if let Some((_, method_signature)) = func {
                            if method_signature.modifier == HirClassMethodModifier::None || method_signature.modifier == HirClassMethodModifier::Const {
                                return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                    span: SourceSpan::new(
                                        SourceOffset::from(static_access.span.start),
                                        static_access.span.end - static_access.span.start,
                                    ),
                                    expr: "Instance method call".to_string(),
                                    src: self.src.clone(),
                                }));
                            }
                            if method_signature.params.len() != func_expr.args.len() {
                                return Err(HirError::FunctionTypeMismatch(FunctionTypeMismatchError {
                                    expected_ty: format!("{:?}", method_signature),
                                    span: SourceSpan::new(
                                        SourceOffset::from(static_access.span.start),
                                        static_access.span.end - static_access.span.start,
                                    ),
                                    src: self.src.clone(),
                                }));
                            }
                            for (param, arg) in method_signature.params.iter().zip(func_expr.args.iter_mut()) {
                                let arg_ty = self.check_expr(arg)?;
                                if HirTyId::from(arg_ty) != HirTyId::from(param.ty) {
                                    return Err(HirError::TypeMismatch(TypeMismatchError {
                                        actual_type: format!("{}", arg_ty),
                                        actual_loc: SourceSpan::new(
                                            SourceOffset::from(arg.span().start),
                                            arg.span().end - arg.span().start,
                                        ),
                                        expected_type: format!("{}", param.ty),
                                        expected_loc: SourceSpan::new(
                                            SourceOffset::from(param.span.start),
                                            param.span.end - param.span.start,
                                        ),
                                        src: self.src.clone(),
                                    }));
                                }
                            }

                            static_access.ty = method_signature.return_ty;
                            func_expr.ty = method_signature.return_ty;
                            static_access.target.ty = self.arena.types().get_named_ty(class.name, class.span.clone());
                            static_access.field.ty = method_signature.return_ty;

                            Ok(method_signature.return_ty)
                        } else {
                            Err(HirError::UnknownType(UnknownTypeError {
                                name: static_access.field.name.to_string(),
                                span: SourceSpan::new(
                                    SourceOffset::from(static_access.span.start),
                                    static_access.span.end - static_access.span.start,
                                ),
                                src: self.src
                                    .clone(),
                            }))
                        }
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            HirExpr::Assign(a) => {
                let rhs = self.check_expr(&mut a.rhs)?;
                let lhs = self.check_expr(&mut a.lhs)?;
                if HirTyId::from(lhs) != HirTyId::from(rhs) {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", rhs),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(a.rhs.span().start),
                            a.rhs.span().end - a.rhs.span().start,
                        ),
                        expected_type: format!("{}", lhs),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(a.lhs.span().start),
                            a.lhs.span().end - a.lhs.span().start,
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(lhs)
            }
            HirExpr::Ident(i) => {
                let ctx_var = self.get_ident_ty(i)?;
                Ok(ctx_var.ty)
            }
            HirExpr::Delete(d) => {
                let to_delete = self.check_expr(&mut d.expr)?;
                match to_delete {
                    HirTy::Named(_) | HirTy::List(_) | HirTy::String(_) => {
                        Ok(self.arena.types().get_unit_ty())
                    }
                    _ => {
                        Err(HirError::TypeMismatch(TypeMismatchError {
                            actual_type: format!("{}", to_delete),
                            actual_loc: SourceSpan::new(
                                SourceOffset::from(d.expr.span().start),
                                d.expr.span().end - d.expr.span().start,
                            ),
                            expected_type: String::from("Named, [T] or str"),
                            expected_loc: SourceSpan::new(
                                SourceOffset::from(d.expr.span().start),
                                d.expr.span().end - d.expr.span().start,
                            ),
                            src: self.src.clone(),
                        }))
                    }
                }
            }
            HirExpr::NewObj(obj) => {
                let class_ty;
                let class_signature = if let HirTy::Named(n) = obj.ty {
                    class_ty = n;
                    let tmp = match self.signature.classes.get(n.name) {
                        Some(c) => c,
                        None => {
                            return Err(HirError::UnknownType(UnknownTypeError {
                                name: n.name.to_string(),
                                span: SourceSpan::new(
                                    SourceOffset::from(obj.span.start),
                                    obj.span.end - obj.span.start,
                                ),
                                src: self.src.clone(),
                            }));
                        }
                    };
                    *tmp
                } else {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", obj.ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(obj.span.start),
                            obj.span.end - obj.span.start,
                        ),
                        expected_type: String::from("Named"),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(obj.span.start),
                            obj.span.end - obj.span.start,
                        ),
                        src: self.src.clone(),
                    }));
                };
                for (param, arg) in class_signature.constructor.params.iter().zip(obj.args.iter_mut()) {
                    let arg_ty = self.check_expr(arg)?;
                    if HirTyId::from(arg_ty) != HirTyId::from(param.ty) {
                        return Err(HirError::TypeMismatch(TypeMismatchError {
                            actual_type: format!("{}", arg_ty),
                            actual_loc: SourceSpan::new(
                                SourceOffset::from(arg.span().start),
                                arg.span().end - arg.span().start,
                            ),
                            expected_type: format!("{}", param.ty),
                            expected_loc: SourceSpan::new(
                                SourceOffset::from(param.span.start),
                                param.span.end - param.span.start,
                            ),
                            src: self.src.clone(),
                        }));
                    }
                }
                obj.ty = self.arena.types().get_named_ty(class_ty.name, class_ty.span.clone());
                Ok(obj.ty)
            }
            HirExpr::FieldAccess(field_access) => {
                let target_ty = self.check_expr(&mut field_access.target)?;
                if let HirTy::Named(n) = target_ty {
                    let class = match self.signature.classes.get(n.name) {
                        Some(c) => *c,
                        None => {
                            return Err(HirError::UnknownType(UnknownTypeError {
                                name: n.name.to_string(),
                                span: SourceSpan::new(
                                    SourceOffset::from(field_access.span.start),
                                    field_access.span.end - field_access.span.start,
                                ),
                                src: self.src.clone(),
                            }));
                        }
                    };
                    let field = class.fields.iter().find(|f| *f.0 == field_access.field.name);
                    if let Some((_, field_signature)) = field {
                        if self.current_class_name != Some(n.name) && field_signature.vis != HirVisibility::Public {
                            return Err(HirError::AccessingPrivateField(AccessingPrivateFieldError {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start),
                                    expr.span().end - expr.span().start,
                                ),
                                kind: FieldKind::Field,
                                src: self.src.clone(),
                            }));
                        }
                        field_access.ty = field_signature.ty;
                        field_access.field.ty = field_signature.ty;
                        Ok(field_signature.ty)
                    } else {
                        Err(HirError::UnknownType(UnknownTypeError {
                            name: field_access.field.name.to_string(),
                            span: SourceSpan::new(
                                SourceOffset::from(field_access.span.start),
                                field_access.span.end - field_access.span.start,
                            ),
                            src: self.src.clone(),
                        }))
                    }
                } else {
                    Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", target_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(field_access.span.start),
                            field_access.span.end - field_access.span.start,
                        ),
                        expected_type: String::from("Named"),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(field_access.span.start),
                            field_access.span.end - field_access.span.start,
                        ),
                        src: self.src.clone(),
                    }))
                }
            }
            HirExpr::StaticAccess(static_access) => {
                let class = match self.signature.classes.get(static_access.target.name) {
                    Some(c) => *c,
                    None => {
                        return Err(HirError::UnknownType(UnknownTypeError {
                            name: static_access.target.name.to_string(),
                            span: SourceSpan::new(
                                SourceOffset::from(static_access.span.start),
                                static_access.span.end - static_access.span.start,
                            ),
                            src: self.src.clone(),
                        }));
                    }
                };
                let constant = class.constants.iter().find(|f| *f.0 == static_access.field.name);
                if let Some((_, const_signature)) = constant {
                    static_access.target.ty = self.arena.types().get_named_ty(class.name, class.span.clone());
                    static_access.field.ty = const_signature.ty;
                    static_access.ty = const_signature.ty;
                    Ok(const_signature.ty)
                } else {
                    Err(HirError::UnknownType(UnknownTypeError {
                        name: static_access.field.name.to_string(),
                        span: SourceSpan::new(
                            SourceOffset::from(static_access.span.start),
                            static_access.span.end - static_access.span.start,
                        ),
                        src: self.src.clone(),
                    }))
                }
            }
        }
    }

    fn check_extern_fn(&mut self, name: &'hir str, expr: &mut HirFunctionCallExpr<'hir>, signature: &'hir HirFunctionSignature<'hir>) -> HirResult<&'hir HirTy<'hir>> {
        let args_ty = expr
            .args
            .iter_mut()
            .map(|a| self.check_expr(a))
            .collect::<HirResult<Vec<_>>>()?;
        let monomorphized = self.extern_monomorphized.get(&(name, args_ty.clone()));
        if let Some(m) = monomorphized {
            return Ok(m.return_ty);
        }
        //Contains the name + the actual type of that generic
        let mut generics: Vec<(&'hir str, &'hir HirTy<'hir>)> = Vec::new();
        let mut params = vec![];
        for (i, (param, arg)) in signature.params.iter().zip(args_ty.iter()).enumerate() {
            //This only take the name of the generic type (e.g. `T` in `extern foo<T>(a: T) -> T`)
            //So `extern foo<T>(a: [T]) -> T` won't be correctly type checked

            if let Some(name) = Self::get_generic_name(param.ty) {
                let ty = if let Some(ty) = Self::get_generic_ty(param.ty, arg) {
                    ty
                } else {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{}", arg),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(expr.args[i].span().start),
                            expr.args[i].span().end - expr.args[i].span().start,
                        ),
                        expected_type: format!("{}", param.ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(param.span.start),
                            param.span.end - param.span.start,
                        ),
                        src: self.src.clone(),
                    }));
                };
                generics.push((name, ty));
            }
            let param_sign: &'hir HirFunctionParameterSignature = self.arena.intern(HirFunctionParameterSignature {
                name: param.name,
                name_span: param.name_span.clone(),
                span: param.span.clone(),
                ty: arg,
                ty_span: param.ty_span.clone(),
            });
            params.push(param_sign);
        }

        let mut monomorphized = signature.clone();
        monomorphized.params = params;
        if let Some(name) = Self::get_generic_name(monomorphized.return_ty) {
            let actual_generic_ty = generics.iter().find(|(n, _)| *n == name).unwrap().1;
            let return_ty = self.get_generic_ret_ty(monomorphized.return_ty, actual_generic_ty);

            monomorphized.return_ty = return_ty;
        };

        monomorphized.generics = None;
        let signature = self.arena.intern(monomorphized);
        self.extern_monomorphized.insert((name, args_ty), signature);
        Ok(signature.return_ty)
    }

    fn get_generic_name(ty: &'hir HirTy<'hir>) -> Option<&'hir str> {
        match ty {
            HirTy::List(l) => Self::get_generic_name(l.inner),
            HirTy::Named(n) => Some(n.name),
            _ => None,
        }
    }

    fn get_generic_ret_ty(&self, ty: &'hir HirTy<'hir>, actual_generic_ty: &'hir HirTy<'hir>) -> &'hir HirTy<'hir> {
        match ty {
            HirTy::List(l) => self.arena.types().get_list_ty(self.get_generic_ret_ty(l.inner, actual_generic_ty)),
            HirTy::Named(_) => {
                actual_generic_ty
            }
            _ => actual_generic_ty,
        }
    }

    /// Return the type of the generic after monormophization
    /// e.g. `foo<T>(a: T) -> T` with `foo(42)` will return `int64`
    fn get_generic_ty(ty: &'hir HirTy<'hir>, given_ty: &'hir HirTy<'hir>) -> Option<&'hir HirTy<'hir>> {
        match (ty, given_ty) {
            (HirTy::List(l1), HirTy::List(l2)) => Self::get_generic_ty(l1.inner, l2.inner),
            (HirTy::Named(_), _) => {
                Some(given_ty)
            }
            _ => None,
        }
    }

    fn get_ident_ty(&mut self, i: &mut HirIdentExpr<'hir>) -> HirResult<&ContextVariable<'hir>> {
        if let Some(ctx_var) = self
            .context_functions
            .last()
            .unwrap()
            .get(self.current_func_name.unwrap())
            .unwrap()
            .get(i.name)
        {
            i.ty = ctx_var.ty;
            Ok(ctx_var)
        } else {
            Err(HirError::UnknownType(UnknownTypeError {
                name: i.name.to_string(),
                span: SourceSpan::new(
                    SourceOffset::from(i.span.start),
                    i.span.end - i.span.start,
                ),
                src: self.src.clone(),
            }))
        }
    }
}
