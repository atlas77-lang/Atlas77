/// Contains the definition of the CodeGenArena
pub mod arena;

use crate::{
    atlas_hir::{
        error::{HirResult, UnsupportedExpr, UnsupportedStatement},
        expr::HirExpr,
        signature::HirFunctionParameterSignature,
        stmt::{HirBlock, HirStatement},
        ty::HirTy,
        HirModule,
    },
    atlas_vm::instruction::{ImportedLibrary, Instruction, Label, Program},
};

use arena::CodeGenArena;
use atlas_core::prelude::Spanned;
use miette::{SourceOffset, SourceSpan};

/// Result of codegen
pub(crate) type CodegenResult<T> = Result<T, crate::atlas_hir::error::HirError>;

/// Unit of codegen
pub(crate) struct CodeGenUnit<'hir, 'gen>
where
    'gen: 'hir,
{
    hir: HirModule<'hir>,
    program: Program<'gen>,
    arena: CodeGenArena<'gen>,
    current_pos: usize,
}

impl<'hir, 'gen> CodeGenUnit<'hir, 'gen>
where
    'gen: 'hir,
{
    /// Create a new CodeGenUnit
    pub(crate) fn new(hir: HirModule<'hir>, arena: CodeGenArena<'gen>) -> Self {
        Self {
            hir,
            program: Program::new(),
            current_pos: 0,
            arena,
        }
    }
    /// Take the HIR and convert it to a VM representation
    pub(crate) fn compile(&mut self) -> CodegenResult<Program> {
        let mut labels: Vec<Label> = Vec::new();
        for func in self.hir.body.functions.clone() {
            let mut bytecode = Vec::new();

            let params = func.1.signature.params.clone();
            Self::generate_bytecode_args(params, &mut bytecode)?;
            Self::generate_bytecode_block(&func.1.body, &mut bytecode)?;

            let len = bytecode.len();
            if func.0 == "main" {
                bytecode.push(Instruction::Halt);
            }

            labels.push(Label {
                name: func.0.to_string(),
                position: self.current_pos,
                body: self.arena.alloc_vec(bytecode),
            });

            self.current_pos += len;
        }
        self.program.entry_point = "main";
        self.program.labels = self.arena.alloc_vec(labels);
        let libraries = self
            .hir
            .body
            .imports
            .iter()
            .map(|l| ImportedLibrary {
                name: l.path.to_string(),
                is_std: true,
            })
            .collect::<Vec<_>>();
        self.program.libraries = self.arena.alloc_vec(libraries);
        Ok(self.program)
    }

    fn generate_bytecode_block(
        block: &HirBlock<'hir>,
        bytecode: &mut Vec<Instruction>,
    ) -> HirResult<()> {
        for stmt in &block.statements {
            Self::generate_bytecode_stmt(stmt, bytecode)?;
        }
        Ok(())
    }

    fn generate_bytecode_stmt(
        stmt: &HirStatement<'hir>,
        bytecode: &mut Vec<Instruction>,
    ) -> HirResult<()> {
        match stmt {
            HirStatement::Return(e) => {
                Self::generate_bytecode_expr(e.value, bytecode)?;
                bytecode.push(Instruction::Return);
            }
            HirStatement::IfElse(i) => {
                Self::generate_bytecode_expr(i.condition, bytecode)?;
                let mut then_body = Vec::new();
                Self::generate_bytecode_block(i.then_branch, &mut then_body)?;

                bytecode.push(Instruction::JmpZ {
                    pos: (then_body.len() + if i.else_branch.is_some() { 1 } else { 0 }) as isize,
                });
                bytecode.append(&mut then_body);
                if let Some(e) = i.else_branch {
                    let mut else_body = Vec::new();
                    Self::generate_bytecode_block(e, &mut else_body)?;

                    bytecode.push(Instruction::Jmp {
                        pos: (else_body.len() + 1) as isize,
                    });
                    bytecode.append(&mut else_body);
                }
            }
            HirStatement::While(w) => {
                let start = bytecode.len() as isize;
                Self::generate_bytecode_expr(w.condition, bytecode)?;
                let mut body = Vec::new();

                Self::generate_bytecode_block(w.body, &mut body)?;
                //If the condition is false jump to the end of the loop
                bytecode.push(Instruction::JmpZ {
                    pos: (body.len() + 1) as isize,
                });
                bytecode.append(&mut body);
                //Jump back to the start of the loop
                bytecode.push(Instruction::Jmp {
                    pos: start - bytecode.len() as isize,
                });
            }
            HirStatement::Let(l) => {
                let mut value = Vec::new();
                Self::generate_bytecode_expr(l.value, &mut value)?;
                match l.ty {
                    HirTy::Int64(_) => {
                        value.push(Instruction::StoreI64 {
                            var_name: l.name.to_string(),
                        });
                        bytecode.append(&mut value);
                    }
                    _ => unimplemented!("Unsupported type for now"),
                }
            }
            HirStatement::Expr(e) => Self::generate_bytecode_expr(e.expr, bytecode)?,
            _ => {
                return Err(crate::atlas_hir::error::HirError::UnsupportedStatement(
                    UnsupportedStatement {
                        span: SourceSpan::new(
                            SourceOffset::from(stmt.span().start()),
                            stmt.span().end() - stmt.span().start(),
                        ),
                        stmt: format!("{:?}", stmt),
                    },
                ))
            }
        }
        Ok(())
    }

    fn generate_bytecode_expr(
        expr: &HirExpr<'hir>,
        bytecode: &mut Vec<Instruction>,
    ) -> HirResult<()> {
        match expr {
            HirExpr::Assign(a) => {
                let lhs = a.lhs.as_ref();
                match lhs {
                    HirExpr::Ident(i) => {
                        Self::generate_bytecode_expr(&a.rhs, bytecode)?;
                        match i.ty {
                            HirTy::Int64(_) => {
                                bytecode.push(Instruction::StoreI64 {
                                    var_name: i.name.to_string(),
                                });
                            }
                            HirTy::Float64(_) => {
                                bytecode.push(Instruction::StoreF64 {
                                    var_name: i.name.to_string(),
                                });
                            }
                            HirTy::UInt64(_) => {
                                bytecode.push(Instruction::StoreU64 {
                                    var_name: i.name.to_string(),
                                });
                            }
                            HirTy::Uninitialized(_) => {
                                bytecode.push(Instruction::StoreI64 {
                                    var_name: i.name.to_string(),
                                });
                            }
                            _ => unimplemented!("Unsupported type for now {:?}", i.ty),
                        }
                    }
                    _ => {
                        return Err(crate::atlas_hir::error::HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start()),
                                    expr.span().end() - expr.span().start(),
                                ),
                                expr: format!("{:?}", expr),
                            },
                        ));
                    }
                }
            }
            HirExpr::HirBinaryOp(b) => {
                Self::generate_bytecode_expr(&b.lhs, bytecode)?;
                Self::generate_bytecode_expr(&b.rhs, bytecode)?;
                match b.op {
                    crate::atlas_hir::expr::HirBinaryOp::Add => {
                        bytecode.push(Instruction::AddI64);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Sub => {
                        bytecode.push(Instruction::SubI64);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Mul => {
                        bytecode.push(Instruction::MulI64);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Div => {
                        bytecode.push(Instruction::DivI64);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Mod => {
                        bytecode.push(Instruction::ModI64);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Eq => {
                        bytecode.push(Instruction::Eq);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Neq => {
                        bytecode.push(Instruction::Neq);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Gt => {
                        bytecode.push(Instruction::Gt);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Gte => {
                        bytecode.push(Instruction::Gte);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Lt => {
                        bytecode.push(Instruction::Lt);
                    }
                    crate::atlas_hir::expr::HirBinaryOp::Lte => {
                        bytecode.push(Instruction::Lte);
                    }
                    _ => unimplemented!("Unsupported binary operator for now"),
                }
            }
            HirExpr::Unary(u) => {
                //The operator are not yet implemented
                Self::generate_bytecode_expr(&u.expr, bytecode)?;
            }
            //This need to be thoroughly tested
            HirExpr::Call(f) => {
                for arg in &f.args {
                    Self::generate_bytecode_expr(arg, bytecode)?;
                }
                let callee = f.callee.as_ref();
                match callee {
                    HirExpr::Ident(i) => {
                        if i.name == "print" || i.name == "println" {
                            bytecode.push(Instruction::ExternCall {
                                name: i.name.to_string(),
                                args: f.args.len() as u8,
                            });
                        } else {
                            bytecode.push(Instruction::CallFunction {
                                name: i.name.to_string(),
                                args: f.args.len() as u8,
                            });
                        }
                    }
                    _ => {
                        return Err(crate::atlas_hir::error::HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start()),
                                    expr.span().end() - expr.span().start(),
                                ),
                                expr: format!("Can't call from: {:?}", expr),
                            },
                        ))
                    }
                }
            }
            HirExpr::Ident(i) => match i.ty {
                HirTy::Int64(_) => {
                    bytecode.push(Instruction::LoadI64 {
                        var_name: i.name.to_string(),
                    });
                }
                HirTy::Float64(_) => {
                    bytecode.push(Instruction::LoadF64 {
                        var_name: i.name.to_string(),
                    });
                }
                HirTy::UInt64(_) => {
                    bytecode.push(Instruction::LoadU64 {
                        var_name: i.name.to_string(),
                    });
                }
                //By default it will be an integer
                _ => bytecode.push(Instruction::LoadI64 {
                    var_name: i.name.to_string(),
                }),
            },
            HirExpr::IntegerLiteral(i) => bytecode.push(Instruction::PushInt(i.value)),
            HirExpr::FloatLiteral(f) => bytecode.push(Instruction::PushFloat(f.value)),
            HirExpr::UnsignedIntegererLiteral(u) => {
                bytecode.push(Instruction::PushUnsignedInt(u.value))
            }
            _ => {
                return Err(crate::atlas_hir::error::HirError::UnsupportedExpr(
                    UnsupportedExpr {
                        span: SourceSpan::new(
                            SourceOffset::from(expr.span().start()),
                            expr.span().end() - expr.span().start(),
                        ),
                        expr: format!("{:?}", expr),
                    },
                ))
            }
        }
        Ok(())
    }

    fn generate_bytecode_args(
        args: Vec<&HirFunctionParameterSignature<'hir>>,
        bytecode: &mut Vec<Instruction>,
    ) -> HirResult<()> {
        let args = args.iter().rev().cloned().collect::<Vec<_>>();
        for arg in args {
            match arg.ty {
                HirTy::Int64(_) => {
                    bytecode.push(Instruction::StoreI64 {
                        var_name: arg.name.to_string(),
                    });
                }
                HirTy::Float64(_) => {
                    bytecode.push(Instruction::StoreF64 {
                        var_name: arg.name.to_string(),
                    });
                }
                HirTy::UInt64(_) => {
                    bytecode.push(Instruction::StoreU64 {
                        var_name: arg.name.to_string(),
                    });
                }
                _ => unimplemented!("Unsupported argument type for now"),
            }
        }
        Ok(())
    }
}
