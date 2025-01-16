use atlas_core::prelude::{Span, Spanned};
use miette::{SourceOffset, SourceSpan};

use crate::{
    atlas_frontend::parser::ast::{
        AstBinaryOp, AstBlock, AstExpr, AstFunction, AstItem, AstLiteral, AstObjField, AstProgram,
        AstStatement, AstType, AstUnaryOp,
    },
    atlas_hir::{
        expr::HirAssignExpr, item::HirImport, signature::HirFunctionSignature, stmt::HirLetStmt,
    },
};

use super::{
    arena::HirArena,
    error::{HirResult, UnsupportedExpr, UnsupportedStatement},
    expr::{
        HirBinaryOp, HirBinaryOpExpr, HirExpr, HirFloatLiteralExpr, HirFunctionCallExpr,
        HirIdentExpr, HirIntegerLiteralExpr, HirUnsignedIntegerLiteralExpr, UnaryOp, UnaryOpExpr,
    },
    item::{HirFunction, HirItem},
    signature::{HirFunctionParameterSignature, HirModuleSignature, HirTypeParameterItemSignature},
    stmt::{HirBlock, HirExprStmt, HirIfElseStmt, HirReturn, HirStatement, HirWhileStmt},
    ty::HirTy,
    HirModule, HirModuleBody,
};

pub(crate) struct AstSyntaxLoweringPass<'ast, 'hir> {
    arena: &'hir HirArena<'hir>,
    ast: &'ast AstProgram<'ast>,
}

impl<'ast, 'hir> AstSyntaxLoweringPass<'ast, 'hir> {
    pub(crate) fn new(arena: &'hir HirArena<'hir>, ast: &'ast AstProgram) -> Self {
        Self { arena, ast }
    }
}

impl<'ast, 'hir> AstSyntaxLoweringPass<'ast, 'hir> {
    pub(crate) fn lower(&self) -> HirResult<HirModule> {
        let mut module_body = HirModuleBody::default();
        let mut module_signature = HirModuleSignature::default();

        let mut items = Vec::new();
        for item in self.ast.items {
            items.push(self.visit_item(&mut module_body, &mut module_signature, item)?);
        }
        Ok(HirModule {
            body: module_body,
            signature: module_signature,
        })
    }
    pub(crate) fn visit_item(
        &self,
        module_body: &mut HirModuleBody<'hir>,
        module_signature: &mut HirModuleSignature<'hir>,
        item: &'ast AstItem<'ast>,
    ) -> HirResult<HirItem<'hir>> {
        match item {
            AstItem::Func(f) => {
                let fun = self.visit_func(f)?;
                let name = self.arena.names().get(f.name.name);
                module_signature.functions.insert(name, fun.signature);
                module_body.functions.insert(name, fun);
            }
            AstItem::Import(i) => {
                let hir = self.arena.intern(HirImport {
                    span: i.span,
                    path: self.arena.names().get(i.path),
                    path_span: Span::empty(),
                    alias: None,
                    alias_span: None,
                });

                module_body.imports.push(hir);
            }
            _ => {}
        }
        Ok(HirItem::Function(HirFunction {
            span: Span::empty(),
            name: "",
            name_span: Span::empty(),
            signature: self.arena.intern(HirFunctionSignature {
                span: Span::empty(),
                params: Vec::new(),
                type_params: Vec::new(),
                return_ty: self.arena.types().get_uninitialized_ty(),
                return_ty_span: None,
                is_external: false,
            }),
            body: HirBlock {
                statements: Vec::new(),
                span: Span::empty(),
            },
        }))
    }

    fn visit_block(&self, node: &'ast AstBlock<'ast>) -> HirResult<HirBlock<'hir>> {
        let statements = node
            .stmts
            .iter()
            .map(|stmt| self.visit_stmt(stmt))
            .collect::<HirResult<Vec<_>>>()?;
        Ok(HirBlock {
            statements,
            span: node.span,
        })
    }

    fn visit_stmt(&self, node: &'ast AstStatement<'ast>) -> HirResult<&'hir HirStatement<'hir>> {
        match node {
            AstStatement::While(w) => {
                let condition = self.visit_expr(w.condition)?;
                let body = self.visit_block(w.body)?;
                let hir = self.arena.intern(HirStatement::While(HirWhileStmt {
                    span: node.span(),
                    condition,
                    body: self.arena.intern(body),
                }));
                Ok(hir)
            }
            AstStatement::Let(l) => {
                let name = self.arena.names().get(l.name.name);
                let ty = match l.ty {
                    Some(ty) => self.visit_ty(ty)?,
                    None => self.arena.types().get_uninitialized_ty(),
                };
                let value = self.visit_expr(l.value)?;
                let hir = self.arena.intern(HirStatement::Let(HirLetStmt {
                    span: node.span(),
                    name,
                    name_span: l.name.span,
                    ty,
                    value,
                }));
                Ok(hir)
            }
            AstStatement::IfElse(i) => {
                let condition = self.visit_expr(i.condition)?;
                let then_branch = self.arena.intern(self.visit_block(i.body)?);
                //If you don't type, the compiler will use it as an "Option<&mut HirBlock<'hir>>"
                //Which is dumb asf
                let else_branch: Option<&HirBlock<'hir>> = match i.else_body {
                    Some(else_body) => Some(self.arena.intern(self.visit_block(else_body)?)),
                    None => None,
                };
                let hir = self.arena.intern(HirStatement::IfElse(HirIfElseStmt {
                    span: node.span(),
                    condition,
                    then_branch,
                    else_branch,
                }));
                Ok(hir)
            }
            AstStatement::Break(b) => {
                let hir = self.arena.intern(HirStatement::Break(b.span));
                Ok(hir)
            }
            AstStatement::Continue(c) => {
                let hir = self.arena.intern(HirStatement::Continue(c.span));
                Ok(hir)
            }
            //The parser really need a bit of work
            AstStatement::Return(r) => {
                let expr = self.visit_expr(r.value)?;
                let hir = self.arena.intern(HirStatement::Return(HirReturn {
                    span: node.span(),
                    value: expr,
                    ty: expr.ty(),
                }));
                Ok(hir)
            }
            AstStatement::Expr(e) => {
                let expr = self.visit_expr(e)?;
                let hir = self.arena.intern(HirStatement::Expr(HirExprStmt {
                    span: node.span(),
                    expr,
                }));
                Ok(hir)
            }
            _ => Err(super::error::HirError::UnsupportedStatement(
                UnsupportedStatement {
                    span: SourceSpan::new(
                        SourceOffset::from(node.span().start()),
                        node.span().end() - node.span().start(),
                    ),
                    stmt: format!("{:?}", node),
                },
            )),
        }
    }

    fn visit_expr(&self, node: &'ast AstExpr<'ast>) -> HirResult<&'hir HirExpr<'hir>> {
        match node {
            AstExpr::Assign(a) => {
                let target = self.visit_expr(a.target)?;
                let value = self.visit_expr(a.value)?;
                let hir = self.arena.intern(HirExpr::Assign(HirAssignExpr {
                    span: node.span(),
                    lhs: Box::new(target.clone()),
                    rhs: Box::new(value.clone()),
                    ty: self.arena.types().get_uninitialized_ty(),
                }));
                Ok(hir)
            }
            AstExpr::BinaryOp(b) => {
                let lhs = self.visit_expr(b.lhs)?;
                let rhs = self.visit_expr(b.rhs)?;
                let op = self.visit_bin_op(&b.op)?;
                let hir = self.arena.intern(HirExpr::HirBinaryOp(HirBinaryOpExpr {
                    span: node.span(),
                    op,
                    op_span: Span::empty(),
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(rhs.clone()),
                    ty: self.arena.types().get_uninitialized_ty(),
                }));
                Ok(hir)
            }
            AstExpr::UnaryOp(u) => {
                let expr = self.visit_expr(u.expr)?;
                let hir = self.arena.intern(HirExpr::Unary(UnaryOpExpr {
                    span: node.span(),
                    op: match u.op {
                        Some(AstUnaryOp::Neg) => Some(UnaryOp::Neg),
                        Some(AstUnaryOp::Not) => Some(UnaryOp::Not),
                        _ => None,
                    },
                    expr: Box::new(expr.clone()),
                    ty: expr.ty(),
                }));
                Ok(hir)
            }
            AstExpr::Call(c) => {
                let callee = self.visit_expr(c.callee)?;
                let args = c
                    .args
                    .iter()
                    .map(|arg| self.visit_expr(arg).cloned())
                    .collect::<HirResult<Vec<_>>>()?;
                let hir = self.arena.intern(HirExpr::Call(HirFunctionCallExpr {
                    span: node.span(),
                    callee: Box::new(callee.clone()),
                    callee_span: callee.span(),
                    args,
                    args_ty: Vec::new(),
                    ty: self.arena.types().get_uninitialized_ty(),
                }));
                Ok(hir)
            }
            AstExpr::Identifier(i) => {
                let hir = self.arena.intern(HirExpr::Ident(HirIdentExpr {
                    name: self.arena.names().get(i.name),
                    span: i.span,
                    ty: self.arena.types().get_uninitialized_ty(),
                }));
                Ok(hir)
            }
            AstExpr::Literal(l) => {
                let hir = match l {
                    AstLiteral::Integer(i) => {
                        self.arena
                            .intern(HirExpr::IntegerLiteral(HirIntegerLiteralExpr {
                                span: l.span(),
                                value: i.value,
                                ty: self.arena.types().get_integer64_ty(),
                            }))
                    }
                    AstLiteral::Float(f) => {
                        self.arena
                            .intern(HirExpr::FloatLiteral(HirFloatLiteralExpr {
                                span: l.span(),
                                value: f.value,
                                ty: self.arena.types().get_float64_ty(),
                            }))
                    }
                    AstLiteral::UnsignedIntegerer(u) => self.arena.intern(
                        HirExpr::UnsignedIntegererLiteral(HirUnsignedIntegerLiteralExpr {
                            span: l.span(),
                            value: u.value,
                            ty: self.arena.types().get_uint64_ty(),
                        }),
                    ),
                    _ => {
                        return Err(super::error::HirError::UnsupportedExpr(UnsupportedExpr {
                            span: SourceSpan::new(
                                SourceOffset::from(node.span().start()),
                                node.span().end() - node.span().start(),
                            ),
                            expr: format!("{:?}", node),
                        }));
                    }
                };
                Ok(hir)
            }
            _ => Err(super::error::HirError::UnsupportedExpr(UnsupportedExpr {
                span: SourceSpan::new(
                    SourceOffset::from(node.span().start()),
                    node.span().end() - node.span().start(),
                ),
                expr: format!("{:?}", node),
            })),
        }
    }

    fn visit_bin_op(&self, bin_op: &'ast AstBinaryOp) -> HirResult<HirBinaryOp> {
        let op = match bin_op {
            AstBinaryOp::Add => HirBinaryOp::Add,
            AstBinaryOp::Sub => HirBinaryOp::Sub,
            AstBinaryOp::Mul => HirBinaryOp::Mul,
            AstBinaryOp::Div => HirBinaryOp::Div,
            AstBinaryOp::Mod => HirBinaryOp::Mod,
            AstBinaryOp::Eq => HirBinaryOp::Eq,
            AstBinaryOp::NEq => HirBinaryOp::Neq,
            AstBinaryOp::Lt => HirBinaryOp::Lt,
            AstBinaryOp::Lte => HirBinaryOp::Lte,
            AstBinaryOp::Gt => HirBinaryOp::Gt,
            AstBinaryOp::Gte => HirBinaryOp::Gte,
            //Other operators will soon come
        };
        Ok(op)
    }

    fn visit_func(&self, node: &'ast AstFunction<'ast>) -> HirResult<HirFunction<'hir>> {
        let type_parameters = node
            .args
            .iter()
            .map(|arg| self.visit_type_param_item(arg))
            .collect::<HirResult<Vec<_>>>();
        let ret_type_span = node.ret.span();
        let ret_type = self.visit_ty(node.ret)?;
        let parameters = node
            .args
            .iter()
            .map(|arg| self.visit_func_param(arg))
            .collect::<HirResult<Vec<_>>>();

        let body = self.visit_block(node.body)?;
        let signature = self.arena.intern(HirFunctionSignature {
            span: node.span,
            params: parameters?,
            type_params: type_parameters?,
            return_ty: ret_type,
            return_ty_span: Some(ret_type_span),
            is_external: false,
        });
        let fun = HirFunction {
            span: node.span,
            name: self.arena.names().get(node.name.name),
            name_span: node.name.span,
            signature,
            body,
        };
        Ok(fun)
    }

    fn visit_func_param(
        &self,
        node: &'ast AstObjField<'ast>,
    ) -> HirResult<&'hir HirFunctionParameterSignature<'hir>> {
        let name = self.arena.names().get(node.name.name);
        let ty = self.visit_ty(node.ty)?;

        let hir = self.arena.intern(HirFunctionParameterSignature {
            span: node.span,
            name,
            name_span: node.name.span,
            ty,
            ty_span: node.ty.span(),
        });
        Ok(hir)
    }

    fn visit_type_param_item(
        &self,
        node: &'ast AstObjField<'ast>,
    ) -> HirResult<&'hir HirTypeParameterItemSignature<'hir>> {
        let name = self.arena.names().get(node.name.name);

        let hir = self.arena.intern(HirTypeParameterItemSignature {
            span: node.span,
            name,
            name_span: node.name.span,
        });
        Ok(hir)
    }

    fn visit_ty(&self, node: &'ast AstType<'ast>) -> HirResult<&'hir HirTy<'hir>> {
        let ty = match node {
            AstType::Boolean(_) => self.arena.types().get_boolean_ty(),
            AstType::Integer(_) => self.arena.types().get_integer64_ty(),
            AstType::Float(_) => self.arena.types().get_float64_ty(),
            AstType::UnsignedIntegerer(_) => self.arena.types().get_uint64_ty(),
            AstType::Unit(_) => self.arena.types().get_unit_ty(),
            _ => unimplemented!("visit_ty, {:?}", node),
        };
        Ok(ty)
    }
}
