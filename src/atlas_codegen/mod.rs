/// Contains the definition of the CodeGenArena
pub mod arena;

use crate::{
    atlas_hir::{
        expr::HirExpr,
        signature::HirFunctionParameterSignature,
        stmt::{HirBlock, HirStatement},
        ty::HirTy,
        HirModule,
    },
    atlas_vm::instruction::{Instruction, Label, Program},
};

use arena::CodeGenArena;

/// Result of codegen
pub type CodegenResult<T> = Result<T, String>;

/// Unit of codegen
pub struct CodeGenUnit<'hir, 'gen>
where
    'gen: 'hir,
{
    hir: HirModule<'hir>,
    program: Program<'gen>,
    arena: CodeGenArena<'gen>,
    current_pos: usize,
}

impl<'hir, 'gen> CodeGenUnit<'hir, 'gen> {
    /// Create a new CodeGenUnit
    pub fn new(hir: HirModule<'hir>, arena: CodeGenArena<'gen>) -> Self {
        Self {
            hir,
            program: Program::new(),
            current_pos: 0,
            arena,
        }
    }
    /// Take the HIR and convert it to a VM representation
    pub fn compile(&mut self) -> CodegenResult<Program> {
        let mut labels: Vec<Label> = Vec::new();
        for func in self.hir.body.functions.clone() {
            let mut bytecode = Vec::new();

            let params = func.1.signature.params.clone();
            self.generate_bytecode_args(params, &mut bytecode);
            self.generate_bytecode_block(&func.1.body, &mut bytecode);

            let len = bytecode.len();

            labels.push(Label {
                name: func.0.to_string(),
                position: self.current_pos,
                body: self.arena.alloc_vec(bytecode),
            });

            self.current_pos += len;
        }
        self.program.entry_point = "main";
        self.program.labels = self.arena.alloc_vec(labels);
        Ok(self.program)
    }

    fn generate_bytecode_block(&mut self, block: &HirBlock<'hir>, bytecode: &mut Vec<Instruction>) {
        for stmt in &block.statements {
            self.generate_bytecode_stmt(stmt, bytecode);
        }
    }

    fn generate_bytecode_stmt(
        &mut self,
        stmt: &HirStatement<'hir>,
        bytecode: &mut Vec<Instruction>,
    ) {
        match stmt {
            HirStatement::Return(e) => {
                self.generate_bytecode_expr(e.value, bytecode);
                bytecode.push(Instruction::Return);
            }
            HirStatement::IfElse(i) => {
                self.generate_bytecode_expr(i.condition, bytecode);
                let mut then_body = Vec::new();
                self.generate_bytecode_block(&i.then_branch, &mut then_body);

                bytecode.push(Instruction::JmpZ {
                    pos: then_body.len() + 1,
                });
                bytecode.append(&mut then_body);
                if let Some(e) = i.else_branch {
                    let mut else_body = Vec::new();
                    self.generate_bytecode_block(e, &mut else_body);

                    bytecode.push(Instruction::Jmp {
                        pos: else_body.len() + 1,
                    });
                    bytecode.append(&mut else_body);
                }
            }
            HirStatement::Expr(e) => self.generate_bytecode_expr(e.expr, bytecode),
            _ => unimplemented!("Unsupported statement for now"),
        }
    }

    fn generate_bytecode_expr(&mut self, expr: &HirExpr<'hir>, bytecode: &mut Vec<Instruction>) {
        match expr {
            HirExpr::HirBinaryOp(b) => {
                self.generate_bytecode_expr(&b.lhs, bytecode);
                self.generate_bytecode_expr(&b.rhs, bytecode);
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
                self.generate_bytecode_expr(&u.expr, bytecode);
            }
            //This need to be thoroughly tested
            HirExpr::Call(f) => {
                for arg in &f.args {
                    self.generate_bytecode_expr(arg, bytecode);
                }
                let callee = f.callee.as_ref();
                match callee {
                    HirExpr::Ident(i) => {
                        bytecode.push(Instruction::CallFunction {
                            name: i.name.to_string(),
                            args: f.args.len() as u8,
                        });
                    }
                    _ => unimplemented!("Unsupported callee for now"),
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
            _ => unimplemented!("Unsupported expression for now"),
        }
    }

    fn generate_bytecode_args(
        &mut self,
        args: Vec<&HirFunctionParameterSignature<'hir>>,
        bytecode: &mut Vec<Instruction>,
    ) {
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
    }
}
