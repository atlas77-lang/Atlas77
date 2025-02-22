/// Contains the definition of the CodeGenArena
pub mod arena;
mod table;

use crate::atlas_c::atlas_hir::{
    error::{HirResult, UnsupportedExpr, UnsupportedStatement},
    expr::HirExpr,
    signature::HirFunctionParameterSignature,
    stmt::{HirBlock, HirStatement},
    ty::HirTy,
    HirModule,
};
use crate::atlas_vm::runtime::instruction::{ConstantClass, ImportedLibrary, Instruction, Label, Program, Type};
use std::collections::BTreeMap;

use crate::atlas_c::atlas_codegen::table::_Table;
use crate::atlas_c::atlas_hir;
use crate::atlas_c::atlas_hir::error::HirError;
use crate::atlas_c::atlas_hir::expr::HirUnaryOp;
use crate::atlas_c::atlas_hir::item::{HirClass, HirClassConstructor};
use crate::atlas_c::atlas_hir::signature::{ConstantValue, HirClassMethodModifier};
use arena::CodeGenArena;
use miette::{SourceOffset, SourceSpan};

/// Result of codegen
pub type CodegenResult<T> = Result<T, HirError>;

/// Unit of codegen
pub struct CodeGenUnit<'hir, 'gen>
where
    'gen: 'hir,
{
    hir: HirModule<'hir>,
    program: Program<'gen>,
    arena: CodeGenArena<'gen>,
    //simulate a var_map so the codegen can translate it into stack operations
    _variables: _Table<&'hir str>,
    //store the function position
    _global: _Table<&'hir str>,
    current_pos: usize,
    string_pool: Vec<&'gen str>,
    class_pool: Vec<ConstantClass<'gen>>,
    //todo: Replace this with the path of the current module to be codegen
    src: String,
}

impl<'hir, 'gen> CodeGenUnit<'hir, 'gen> {
    /// Create a new CodeGenUnit
    pub fn new(hir: HirModule<'hir>, arena: CodeGenArena<'gen>, src: String) -> Self {
        Self {
            hir,
            program: Program::new(),
            arena,
            _variables: _Table::new(),
            _global: _Table::new(),
            current_pos: 0,
            string_pool: Vec::new(),
            class_pool: Vec::new(),
            src,
        }
    }

    /// Take the HIR and convert it to a VM representation
    pub fn compile(&mut self) -> CodegenResult<Program> {
        let mut labels: Vec<Label> = Vec::new();
        for (func_name, function) in self.hir.body.functions.clone() {
            let mut bytecode = Vec::new();

            let params = function.signature.params.clone();
            self.generate_bytecode_args(params, &mut bytecode)?;
            self.generate_bytecode_block(&function.body, &mut bytecode, self.src.clone())?;

            if func_name == "main" {
                bytecode.push(Instruction::Halt);
            }
            let len = bytecode.len();

            labels.push(Label {
                name: self.arena.alloc(func_name.to_string()),
                position: self.current_pos,
                body: self.arena.alloc(bytecode),
            });

            self.current_pos += len;
        }
        //Need to find something else than ".clone()"
        //Todo: make a function for the class codegen
        for (class_name, class) in self.hir.body.classes.clone() {
            self.generate_bytecode_class(class_name, &class, &mut labels, self.src.clone())?
        }
        self.program.entry_point = String::from("main");
        self.program.labels = labels;
        self.program.global.string_pool = self.arena.alloc(self.string_pool.clone());
        self.program.global.class_pool = self.arena.alloc(self.class_pool.clone());
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
        Ok(self.program.clone())
    }

    fn generate_bytecode_class(
        &mut self,
        class_name: &str,
        class: &HirClass<'hir>,
        labels: &mut Vec<Label<'gen>>,
        src: String,
    ) -> HirResult<()> {
        for method in class.methods.iter() {
            let mut bytecode = Vec::new();
            let params = method.signature.params.clone();
            self.generate_bytecode_args(params, &mut bytecode)?;
            if method.signature.modifier != HirClassMethodModifier::Static {
                bytecode.push(Instruction::Store {
                    var_name: self.arena.alloc(String::from("self")),
                });
            }
            self.generate_bytecode_block(&method.body, &mut bytecode, src.clone())?;
            let len = bytecode.len();
            labels.push(Label {
                name: self.arena.alloc(
                    if method.signature.modifier == HirClassMethodModifier::Static {
                        format!("{}::{}", class_name, method.name)
                    } else {
                        format!("{}.{}", class_name, method.name)
                    }
                ),
                position: self.current_pos,
                body: self.arena.alloc(bytecode),
            });
            self.current_pos += len;
        }
        let mut fields: Vec<&'gen str> = Vec::new();
        let mut constants: BTreeMap<&'gen str, ConstantValue> = BTreeMap::new();
        for field in class.fields.iter() {
            fields.push(self.arena.alloc(field.name.to_string()));
        }
        for (constant_name, constant) in class.signature.constants.iter() {
            constants.insert(self.arena.alloc(constant_name.to_string()), constant.value.clone());
        }
        let class_constant = ConstantClass {
            name: self.arena.alloc(class_name.to_string()),
            fields,
            constructor_nb_args: class.constructor.params.len(),
            constants,
        };
        self.class_pool.push(class_constant);


        self.generate_bytecode_constructor(class_name, &class.constructor, labels, src.clone())?;
        Ok(())
    }

    fn generate_bytecode_constructor(
        &mut self,
        class_name: &str,
        constructor: &HirClassConstructor<'hir>,
        labels: &mut Vec<Label<'gen>>,
        src: String,
    ) -> HirResult<()> {
        let mut bytecode = Vec::new();
        let params = constructor.params.clone();

        self.generate_bytecode_args(params, &mut bytecode)?;
        //self reference of the object
        bytecode.push(Instruction::Store {
            var_name: self.arena.alloc(String::from("self")),
        });

        self.generate_bytecode_block(&constructor.body, &mut bytecode, src.clone())?;

        //Return the self reference
        bytecode.push(Instruction::Load {
            var_name: self.arena.alloc(String::from("self")),
        });
        bytecode.push(Instruction::Return);

        let len = bytecode.len();
        labels.push(Label {
            name: self.arena.alloc(format!("{}.{}", class_name, class_name)),
            position: self.current_pos,
            body: self.arena.alloc(bytecode),
        });
        self.current_pos += len;

        Ok(())
    }

    fn generate_bytecode_block(
        &mut self,
        block: &HirBlock<'hir>,
        bytecode: &mut Vec<Instruction<'gen>>,
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
        bytecode: &mut Vec<Instruction<'gen>>,
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
            HirStatement::Const(let_stmt) => {
                println!("const {}: {}", let_stmt.name, let_stmt.ty.unwrap());
                let mut value = Vec::new();
                self.generate_bytecode_expr(&let_stmt.value, &mut value, src)?;
                value.push(Instruction::Store {
                    var_name: self.arena.alloc(let_stmt.name.to_string()),
                });
                bytecode.append(&mut value);
            }
            HirStatement::Let(let_stmt) => {
                println!("let {}: {}", let_stmt.name, let_stmt.ty.unwrap());
                let mut value = Vec::new();
                self.generate_bytecode_expr(&let_stmt.value, &mut value, src)?;
                value.push(Instruction::Store {
                    var_name: self.arena.alloc(let_stmt.name.to_string()),
                });
                bytecode.append(&mut value);
            }
            HirStatement::Expr(e) => {
                self.generate_bytecode_expr(&e.expr, bytecode, src)?;
                bytecode.push(Instruction::Pop);
            }
            _ => {
                return Err(atlas_hir::error::HirError::UnsupportedStatement(
                    UnsupportedStatement {
                        span: SourceSpan::new(
                            SourceOffset::from(stmt.span().start),
                            stmt.span().end - stmt.span().start,
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
        bytecode: &mut Vec<Instruction<'gen>>,
        src: String,
    ) -> HirResult<()> {
        match expr {
            HirExpr::Assign(a) => {
                let lhs = a.lhs.as_ref();
                match lhs {
                    HirExpr::Ident(i) => {
                        self.generate_bytecode_expr(&a.rhs, bytecode, src.clone())?;
                        bytecode.push(Instruction::Store {
                            var_name: self.arena.alloc(i.name.to_string()),
                        });
                    }
                    HirExpr::Indexing(i) => {
                        match i.target.ty() {
                            HirTy::List(_) => {
                                //Get the Index
                                self.generate_bytecode_expr(&i.index, bytecode, src.clone())?;
                                //Get the list pointer
                                self.generate_bytecode_expr(&i.target, bytecode, src.clone())?;
                                //Get the value
                                self.generate_bytecode_expr(&a.rhs, bytecode, src)?;
                                //Store the value in the list
                                bytecode.push(Instruction::ListStore);
                            }
                            HirTy::String(_) => {
                                eprintln!("String store: {:?}", a);
                                //Get the Index
                                self.generate_bytecode_expr(&i.index, bytecode, src.clone())?;
                                //Get the string pointer
                                self.generate_bytecode_expr(&i.target, bytecode, src.clone())?;
                                //Get the value
                                self.generate_bytecode_expr(&a.rhs, bytecode, src)?;
                                //Store the value in the string
                                bytecode.push(Instruction::StringStore);
                            }
                            _ => {
                                return Err(HirError::UnsupportedExpr(
                                    UnsupportedExpr {
                                        span: SourceSpan::new(
                                            SourceOffset::from(expr.span().start),
                                            expr.span().end - expr.span().start,
                                        ),
                                        expr: format!("{:?}", expr),
                                        src: src.clone(),
                                    },
                                ));
                            }
                        }
                    }
                    HirExpr::FieldAccess(field_access) => {
                        //Get the Class pointer
                        self.generate_bytecode_expr(&field_access.target, bytecode, src.clone())?;
                        //Get the value
                        self.generate_bytecode_expr(&a.rhs, bytecode, src)?;
                        //Store the value in the field
                        bytecode.push(Instruction::SetField {
                            field_name: self.arena.alloc(String::from(field_access.field.name))
                        })
                    }
                    _ => {
                        return Err(HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start),
                                    expr.span().end - expr.span().start,
                                ),
                                expr: format!("{:?}", expr),
                                src: src.clone(),
                            },
                        ));
                    }
                }
                bytecode.push(Instruction::PushUnit);
            }
            HirExpr::HirBinaryOp(b) => {
                self.generate_bytecode_expr(&b.lhs, bytecode, src.clone())?;
                self.generate_bytecode_expr(&b.rhs, bytecode, src)?;
                match b.op {
                    atlas_hir::expr::HirBinaryOp::Add => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::IAdd);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::FAdd);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::UIAdd);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Sub => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::ISub);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::FSub);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::UISub);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Mul => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::IMul);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::FMul);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::UIMul);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Div => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::IDiv);
                        }
                        HirTy::Float64(_) => {
                            bytecode.push(Instruction::FDiv);
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::UIDiv);
                        }
                        HirTy::Char(_) => {
                            bytecode.push(Instruction::IDiv);
                        }
                        _ => unimplemented!("Unsupported type for now"),
                    },
                    atlas_hir::expr::HirBinaryOp::Mod => match b.ty {
                        HirTy::Int64(_) => {
                            bytecode.push(Instruction::IMod);
                        }
                        HirTy::Float64(_) => {
                            //Should be a proper error
                            unimplemented!("Modulo not supported for float");
                        }
                        HirTy::UInt64(_) => {
                            bytecode.push(Instruction::IMod);
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
                //There is no unary instruction, so -x is the same as 0 - x
                //And !x is the same as x == 0
                self.generate_bytecode_expr(&u.expr, bytecode, src.clone())?;
                if let Some(op) = &u.op {
                    match op {
                        HirUnaryOp::Neg => {
                            match u.expr.ty() {
                                HirTy::Int64(_) => {
                                    bytecode.push(Instruction::PushInt(0));
                                    bytecode.push(Instruction::Swap);
                                    bytecode.push(Instruction::ISub);
                                }
                                HirTy::Float64(_) => {
                                    bytecode.push(Instruction::PushFloat(0.0));
                                    bytecode.push(Instruction::Swap);
                                    bytecode.push(Instruction::FSub);
                                }
                                // This won't really work, because you're subtracting a 32-bit char from a 64-bit integer
                                HirTy::Char(_) => {
                                    bytecode.push(Instruction::PushInt(0));
                                    bytecode.push(Instruction::Swap);
                                    bytecode.push(Instruction::ISub);
                                }
                                _ => {
                                    return Err(atlas_hir::error::HirError::UnsupportedExpr(
                                        UnsupportedExpr {
                                            span: SourceSpan::new(
                                                SourceOffset::from(expr.span().start),
                                                expr.span().end - expr.span().start,
                                            ),
                                            expr: format!("Can't negate: {:?}", expr),
                                            src,
                                        },
                                    ))
                                }
                            }
                        }
                        HirUnaryOp::Not => {
                            if let HirTy::Boolean(_) = u.expr.ty() {
                                bytecode.push(Instruction::PushBool(false));
                                bytecode.push(Instruction::Eq);
                            } else {
                                return Err(atlas_hir::error::HirError::UnsupportedExpr(
                                    UnsupportedExpr {
                                        span: SourceSpan::new(
                                            SourceOffset::from(expr.span().start),
                                            expr.span().end - expr.span().start,
                                        ),
                                        expr: format!("Can't negate: {:?}", expr),
                                        src,
                                    },
                                ));
                            }
                        }
                    }
                }
            }
            HirExpr::Casting(c) => {
                self.generate_bytecode_expr(&c.expr, bytecode, src.clone())?;
                match c.ty {
                    HirTy::Int64(_) => {
                        bytecode.push(Instruction::CastTo(Type::Integer));
                    }
                    HirTy::Float64(_) => {
                        bytecode.push(Instruction::CastTo(Type::Float));
                    }
                    HirTy::UInt64(_) => {
                        bytecode.push(Instruction::CastTo(Type::UnsignedInteger));
                    }
                    HirTy::Boolean(_) => {
                        bytecode.push(Instruction::CastTo(Type::Boolean));
                    }
                    HirTy::String(_) => {
                        bytecode.push(Instruction::CastTo(Type::String));
                    }
                    HirTy::Char(_) => {
                        bytecode.push(Instruction::CastTo(Type::Char));
                    }
                    _ => {
                        return Err(atlas_hir::error::HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start),
                                    expr.span().end - expr.span().start,
                                ),
                                expr: format!("Can't cast: {:?}", expr),
                                src,
                            },
                        ))
                    }
                }
            }
            HirExpr::Indexing(i) => {
                match i.target.ty() {
                    HirTy::List(_) => {
                        self.generate_bytecode_expr(&i.target, bytecode, src.clone())?;
                        self.generate_bytecode_expr(&i.index, bytecode, src)?;
                        bytecode.push(Instruction::ListLoad);
                    }
                    HirTy::String(_) => {
                        self.generate_bytecode_expr(&i.target, bytecode, src.clone())?;
                        self.generate_bytecode_expr(&i.index, bytecode, src)?;
                        bytecode.push(Instruction::StringLoad);
                    }
                    _ => {
                        return Err(HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start),
                                    expr.span().end - expr.span().start,
                                ),
                                expr: format!("Can't index: {:?}", expr),
                                src: src.clone(),
                            },
                        ))
                    }
                }
            }
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
                                function_name: self.arena.alloc(i.name.to_string()),
                                nb_args: f.args.len() as u8,
                            });
                        } else {
                            bytecode.push(Instruction::FunctionCall {
                                function_name: self.arena.alloc(i.name.to_string()),
                                nb_args: f.args.len() as u8,
                            });
                        }
                    }
                    HirExpr::FieldAccess(field_access) => {
                        //Get the Class pointer:
                        self.generate_bytecode_expr(&field_access.target, bytecode, src.clone())?;
                        //Get the arguments
                        for arg in f.args.iter() {
                            self.generate_bytecode_expr(arg, bytecode, src.clone())?;
                        }

                        let class_name = match field_access.target.ty() {
                            HirTy::Named(class_name) => class_name,
                            _ => {
                                return Err(HirError::UnsupportedExpr(
                                    UnsupportedExpr {
                                        span: SourceSpan::new(
                                            SourceOffset::from(expr.span().start),
                                            expr.span().end - expr.span().start,
                                        ),
                                        expr: format!("Can't call from: {:?}", expr),
                                        src: src.clone(),
                                    },
                                ))
                            }
                        };
                        bytecode.push(Instruction::MethodCall {
                            method_name: self.arena.alloc(format!("{}.{}", class_name.name, field_access.field.name)),
                            nb_args: f.args.len() as u8 + 1,
                        })
                    }
                    HirExpr::StaticAccess(static_access) => {
                        for arg in f.args.iter() {
                            self.generate_bytecode_expr(arg, bytecode, src.clone())?;
                        }
                        bytecode.push(Instruction::StaticCall {
                            method_name: self.arena.alloc(format!("{}::{}", static_access.target.name, static_access.field.name)),
                            nb_args: f.args.len() as u8,
                        })
                    }
                    _ => {
                        return Err(HirError::UnsupportedExpr(
                            UnsupportedExpr {
                                span: SourceSpan::new(
                                    SourceOffset::from(expr.span().start),
                                    expr.span().end - expr.span().start,
                                ),
                                expr: format!("Can't call from: {:?}", expr),
                                src: src.clone(),
                            },
                        ));
                    }
                }
            }
            HirExpr::IntegerLiteral(i) => bytecode.push(Instruction::PushInt(i.value)),
            HirExpr::FloatLiteral(f) => bytecode.push(Instruction::PushFloat(f.value)),
            HirExpr::BooleanLiteral(b) => bytecode.push(Instruction::PushBool(b.value)),
            HirExpr::CharLiteral(c) => bytecode.push(Instruction::PushChar(c.value)),
            HirExpr::UnitLiteral(_) => bytecode.push(Instruction::PushUnit),
            HirExpr::UnsignedIntegerLiteral(u) => {
                bytecode.push(Instruction::PushUnsignedInt(u.value))
            }
            HirExpr::StringLiteral(s) => {
                self.string_pool.push(self.arena.alloc(s.value.to_string()));
                let index = self.string_pool.len() - 1;
                bytecode.push(Instruction::PushStr(index));
            }
            HirExpr::ListLiteral(l) => {
                bytecode.push(Instruction::PushUnsignedInt(l.items.len() as u64));
                bytecode.push(Instruction::NewList);
                l.items.iter().enumerate().for_each(|(u, i)| {
                    //Duplicate the list reference
                    bytecode.push(Instruction::Dup);
                    //Push the index
                    bytecode.push(Instruction::PushUnsignedInt(u as u64));
                    //Swap the index and the list reference
                    bytecode.push(Instruction::Swap);
                    //Generate the expression
                    self.generate_bytecode_expr(i, bytecode, src.clone()).unwrap();
                    //Store the value in the list
                    bytecode.push(Instruction::ListStore);
                });
            }
            HirExpr::Delete(d) => {
                self.generate_bytecode_expr(&d.expr, bytecode, src)?;
                bytecode.push(Instruction::DeleteObj);
            }
            HirExpr::Ident(i) => bytecode.push(Instruction::Load { var_name: self.arena.alloc(i.name.to_string()) }),
            HirExpr::SelfLiteral(_) => bytecode.push(Instruction::Load { var_name: self.arena.alloc(String::from("self")) }),
            HirExpr::FieldAccess(field_access) => {
                self.generate_bytecode_expr(field_access.target.as_ref(), bytecode, src.clone())?;
                bytecode.push(Instruction::GetField {
                    field_name: self.arena.alloc(String::from(field_access.field.name))
                })
            }
            HirExpr::StaticAccess(static_access) => {
                match static_access.field.ty {
                    HirTy::String(_) => {
                        let target_name = static_access.target.name;
                        let class_signature = self.hir.signature.classes.get(target_name).unwrap();
                        let value = match class_signature.constants.get(static_access.field.name).unwrap().value {
                            ConstantValue::String(s) => String::from(s),
                            _ => {
                                return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                    span: SourceSpan::new(
                                        SourceOffset::from(expr.span().start),
                                        expr.span().end - expr.span().start,
                                    ),
                                    expr: format!("No string constant for {}", static_access.field.name),
                                    src: src.clone(),
                                }))
                            }
                        };
                        self.string_pool.push(self.arena.alloc(value));
                        let index = self.string_pool.len() - 1;
                        bytecode.push(Instruction::PushStr(index));
                    }
                    HirTy::Float64(_) => {
                        let class_signature = self.hir.signature.classes.get(static_access.target.name).unwrap();
                        let value = match class_signature.constants.get(static_access.field.name).unwrap().value {
                            ConstantValue::Float(f) => *f,
                            _ => {
                                return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                    span: SourceSpan::new(
                                        SourceOffset::from(expr.span().start),
                                        expr.span().end - expr.span().start,
                                    ),
                                    expr: format!("No float constant for {}", static_access.field.name),
                                    src: src.clone(),
                                }))
                            }
                        };
                        bytecode.push(Instruction::PushFloat(value));
                    }
                    HirTy::Int64(_) => {
                        let class_signature = self.hir.signature.classes.get(static_access.target.name).unwrap();
                        let value = match class_signature.constants.get(static_access.field.name).unwrap().value {
                            ConstantValue::Int(i) => *i,
                            _ => {
                                return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                    span: SourceSpan::new(
                                        SourceOffset::from(expr.span().start),
                                        expr.span().end - expr.span().start,
                                    ),
                                    expr: format!("No int constant for {}", static_access.field.name),
                                    src: src.clone(),
                                }))
                            }
                        };
                        bytecode.push(Instruction::PushInt(value));
                    }
                    HirTy::Char(_) => {
                        let class_signature = self.hir.signature.classes.get(static_access.target.name).unwrap();
                        let value = match class_signature.constants.get(static_access.field.name).unwrap().value {
                            ConstantValue::Char(c) => *c,
                            _ => {
                                return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                    span: SourceSpan::new(
                                        SourceOffset::from(expr.span().start),
                                        expr.span().end - expr.span().start,
                                    ),
                                    expr: format!("No char constant for {}", static_access.field.name),
                                    src: src.clone(),
                                }))
                            }
                        };
                        bytecode.push(Instruction::PushChar(value));
                    }
                    HirTy::UInt64(_) => {
                        let class_signature = self.hir.signature.classes.get(static_access.target.name).unwrap();
                        let value = match class_signature.constants.get(static_access.field.name).unwrap().value {
                            ConstantValue::UInt(u) => *u,
                            _ => {
                                return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                                    span: SourceSpan::new(
                                        SourceOffset::from(expr.span().start),
                                        expr.span().end - expr.span().start,
                                    ),
                                    expr: format!("No uint constant for {}", static_access.field.name),
                                    src: src.clone(),
                                }))
                            }
                        };
                        bytecode.push(Instruction::PushUnsignedInt(value));
                    }
                    HirTy::List(_) => {
                        return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                            span: SourceSpan::new(
                                SourceOffset::from(expr.span().start),
                                expr.span().end - expr.span().start,
                            ),
                            expr: format!("Lists aren't supported as constants for now {}", static_access.field.name),
                            src: src.clone(),
                        }))
                    }
                    _ => {
                        return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                            span: SourceSpan::new(
                                SourceOffset::from(expr.span().start),
                                expr.span().end - expr.span().start,
                            ),
                            expr: format!("Unsupported type for now {}", static_access.field.name),
                            src: src.clone(),
                        }))
                    }
                }
            }
            HirExpr::NewArray(a) => {
                self.generate_bytecode_expr(&a.size, bytecode, src.clone())?;
                bytecode.push(Instruction::NewList);
            }
            HirExpr::NewObj(new_obj) => {
                let class_name = match new_obj.ty {
                    HirTy::Named(class_name) => class_name,
                    _ => return Err(HirError::UnsupportedExpr(UnsupportedExpr {
                        span: SourceSpan::new(
                            SourceOffset::from(new_obj.span.start),
                            new_obj.span.end - new_obj.span.start,
                        ),
                        expr: format!("No constructor for {}", new_obj.ty),
                        src: src.clone(),
                    }))
                };
                //Need to create a NewObj & call its constructor (constructor name = ClassName.ClassName)
                bytecode.push(Instruction::NewObj {
                    class_name: self.arena.alloc(class_name.name.to_string()),
                });
                for arg in new_obj.args.iter() {
                    self.generate_bytecode_expr(arg, bytecode, src.clone())?;
                }
                bytecode.push(Instruction::MethodCall {
                    method_name: self.arena.alloc(format!("{}.{}", class_name.name, class_name.name)),
                    nb_args: new_obj.args.len() as u8 + 1,
                });
            }
        }
        Ok(())
    }

    fn generate_bytecode_args(
        &self,
        args: Vec<&HirFunctionParameterSignature<'hir>>,
        bytecode: &mut Vec<Instruction<'gen>>,
    ) -> HirResult<()> {
        let args = args.iter().rev().cloned().collect::<Vec<_>>();
        for arg in args {
            bytecode.push(Instruction::Store {
                var_name: self.arena.alloc(arg.name.to_string()),
            });
        }
        Ok(())
    }
}
