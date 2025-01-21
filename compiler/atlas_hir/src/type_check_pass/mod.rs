//The first view versions of the type checking will be quite simple.
//As there will only be primitive types to check.
//A rework of the type checker will be done when structs, classes, enums and unions are added.

use atlas_core::prelude::{Span, Spanned};
use miette::{SourceOffset, SourceSpan};
use std::collections::HashMap;

use super::{
    arena::HirArena,
    error::{
        FunctionTypeMismatchError, HirError, HirResult, TryingToMutateImmutableVariableError,
        TryingToNegateUnsignedError, TypeMismatchError, UnknownTypeError,
    },
    expr,
    expr::{HirBinaryOp, HirExpr},
    stmt::HirStatement,
    ty::{HirTy, HirTyId},
    HirFunction, HirModule, HirModuleSignature,
};

pub struct TypeChecker<'hir> {
    arena: &'hir HirArena<'hir>,
    ///Keep track of the scopes and variables
    ///
    /// Should be rework in the future, variables should only be represented as (usize, usize)
    ///  (i.e. (scope, var_name) var_name being in the arena)
    context: Vec<HashMap<String, ContextFunction<'hir>>>,
    signature: HirModuleSignature<'hir>,
    current_func_name: Option<&'hir str>,
    // Source code
    src: String,
}

pub struct ContextFunction<'hir> {
    pub scopes: Vec<ContextScope<'hir>>,
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
    span: Span,
}

impl<'hir> TypeChecker<'hir> {
    pub fn new(arena: &'hir HirArena<'hir>, src: String) -> Self {
        Self {
            arena,
            context: vec![],
            src,
            signature: HirModuleSignature::default(),
            current_func_name: None,
        }
    }

    pub fn check(&mut self, hir: &mut HirModule<'hir>) -> HirResult<()> {
        self.signature = hir.signature.clone();
        for func in &mut hir.body.functions {
            self.current_func_name = Some(func.0);
            self.check_func(func.1)?;
        }
        Ok(())
    }

    pub fn check_func(&mut self, func: &mut HirFunction<'hir>) -> HirResult<()> {
        self.context.push(HashMap::new());
        self.context.last_mut().unwrap().insert(
            self.current_func_name.unwrap().to_string(),
            ContextFunction::new(),
        );
        for param in &func.signature.params {
            self.context
                .last_mut()
                .unwrap()
                .get_mut(self.current_func_name.unwrap())
                .unwrap()
                .insert(
                    param.name,
                    ContextVariable {
                        _name: param.name,
                        name_span: param.span,
                        ty: param.ty,
                        ty_span: param.ty_span,
                        is_mut: false,
                        span: param.span,
                    },
                );
        }
        for stmt in &mut func.body.statements {
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
                let func_ret_from = self
                    .signature
                    .functions
                    .get(self.current_func_name.unwrap())
                    .unwrap();
                let expected_ret_ty = func_ret_from.return_ty;
                if HirTyId::from(actual_ret_ty) != HirTyId::from(expected_ret_ty) {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", actual_ret_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(r.value.start()),
                            r.value.end() - r.value.start(),
                        ),
                        expected_type: format!("{:?}", expected_ret_ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(
                                func_ret_from.return_ty_span.unwrap_or(r.span).start(),
                            ),
                            func_ret_from.return_ty_span.unwrap_or(r.span).end()
                                - func_ret_from.return_ty_span.unwrap_or(r.span).start(),
                        ),
                        src: self.src.clone(),
                    }));
                }

                Ok(())
            }
            HirStatement::While(w) => {
                let cond_ty = self.check_expr(&mut w.condition)?;
                if HirTyId::from(cond_ty) != HirTyId::compute_boolean_ty_id() {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", cond_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(w.condition.start()),
                            w.condition.end() - w.condition.start(),
                        ),
                        expected_type: format!("{:?}", self.arena.types().get_boolean_ty()),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(w.condition.start()),
                            w.condition.end() - w.condition.start(),
                        ),
                        src: self.src.clone(),
                    }));
                }
                //there should be just "self.context.new_scope()" and "self.context.end_scope()"
                self.context
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .new_scope();
                for stmt in &mut w.body.statements {
                    self.check_stmt(stmt)?;
                }
                self.context
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
                        actual_type: format!("{:?}", cond_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(i.condition.start()),
                            i.condition.end() - i.condition.start(),
                        ),
                        expected_type: format!("{:?}", self.arena.types().get_boolean_ty()),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(i.condition.start()),
                            i.condition.end() - i.condition.start(),
                        ),
                        src: self.src.clone(),
                    }));
                }

                self.context
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .new_scope();
                for stmt in &mut i.then_branch.statements {
                    self.check_stmt(stmt)?;
                }
                self.context
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .end_scope();
                if let Some(ref mut else_branch) = &mut i.else_branch {
                    self.context
                        .last_mut()
                        .unwrap()
                        .get_mut(self.current_func_name.unwrap())
                        .unwrap()
                        .new_scope();
                    for stmt in &mut else_branch.statements {
                        self.check_stmt(stmt)?;
                    }
                    self.context
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
                let const_ty = c.ty.unwrap_or_else(|| expr_ty);
                c.ty = Some(const_ty);
                let ty = HirTyId::from(const_ty);
                self.context
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .insert(
                        c.name,
                        ContextVariable {
                            _name: c.name,
                            name_span: c.name_span,
                            ty: const_ty,
                            ty_span: c.ty_span.unwrap_or_else(|| c.name_span),
                            is_mut: false,
                            span: c.span,
                        },
                    );

                if HirTyId::from(expr_ty) != ty {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", expr_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(c.value.start()),
                            c.value.end() - c.value.start(),
                        ),
                        expected_type: format!("{:?}", c.ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(c.span.start()),
                            c.name_span.end() - c.span.start(),
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(())
            }
            HirStatement::Let(l) => {
                let expr_ty = self.check_expr(&mut l.value)?;
                let var_ty = l.ty.unwrap_or_else(|| expr_ty);
                l.ty = Some(var_ty);
                let ty = HirTyId::from(var_ty);
                self.context
                    .last_mut()
                    .unwrap()
                    .get_mut(self.current_func_name.unwrap())
                    .unwrap()
                    .insert(
                        l.name,
                        ContextVariable {
                            _name: l.name,
                            name_span: l.name_span,
                            ty: var_ty,
                            ty_span: l.ty_span.unwrap_or_else(|| l.name_span),
                            is_mut: true,
                            span: l.span,
                        },
                    );
                if HirTyId::from(expr_ty) != ty {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", expr_ty),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(l.value.start()),
                            l.value.end() - l.value.start(),
                        ),
                        expected_type: format!("{:?}", l.ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(l.name_span.start()),
                            l.name_span.end() - l.name_span.start(),
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(())
            }
            _ => {
                todo!("TypeChecker::check_stmt")
            }
        }
    }
    pub fn check_expr(&mut self, expr: &mut HirExpr<'hir>) -> HirResult<&'hir HirTy<'hir>> {
        match expr {
            HirExpr::IntegerLiteral(_) => Ok(self.arena.types().get_integer64_ty()),
            HirExpr::FloatLiteral(_) => Ok(self.arena.types().get_float64_ty()),
            HirExpr::UnsignedIntegerLiteral(_) => Ok(self.arena.types().get_uint64_ty()),
            HirExpr::BooleanLiteral(_) => Ok(self.arena.types().get_boolean_ty()),
            HirExpr::Unary(u) => {
                let ty = self.check_expr(&mut u.expr)?;
                match u.op {
                    Some(expr::UnaryOp::Neg) => {
                        if HirTyId::from(ty) != HirTyId::compute_integer64_ty_id() {
                            return Err(HirError::TryingToNegateUnsigned(
                                TryingToNegateUnsignedError {
                                    span: SourceSpan::new(
                                        SourceOffset::from(u.expr.start()),
                                        u.expr.end() - u.expr.start(),
                                    ),
                                    src: self.src.clone(),
                                },
                            ));
                        }
                        Ok(ty)
                    }
                    _ => Ok(ty),
                }
            }
            HirExpr::HirBinaryOp(b) => {
                let lhs = self.check_expr(&mut b.lhs)?;
                b.ty = lhs;
                let rhs = self.check_expr(&mut b.rhs)?;
                if HirTyId::from(lhs) != HirTyId::from(rhs) {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", lhs),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(b.lhs.start()),
                            b.lhs.end() - b.lhs.start(),
                        ),
                        expected_type: format!("{:?}", rhs),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(b.rhs.start()),
                            b.rhs.end() - b.rhs.start(),
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
            //Todo, add support for extern func
            HirExpr::Call(f) => {
                let callee = f.callee.as_ref();
                let name = match callee {
                    //todo: apply the type to the callee (i.ty)
                    HirExpr::Ident(i) => i.name,
                    _ => {
                        todo!("TypeChecker::check_expr")
                    }
                };
                let func = match self.signature.functions.get(name) {
                    Some(f) => *f,
                    None => {
                        return Err(HirError::UnknownType(UnknownTypeError {
                            name: name.to_string(),
                            span: SourceSpan::new(
                                SourceOffset::from(f.span.start()),
                                f.span.end() - f.span.start(),
                            ),
                            src: self.src.clone(),
                        }))
                    }
                };

                if func.params.len() != f.args.len() {
                    return Err(HirError::FunctionTypeMismatch(FunctionTypeMismatchError {
                        expected_ty: format!("{:?}", func),
                        span: SourceSpan::new(
                            SourceOffset::from(f.span.start()),
                            f.span.end() - f.span.start(),
                        ),
                        src: self.src.clone(),
                    }));
                }

                for (param, arg) in func.params.iter().zip(f.args.iter_mut()) {
                    let arg_ty = self.check_expr(arg)?;
                    if HirTyId::from(arg_ty) != HirTyId::from(param.ty) {
                        return Err(HirError::TypeMismatch(TypeMismatchError {
                            actual_type: format!("{:?}", arg_ty),
                            actual_loc: SourceSpan::new(
                                SourceOffset::from(arg.start()),
                                arg.end() - arg.start(),
                            ),
                            expected_type: format!("{:?}", param.ty),
                            expected_loc: SourceSpan::new(
                                SourceOffset::from(param.span.start()),
                                param.span.end() - param.span.start(),
                            ),
                            src: self.src.clone(),
                        }));
                    }
                }

                Ok(func.return_ty)
            }
            //todo: this should have its own function
            HirExpr::Assign(a) => {
                let rhs = self.check_expr(&mut a.rhs)?;
                let lhs = match a.lhs.as_mut() {
                    HirExpr::Ident(i) => {
                        match self
                            .context
                            .last()
                            .unwrap()
                            .get(self.current_func_name.unwrap())
                        {
                            Some(ctx_func) => {
                                let ctx_var = ctx_func.get(i.name).unwrap();
                                println!(
                                    "Changing type of {} from {} to {}",
                                    i.name, ctx_var.ty, rhs
                                );
                                i.ty = ctx_var.ty;
                                if !ctx_var.is_mut {
                                    return Err(HirError::TryingToMutateImmutableVariable(
                                        TryingToMutateImmutableVariableError {
                                            const_loc: SourceSpan::new(
                                                SourceOffset::from(ctx_var.span.start()),
                                                ctx_var.name_span.end() - ctx_var.span.start(),
                                            ),
                                            var_name: i.name.to_string(),
                                            span: SourceSpan::new(
                                                SourceOffset::from(a.span.start()),
                                                a.span.end() - a.span.start(),
                                            ),
                                            src: self.src.clone(),
                                        },
                                    ));
                                }
                                ctx_var
                            }
                            None => {
                                return Err(HirError::UnknownType(UnknownTypeError {
                                    name: i.name.to_string(),
                                    span: SourceSpan::new(
                                        SourceOffset::from(i.span.start()),
                                        i.span.end() - i.span.start(),
                                    ),
                                    src: self.src.clone(),
                                }))
                            }
                        }
                    }
                    _ => {
                        todo!("TypeChecker::check_expr")
                    }
                };

                if HirTyId::from(lhs.ty) != HirTyId::from(rhs) {
                    Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", rhs),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(a.lhs.start()),
                            a.rhs.end() - a.lhs.start(),
                        ),
                        expected_type: format!("{:?}", lhs.ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(lhs.name_span.start()),
                            lhs.ty_span.end() - lhs.name_span.start(),
                        ),
                        src: self.src.clone(),
                    }))
                } else {
                    Ok(lhs.ty)
                }
            }
            HirExpr::Ident(i) => {
                if let Some(ctx_var) = self
                    .context
                    .last()
                    .unwrap()
                    .get(self.current_func_name.unwrap())
                    .unwrap()
                    .get(i.name)
                {
                    println!(
                        "Changing type of {} from {} to {}",
                        i.name, i.ty, ctx_var.ty
                    );
                    i.ty = ctx_var.ty;
                    Ok(ctx_var.ty)
                } else {
                    Err(HirError::UnknownType(UnknownTypeError {
                        name: i.name.to_string(),
                        span: SourceSpan::new(
                            SourceOffset::from(i.span.start()),
                            i.span.end() - i.span.start(),
                        ),
                        src: self.src.clone(),
                    }))
                }
            }
            _ => {
                todo!("TypeChecker::check_expr")
            }
        }
    }
}
