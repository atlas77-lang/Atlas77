//The first view versions of the type checking will be quite simple.
//As there will only be primitive types to check.
//A rework of the type checker will be done when structs, classes, enums and unions are added.

use std::collections::HashMap;

use atlas_core::prelude::{Span, Spanned};
use miette::{SourceOffset, SourceSpan};

use crate::atlas_hir::expr;

use super::{
    arena::HirArena,
    error::{
        FunctionTypeMismatchError, HirError, HirResult, TryingToMutateImmutableVariableError,
        TryingToNegateUnsignedError, TypeMismatchError, UnknownTypeError,
    },
    expr::{HirBinaryOp, HirExpr},
    stmt::HirStatement,
    ty::{HirTy, HirTyId},
    HirFunction, HirModule, HirModuleSignature,
};

pub(crate) struct TypeChecker<'hir> {
    arena: &'hir HirArena<'hir>,
    ///Keep track of the scopes and variables
    ///
    /// Should be rework in the future, variables should only be represented as (usize, usize)
    ///  (i.e. (scope, var_name) var_name being in the arena)
    context: Vec<HashMap<String, ContextVariable<'hir>>>, //bool is for mutability true=mut false=const
    signature: HirModuleSignature<'hir>,
    current_func_name: Option<&'hir str>,
    // Source code
    src: String,
}

pub(crate) struct ContextVariable<'hir> {
    pub name: &'hir str,
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

    pub fn check(&mut self, hir: &HirModule<'hir>) -> HirResult<()> {
        self.signature = hir.signature.clone();
        for func in &hir.body.functions {
            self.current_func_name = Some(func.0);
            self.check_func(func.1)?;
        }
        Ok(())
    }

    pub fn check_func(&mut self, func: &HirFunction<'hir>) -> HirResult<()> {
        self.context.push(HashMap::new());
        for param in &func.signature.params {
            self.context.last_mut().unwrap().insert(
                param.name.to_string(),
                ContextVariable {
                    name: param.name,
                    name_span: param.span,
                    ty: param.ty,
                    ty_span: param.ty_span,
                    is_mut: false,
                    span: param.span,
                },
            );
        }
        for stmt in &func.body.statements {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }
    pub fn check_stmt(&mut self, stmt: &HirStatement<'hir>) -> HirResult<()> {
        match stmt {
            HirStatement::Expr(e) => {
                self.check_expr(e.expr)?;
                Ok(())
            }
            HirStatement::Return(r) => {
                let actual_ret_ty = self.check_expr(&r.value)?;
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
            HirStatement::IfElse(i) => {
                let cond_ty = self.check_expr(&i.condition)?;
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
                self.context.push(HashMap::new());
                for stmt in &i.then_branch.statements {
                    self.check_stmt(stmt)?;
                }
                self.context.pop();
                if let Some(else_branch) = &i.else_branch {
                    self.context.push(HashMap::new());
                    for stmt in &else_branch.statements {
                        self.check_stmt(stmt)?;
                    }
                    self.context.pop();
                }
                Ok(())
            }
            HirStatement::Const(c) => {
                let ty = HirTyId::from(c.ty);
                self.context.last_mut().unwrap().insert(
                    c.name.to_string(),
                    ContextVariable {
                        name: c.name,
                        name_span: c.name_span,
                        ty: c.ty,
                        ty_span: c.ty_span,
                        is_mut: false,
                        span: c.span,
                    },
                );
                let ty_value = self.check_expr(&c.value)?;
                if HirTyId::from(ty_value) != ty {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", ty_value),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(c.value.start()),
                            c.value.end() - c.value.start(),
                        ),
                        expected_type: format!("{:?}", c.ty),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(c.name_span.start()),
                            c.name_span.end() - c.name_span.start(),
                        ),
                        src: self.src.clone(),
                    }));
                }
                Ok(())
            }
            HirStatement::Let(l) => {
                let ty = HirTyId::from(l.ty);
                self.context.last_mut().unwrap().insert(
                    l.name.to_string(),
                    ContextVariable {
                        name: l.name,
                        name_span: l.name_span,
                        ty: l.ty,
                        ty_span: l.ty_span,
                        is_mut: true,
                        span: l.span,
                    },
                );
                let ty_value = self.check_expr(&l.value)?;
                if HirTyId::from(ty_value) != ty {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", ty_value),
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
                return Ok(());
            }
            _ => {
                todo!("TypeChecker::check_stmt")
            }
        }
    }
    pub fn check_expr(&mut self, expr: &HirExpr<'hir>) -> HirResult<&'hir HirTy<'hir>> {
        match expr {
            HirExpr::IntegerLiteral(_) => Ok(self.arena.types().get_integer64_ty()),
            HirExpr::FloatLiteral(_) => Ok(self.arena.types().get_float64_ty()),
            HirExpr::UnsignedIntegererLiteral(_) => Ok(self.arena.types().get_uint64_ty()),
            HirExpr::BooleanLiteral(_) => Ok(self.arena.types().get_boolean_ty()),
            HirExpr::Unary(u) => {
                let ty = self.check_expr(&u.expr)?;
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
                let lhs = self.check_expr(&b.lhs)?;
                let rhs = self.check_expr(&b.rhs)?;
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
                //Should handle conditions:

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
            HirExpr::Call(f) => {
                let callee = f.callee.as_ref();
                let name = match callee {
                    HirExpr::Ident(i) => i.name,
                    _ => {
                        todo!("TypeChecker::check_expr")
                    }
                };
                let func = *self.signature.functions.get(name).unwrap();

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

                for (param, arg) in func.params.iter().zip(f.args.iter()) {
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
            HirExpr::Assign(a) => {
                let lhs = match a.lhs.as_ref() {
                    HirExpr::Ident(i) => match self.context.last().unwrap().get(i.name) {
                        Some(ctx_var) => {
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
                            } else {
                                ctx_var.ty
                            }
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
                    },
                    _ => {
                        todo!("TypeChecker::check_expr")
                    }
                };
                let rhs = self.check_expr(&a.rhs)?;
                if HirTyId::from(lhs) != HirTyId::from(rhs) {
                    return Err(HirError::TypeMismatch(TypeMismatchError {
                        actual_type: format!("{:?}", lhs),
                        actual_loc: SourceSpan::new(
                            SourceOffset::from(a.lhs.start()),
                            a.lhs.end() - a.lhs.start(),
                        ),
                        expected_type: format!("{:?}", rhs),
                        expected_loc: SourceSpan::new(
                            SourceOffset::from(a.rhs.start()),
                            a.rhs.end() - a.rhs.start(),
                        ),
                        src: self.src.clone(),
                    }));
                } else {
                    Ok(lhs)
                }
            }
            HirExpr::Ident(i) => {
                if let Some(ctx_var) = self.context.last().unwrap().get(i.name) {
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
