// The first view versions of the codegen will be dumb.
// It will just takes the HIR and convert it to a VM representation.
// No optimizations will be done.

use crate::{
    atlas_frontend::parser::{arena::AstArena, ast::{AstBinaryOp, AstBlock, AstExpr, AstFunction, AstItem, AstLiteral, AstProgram, AstUnaryOp}},
    atlas_vm::instruction::{Instruction, Label, Program},
};

pub type CodegenResult<T> = Result<T, String>;

pub struct Codegen<'gen> {
    pub ast: AstProgram<'gen>,
    pub program: Program<'gen>,
    pub current_pos: usize,
    pub arena: AstArena<'gen>,
}
impl<'gen> Codegen<'gen> {
    pub fn new(ast: AstProgram<'gen>, arena: AstArena<'gen>) -> Self {
        Self {
            ast,
            program: Program::new(),
            current_pos: 0,
            arena,
        }
    }

    pub fn compile(&mut self, ast: AstProgram<'gen>) -> CodegenResult<Program> {
        let mut labels = Vec::new();
        for item in ast.items {
            match item {
                AstItem::Func(f) => {
                    let mut bytecode = Vec::new();
                    Self::generate_bytecode_block(f.body, &mut bytecode);
                    labels.push(Label {
                        name: f.name.name,
                        position: self.current_pos,
                        body: self.arena.alloc(bytecode),
                    })
                }
                _ => {}
            }
        }
        Ok(Program {
            labels: self.arena.alloc(labels),
            entry_point: "main",
        })
    }

    fn generate_bytecode_block(block: &AstBlock<'gen>, bytecode: &mut Vec<Instruction>) {
        for expr in block.exprs {
            Self::generate_bytecode_expr(expr, bytecode);
        }
    }

    fn generate_bytecode_expr(expr: &AstExpr<'gen>, bytecode: &mut Vec<Instruction>) {
        match expr {
            AstExpr::BinaryOp(bin_op) => {
                Self::generate_bytecode_expr(bin_op.lhs, bytecode);
                Self::generate_bytecode_expr(bin_op.rhs, bytecode);
                match bin_op.op {
                    AstBinaryOp::Add => bytecode.push(Instruction::AddI64),
                    AstBinaryOp::Sub => bytecode.push(Instruction::SubI64),
                    AstBinaryOp::Mul => bytecode.push(Instruction::MulI64),
                    AstBinaryOp::Div => bytecode.push(Instruction::DivI64),
                    AstBinaryOp::Mod => bytecode.push(Instruction::ModI64),
                    _ => todo!("Implement the rest of the binary operators"),
                }
            }
            AstExpr::UnaryOp(u) => {
                Self::generate_bytecode_expr(u.expr, bytecode);
                match u.op {
                    Some(op) => {
                        todo!("Implement the rest of the unary operators");
                    }
                    _ => {}
                }
            }
            AstExpr::Identifier(i) => {
                bytecode.push(Instruction::LoadI64{ var_name: i.name.to_string() });
            }
            AstExpr::Literal(l) => {
                match l {
                    AstLiteral::Float(f) => bytecode.push(Instruction::PushFloat(f.value)),
                    AstLiteral::Integer(i) => bytecode.push(Instruction::PushInt(i.value)),
                    AstLiteral::UnsignedIntegerer(u) => bytecode.push(Instruction::PushUnsignedInt(u.value)),
                    AstLiteral::String(s) => bytecode.push(Instruction::PushString(s.value.to_string())),
                    _ => todo!("Implement the rest of the literals"),
                }
            }
            AstExpr::Return(e) => {
                Self::generate_bytecode_expr(e.value, bytecode);
                bytecode.push(Instruction::Return);
            }
            AstExpr::Block(b) => {
                Self::generate_bytecode_block(b, bytecode);
            }
            _ => {
                todo!("Implement the rest of the expressions");
            }
        }
    }    
}
