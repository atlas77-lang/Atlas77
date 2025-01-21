/// Contains the definition of the CodeGenArena
pub mod arena;
mod table;

use atlas_hir::{
    error::{HirResult, UnsupportedExpr, UnsupportedStatement},
    expr::HirExpr,
    signature::HirFunctionParameterSignature,
    stmt::{HirBlock, HirStatement},
    ty::HirTy,
    HirModule,
};
use atlas_vm::runtime::instruction::{ImportedLibrary, Instruction, Label, Program};

use crate::table::Table;
use arena::CodeGenArena;
use atlas_core::prelude::Spanned;
use miette::{SourceOffset, SourceSpan};

/// Result of codegen
pub type CodegenResult<T> = Result<T, atlas_hir::error::HirError>;

/// Unit of codegen
pub struct CodeGenUnit<'hir, 'gen>
where
    'gen: 'hir,
{
    hir: HirModule<'hir>,
    program: Program,
    arena: CodeGenArena<'gen>,
    //simulate a var_map so the codegen can translate it into stack operations
    _variables: Table<&'hir str>,
    //store the function position
    _global: Table<&'hir str>,
    current_pos: usize,
    //todo: remove this
    src: String,
}

impl<'hir, 'gen> CodeGenUnit<'hir, 'gen>
where
    'gen: 'hir,
{
    /// Create a new CodeGenUnit
    pub fn new(hir: HirModule<'hir>, arena: CodeGenArena<'gen>, src: String) -> Self {
        Self {
            hir,
            program: Program::new(),
            arena,
            _variables: Table::new(),
            _global: Table::new(),
            current_pos: 0,
            src,
        }
    }
    /// Take the HIR and convert it to a VM representation
    pub fn compile(&mut self) -> CodegenResult<Program> {
        let mut labels: Vec<Label> = Vec::new();
        for func in self.hir.body.functions.clone() {
            let mut bytecode = Vec::new();
            //self.global.insert(func.0);

            let params = func.1.signature.params.clone();
            self.generate_bytecode_args(params, &mut bytecode)?;
            self.generate_bytecode_block(&func.1.body, &mut bytecode, self.src.clone())?;

            if func.0 == "main" {
                bytecode.push(Instruction::Halt);
            }
            let len = bytecode.len();

            labels.push(Label {
                name: func.0.to_string(),
                position: self.current_pos,
                body: bytecode,
            });

            self.current_pos += len;
        }
        self.program.entry_point = String::from("main");
        self.program.labels = labels;
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
        self.program.libraries = libraries;
        /*for fn_name in self.global.into_iter() {}
        self.program.global.function_pool = self
            .global
            .items
            .iter()
            .map(|s| self.global.get_index(*s).unwrap())
            .collect::<Vec<_>>();*/
        Ok(self.program.clone())
    }

    fn generate_bytecode_block(
        &mut self,
        block: &HirBlock<'hir>,
        bytecode: &mut Vec<Instruction>,
        src: String,
    ) -> HirResult<()> {
        for stmt in &block.statements {
            self.generate_bytecode_stmt(stmt, bytecode, src.clone())?;
        }
        Ok(())
    }

    fn generate_bytecode_stmt(
        &mut self,
        stmt: &HirStatement<'hir>,
        bytecode: &mut Vec<Instruction>,
        src: String,
    ) -> HirResult<()> {
        match stmt {
            HirStatement::Return(e) => {
                self.generate_bytecode_expr(&e.value, bytecode, src)?;
                bytecode.push(Instruction::Return);
            }
            HirStatement::IfElse(i) => {
                self.generate_bytecode_expr(&i.condition, bytecode, src.clone())?;
                let mut then_body = Vec::new();
                self.generate_bytecode_block(&i.then_branch, &mut then_body, src.clone())?;

                bytecode.push(Instruction::JmpZ {
                    pos: (then_body.len() + if i.else_branch.is_some() { 1 } else { 0 }) as isize,
                });
                bytecode.append(&mut then_body);
                if let Some(e) = &i.else_branch {
                    let mut else_body = Vec::new();
                    self.generate_bytecode_block(e, &mut else_body, src)?;

                    bytecode.push(Instruction::Jmp {
                        pos: (else_body.len() + 1) as isize,
                    });
                    bytecode.append(&mut else_body);
                }
            }
            HirStatement::While(w) => {
                let start = bytecode.len() as isize;
                self.generate_bytecode_expr(&w.condition, bytecode, src.clone())?;
                let mut body = Vec::new();

                self.generate_bytecode_block(&w.body, &mut body, src)?;
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
                self.generate_bytecode_expr(&l.value, &mut value, src)?;
                let ty = l.ty.unwrap_or_else(|| {
                    unimplemented!("{} should have a type", l.name);
                });
                match ty {
                    HirTy::Int64(_) => {
                        value.push(Instruction::StoreI64 {
                            var_name: l.name.to_string(),
                        });
                    }
                    HirTy::Float64(_) => {
                        value.push(Instruction::StoreF64 {
                            var_name: l.name.to_string(),
                        });
                    }
                    HirTy::UInt64(_) => {
                        value.push(Instruction::StoreU64 {
                            var_name: l.name.to_string(),
                        });
                    }
                    HirTy::Boolean(_) => value.push(Instruction::StoreBool {
                        var_name: l.name.to_string(),
                    }),
                    _ => unimplemented!("Unsupported type for now"),
                }
                bytecode.append(&mut value);
            }
            HirStatement::Expr(e) => self.generate_bytecode_expr(&e.expr, bytecode, src)?,
            _ => {
                return Err(atlas_hir::error::HirError::UnsupportedStatement(
                    UnsupportedStatement {
                        span: SourceSpan::new(
                            SourceOffset::from(stmt.span().start()),
                            stmt.span().end() - stmt.span().start(),
                        ),
                        stmt: format!("{:?}", stmt),
                        src: src.clone(),
                    },
                ))
            }
        }
        Ok(())
    }

    fn generate_bytecode_expr(
        &mut self,
        expr: &HirExpr<'hir>,
        bytecode: &mut Vec<Instruction>,
        src: String,
    ) -> HirResult<()> {
        match expr {
            HirExpr::Assign(a) => {
                let lhs = a.lhs.as_ref();
                match lhs {
                    HirExpr::Ident(i) => {
                        self.generate_bytecode_expr(&a.rhs, bytecode, src)?;
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
                        return Err(atlas_hir::error::HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start()),
                                    expr.span().end() - expr.span().start(),
                                ),
                                expr: format!("{:?}", expr),
                                src: src.clone(),
                            },
                        ));
                    }
                }
            }
            HirExpr::HirBinaryOp(b) => {
                self.generate_bytecode_expr(&b.lhs, bytecode, src.clone())?;
                self.generate_bytecode_expr(&b.rhs, bytecode, src)?;
                match b.op {
                    atlas_hir::expr::HirBinaryOp::Add => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::AddI64);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::AddF64);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::AddU64);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Sub => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::SubI64);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::SubF64);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::SubU64);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Mul => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::MulI64);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::MulF64);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::MulU64);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Div => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::DivI64);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::DivF64);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::DivU64);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Mod => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::ModI64);
                        }
                        HirTy::Float64(_) => {
                            unimplemented!("Modulo not supported for float");
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::ModI64);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Eq => {
                        bytecode.push(Instruction::Eq);
                    }
                    atlas_hir::expr::HirBinaryOp::Neq => {
                        bytecode.push(Instruction::Neq);
                    }
                    atlas_hir::expr::HirBinaryOp::Gt => {
                        bytecode.push(Instruction::Gt);
                    }
                    atlas_hir::expr::HirBinaryOp::Gte => {
                        bytecode.push(Instruction::Gte);
                    }
                    atlas_hir::expr::HirBinaryOp::Lt => {
                        bytecode.push(Instruction::Lt);
                    }
                    atlas_hir::expr::HirBinaryOp::Lte => {
                        bytecode.push(Instruction::Lte);
                    }
                    _ => unimplemented!("Unsupported binary operator for now"),
                }
            }
            HirExpr::Unary(u) => {
                //The operators are not yet implemented
                self.generate_bytecode_expr(&u.expr, bytecode, src)?;
            }
            //This need to be thoroughly tested
            HirExpr::Call(f) => {
                for arg in &f.args {
                    self.generate_bytecode_expr(arg, bytecode, src.clone())?;
                }
                let callee = f.callee.as_ref();
                match callee {
                    HirExpr::Ident(i) => {
                        let func = self.hir.signature.functions.get(i.name).unwrap();
                        if func.is_external {
                            bytecode.push(Instruction::ExternCall {
                                name: i.name.to_string(),
                                args: f.args.len() as u8,
                            });
                        } else {
                            /*if let Some(pos) = self.global.get_index(i.name) {
                                bytecode.push(Instruction::DirectCall {
                                    pos,
                                    args: f.args.len() as u8,
                                })
                            } else {
                                self.global.insert(i.name);
                                bytecode.push(Instruction::DirectCall {
                                    pos: self.global.len() - 1,
                                    args: f.args.len() as u8,
                                })
                            }*/
                            bytecode.push(Instruction::CallFunction {
                                name: i.name.to_string(),
                                args: f.args.len() as u8,
                            });
                        }
                    }
                    _ => {
                        return Err(atlas_hir::error::HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start()),
                                    expr.span().end() - expr.span().start(),
                                ),
                                expr: format!("Can't call from: {:?}", expr),
                                src: src.clone(),
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
            HirExpr::BooleanLiteral(b) => bytecode.push(Instruction::PushBool(b.value)),
            HirExpr::UnsignedIntegerLiteral(u) => {
                bytecode.push(Instruction::PushUnsignedInt(u.value))
            }
            _ => {
                return Err(atlas_hir::error::HirError::UnsupportedExpr(
                    UnsupportedExpr {
                        span: SourceSpan::new(
                            SourceOffset::from(expr.span().start()),
                            expr.span().end() - expr.span().start(),
                        ),
                        expr: format!("{:?}", expr),
                        src: src.clone(),
                    },
                ))
            }
        }
        Ok(())
    }

    fn generate_bytecode_args(
        &self,
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
