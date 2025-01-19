use atlas_core::prelude::{Span, Spanned};
use miette::{SourceOffset, SourceSpan};


use atlas_frontend::{
        parse,
        parser::{
            arena::AstArena,
            ast::{
                AstBinaryOp, AstBlock, AstExpr, AstFunction, AstImport, AstItem, AstLiteral,
                AstObjField, AstProgram, AstStatement, AstType, AstUnaryOp,
            },
        },
    };
use crate::{expr::HirAssignExpr, signature::HirFunctionSignature, stmt::HirLetStmt};

const FILE_ATLAS: &str = include_str!("../../../../library/std/file.atlas");
const IO_ATLAS: &str = include_str!("../../../../library/std/io.atlas");
const MATH_ATLAS: &str = include_str!("../../../../library/std/math.atlas");
const STRING_ATLAS: &str = include_str!("../../../../library/std/string.atlas");


use crate::{
    arena::HirArena,
    error::{HirError, HirResult, UnsupportedExpr, UnsupportedStatement},
    expr::{
        HirBinaryOp, HirBinaryOpExpr, HirBooleanLiteralExpr, HirExpr, HirFloatLiteralExpr,
        HirFunctionCallExpr, HirIdentExpr, HirIntegerLiteralExpr, HirUnsignedIntegerLiteralExpr,
        UnaryOp, UnaryOpExpr,
    },
    item::HirFunction,
    signature::{HirFunctionParameterSignature, HirModuleSignature, HirTypeParameterItemSignature},
    stmt::{HirBlock, HirExprStmt, HirIfElseStmt, HirReturn, HirStatement, HirWhileStmt},
    ty::HirTy,
    HirImport, HirModule, HirModuleBody,
};

pub struct AstSyntaxLoweringPass<'ast, 'hir> {
    arena: &'hir HirArena<'hir>,
    ast: &'ast AstProgram<'ast>,
    ast_arena: &'ast AstArena<'ast>,
    //source code
    src: String,
}

impl<'ast, 'hir> AstSyntaxLoweringPass<'ast, 'hir> {
    pub fn new(
        arena: &'hir HirArena<'hir>,
        ast: &'ast AstProgram,
        ast_arena: &'ast AstArena<'ast>,
        src: String,
    ) -> Self {
        Self {
            arena,
            ast,
            ast_arena,
            src,
        }
    }
}

impl<'ast, 'hir> AstSyntaxLoweringPass<'ast, 'hir>
where
    'ast: 'hir,
{
    pub fn lower(&self) -> HirResult<HirModule> {
        let mut module_body = HirModuleBody::default();
        let mut module_signature = HirModuleSignature::default();

        let mut items = Vec::new();
        for item in self.ast.items {
            items.push(self.visit_item(&mut module_body, &mut module_signature, item)?);
        }
        //println!("{:#?}", module_signature);
        Ok(HirModule {
            body: module_body,
            signature: module_signature,
        })
    }
    pub fn visit_item(
        &self,
        module_body: &mut HirModuleBody<'hir>,
        module_signature: &mut HirModuleSignature<'hir>,
        item: &'ast AstItem<'ast>,
    ) -> HirResult<()> {
        match item {
            AstItem::Func(f) => {
                let fun = self.visit_func(f)?;
                let name = self.arena.names().get(f.name.name);
                module_signature.functions.insert(name, fun.signature);
                module_body.functions.insert(name, fun);
            }
            AstItem::Import(i) => {
                let hir = self.visit_import(i)?;
                let allocated_hir: &'hir HirModule<'hir> = self.arena.intern(hir);
                for (name, signature) in allocated_hir.signature.functions.iter() {
                    module_signature.functions.insert(name, *signature);
                }
                allocated_hir.body.imports.iter().for_each(|i| {
                    module_body.imports.push(i);
                });
            }
            AstItem::ExternFunction(e) => {
                let name = self.arena.names().get(e.name.name);
                let ty = self.visit_ty(e.ret)?;

                let mut params: Vec<&HirFunctionParameterSignature<'hir>> = Vec::new();
                let mut type_params: Vec<&HirTypeParameterItemSignature<'_>> = Vec::new();

                for (arg_name, arg_ty) in e.args_name.iter().zip(e.args_ty.iter()) {
                    let hir_arg_ty = self.visit_ty(arg_ty)?;
                    let hir_arg_name = self.arena.names().get(arg_name.name);

                    params.push(self.arena.intern(HirFunctionParameterSignature {
                        span: arg_name.span,
                        name: hir_arg_name,
                        name_span: arg_name.span,
                        ty: hir_arg_ty,
                        ty_span: arg_ty.span(),
                    }));

                    type_params.push(self.arena.intern(HirTypeParameterItemSignature {
                        span: arg_name.span,
                        name: hir_arg_name,
                        name_span: arg_name.span,
                    }));
                }
                let hir = self.arena.intern(HirFunctionSignature {
                    span: e.span,
                    params,
                    type_params,
                    return_ty: ty,
                    return_ty_span: Some(e.ret.span()),
                    is_external: true,
                });
                module_signature.functions.insert(name, hir);
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_import(&self, node: &'ast AstImport<'ast>) -> HirResult<HirModule<'hir>> {
        match node.path.split("/").last().unwrap() {
            "io" => {
                let ast: AstProgram<'ast> = parse(
                    "atlas_stdlib/io.atlas",
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                )
                .unwrap();
                let allocated_ast = self.ast_arena.alloc(ast);
                let hir = self.arena.intern(AstSyntaxLoweringPass::<'ast, 'hir>::new(
                    self.arena,
                    allocated_ast,
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                ));
                let mut lower = hir.lower()?;

                let hir_import: &'hir HirImport<'_> = self.arena.intern(HirImport {
                    span: node.span,
                    path: node.path,
                    path_span: node.span,
                    alias: None,
                    alias_span: None,
                });

                lower.body.imports.push(hir_import);

                Ok(lower)
            }
            "math" => {
                let ast: AstProgram<'ast> = parse(
                    "atlas_stdlib/math.atlas",
                    self.ast_arena,
                    MATH_ATLAS.to_string(),
                )
                .unwrap();
                let allocated_ast = self.ast_arena.alloc(ast);
                let hir = self.arena.intern(AstSyntaxLoweringPass::<'ast, 'hir>::new(
                    self.arena,
                    allocated_ast,
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                ));
                hir.lower()
            }
            "file" => {
                let ast: AstProgram<'ast> = parse(
                    "atlas_stdlib/file.atlas",
                    self.ast_arena,
                    FILE_ATLAS.to_string(),
                )
                .unwrap();
                let allocated_ast = self.ast_arena.alloc(ast);
                let hir = self.arena.intern(AstSyntaxLoweringPass::<'ast, 'hir>::new(
                    self.arena,
                    allocated_ast,
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                ));
                hir.lower()
            }
            "list" => {
                let ast: AstProgram<'ast> = parse(
                    "atlas_stdlib/list.atlas",
                    self.ast_arena,
                    FILE_ATLAS.to_string(),
                )
                .unwrap();
                let allocated_ast = self.ast_arena.alloc(ast);
                let hir = self.arena.intern(AstSyntaxLoweringPass::<'ast, 'hir>::new(
                    self.arena,
                    allocated_ast,
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                ));
                hir.lower()
            }
            "string" => {
                let ast: AstProgram<'ast> = parse(
                    "atlas_stdlib/string.atlas",
                    self.ast_arena,
                    STRING_ATLAS.to_string(),
                )
                .unwrap();
                let allocated_ast = self.ast_arena.alloc(ast);
                let hir = self.arena.intern(AstSyntaxLoweringPass::<'ast, 'hir>::new(
                    self.arena,
                    allocated_ast,
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                ));
                hir.lower()
            }
            "time" => {
                let ast: AstProgram<'ast> = parse(
                    "atlas_stdlib/time.atlas",
                    self.ast_arena,
                    STRING_ATLAS.to_string(),
                )
                .unwrap();
                let allocated_ast = self.ast_arena.alloc(ast);
                let hir = self.arena.intern(AstSyntaxLoweringPass::<'ast, 'hir>::new(
                    self.arena,
                    allocated_ast,
                    self.ast_arena,
                    IO_ATLAS.to_string(),
                ));
                hir.lower()
            }
            _ => Err(HirError::UnsupportedStatement(UnsupportedStatement {
                span: SourceSpan::new(
                    SourceOffset::from(node.span.start()),
                    node.span.end() - node.span.start(),
                ),
                stmt: format!("{:?}", node),
                src: self.src.clone(),
            })),
        }
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
            AstStatement::Const(c) => {
                let name = self.arena.names().get(c.name.name);
                let ty = match c.ty {
                    Some(ty) => self.visit_ty(ty)?,
                    None => self.arena.types().get_uninitialized_ty(),
                };
                let value = self.visit_expr(c.value)?;
                let hir = self.arena.intern(HirStatement::Const(HirLetStmt {
                    span: node.span(),
                    name,
                    name_span: c.name.span,
                    ty,
                    ty_span: c.ty.unwrap().span(),
                    value,
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
                    ty_span: l.ty.unwrap().span(),
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
                    src: self.src.clone(),
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
                    AstLiteral::Boolean(b) => {
                        self.arena
                            .intern(HirExpr::BooleanLiteral(HirBooleanLiteralExpr {
                                span: l.span(),
                                value: b.value,
                                ty: self.arena.types().get_boolean_ty(),
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
                            src: self.src.clone(),
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
                src: self.src.clone(),
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
