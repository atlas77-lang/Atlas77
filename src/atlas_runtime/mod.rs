pub mod errors;
pub mod value;
pub mod visitor;
pub mod vm_state;

use std::collections::HashMap;

use crate::atlas_frontend::parser::ast::*;
use crate::atlas_frontend::parser::ast::{
    AstBinaryOp, AstBinaryOpExpr, AstBlock, AstBooleanLiteral, AstBooleanType, AstCallExpr,
    AstCompTimeExpr, AstDoExpr, AstEnum, AstEnumVariant, AstExpr, AstExternFunction,
    AstFieldAccessExpr, AstFieldInit, AstFloatLiteral, AstFloatType, AstFunction, AstFunctionType,
    AstIdentifier, AstIfElseExpr, AstImport, AstIndexingExpr, AstIntegerLiteral, AstIntegerType,
    AstItem, AstLambdaExpr, AstLetExpr, AstLiteral, AstMatchArm, AstNamedType, AstNewObjExpr,
    AstObjField, AstPattern, AstPatternKind, AstPointerType, AstProgram, AstStringLiteral,
    AstStringType, AstStruct, AstType, AstUnaryOp, AstUnaryOpExpr, AstUnion, AstUnionVariant,
    AstUnitType, AstUnsignedIntegerLiteral, AstUnsignedIntegerType,
};
use crate::atlas_memory::{
    object_map::{Memory, Object, Structure},
    stack::Stack,
    vm_data::VMData,
};
use crate::atlas_runtime::{errors::RuntimeError, vm_state::VMState};

use errors::RuntimeResult;
use internment::Intern;
use visitor::Visitor;

/// VarMap should be moved to its own file and have a better implementation overall. The concept of scopes should make an appareance
/// Also, to avoid alloc/dealloc/realloc overhead I should keep a free list of all the already existing varmap and just clean them before using them
#[derive(Debug, Clone, Default)]
pub struct VarMap<'run> {
    pub map: HashMap<&'run str, VMData>,
}

pub struct FuncMap<'run> {
    pub map: HashMap<&'run str, &'run AstFunction<'run>>,
}

impl<'run> FuncMap<'run> {
    pub fn new() -> Self {
        FuncMap {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: &'run str, value: &'run AstFunction<'run>) {
        self.map.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<&&'run AstFunction<'run>> {
        let value = self.map.get(name);
        value
    }
    pub fn contains_key(&self, name: &AstIdentifier) -> bool {
        self.map.contains_key(name.name)
    }
}

impl<'run> VarMap<'run> {
    pub fn new() -> Self {
        VarMap {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: &'run str, value: VMData) {
        self.map.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<&VMData> {
        let value = self.map.get(name);
        value
    }
    pub fn contains_key(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
}

pub struct Runtime<'run> {
    pub varmap: Vec<VarMap<'run>>,
    pub stack: Stack,
    pub func_map: FuncMap<'run>,
    pub extern_fn: Vec<(String, <Runtime<'run> as Visitor<'run>>::CallBack)>,
    pub consts: HashMap<&'run str, VMData>,
    pub object_map: Memory,
    pub main_fn: usize,
}

impl Default for Runtime<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'run> Runtime<'run> {
    pub fn add_extern_fn(&mut self, name: &str, f: <Runtime<'run> as Visitor<'run>>::CallBack) {
        self.extern_fn.push((name.to_string(), f));
    }
}

impl Runtime<'_> {
    pub fn new() -> Self {
        Runtime {
            varmap: vec![VarMap::new()],
            stack: Stack::new(),
            func_map: FuncMap::new(),
            extern_fn: Vec::new(),
            consts: {
                let mut h = HashMap::new();
                h.insert("pi", VMData::new_f64(std::f64::consts::PI));
                h.insert("e", VMData::new_f64(std::f64::consts::E));
                h
            },
            object_map: Memory::new(4096),
            main_fn: 0,
        }
    }

    fn find_variable(&self, name: &str) -> Option<&VMData> {
        if let Some(v) = self.consts.get(name) {
            return Some(v);
        }
        if let Some(v) = self.varmap.last().unwrap().get(&name) {
            Some(v)
        } else if let Some(v) = self.varmap[0].get(&name) {
            Some(v)
        } else {
            None
        }
    }
}

impl<'run> Visitor<'run> for Runtime<'run> {
    type CallBack = fn(VMState) -> Result<VMData, RuntimeError>;

    fn visit_block_expression(&mut self, block: &'run AstBlock) -> RuntimeResult<VMData> {
        let mut last_expr = VMData::new_unit();
        for expr in block.exprs {
            last_expr = self.visit_expression(expr)?;
        }
        Ok(last_expr)
    }
    fn visit(&mut self, program: &'run AstProgram, entry_point: &str) -> RuntimeResult<VMData> {
        for item in program.items {
            match item {
                AstItem::Func(f) => {
                    self.func_map.insert(f.name.name, f);
                }
                AstItem::Import(i) => {
                    //bad but it's temporary
                    match i.path {
                        "std/io" => {
                            self.add_extern_fn("print", crate::atlas_stdlib::io::print);
                            self.add_extern_fn("println", crate::atlas_stdlib::io::println);
                            self.add_extern_fn("input", crate::atlas_stdlib::io::input);
                        }
                        _ => unimplemented!("Import not implemented"),
                    }
                }
                _ => {}
            }
        }

        if let Some(f) = self.func_map.get(entry_point) {
            self.visit_function_expression(f)
        } else {
            Err(RuntimeError::EntryPointNotFound(entry_point.to_string()))
        }
    }
    fn visit_expression(&mut self, expression: &'run AstExpr) -> RuntimeResult<VMData> {
        match expression {
            AstExpr::BinaryOp(e) => self.visit_binary_expression(e),
            AstExpr::Block(e) => self.visit_block_expression(e),
            AstExpr::Call(e) => self.visit_function_call(e),
            AstExpr::Identifier(e) => self.visit_identifier(e),
            AstExpr::IfElse(e) => self.visit_if_else_node(e),
            AstExpr::Indexing(e) => self.visit_index_expression(e),
            AstExpr::Match(e) => self.visit_match_expression(e),
            AstExpr::UnaryOp(e) => self.visit_unary_expression(e),
            AstExpr::Let(e) => self.visit_variable_declaration(e),
            AstExpr::FieldAccess(e) => self.visit_field_access_expression(e),
            AstExpr::Literal(e) => self.visit_literal(e),
            _ => unimplemented!("AstExpr not implemented"),
        }
    }

    fn visit_literal(&mut self, literal: &'run AstLiteral) -> RuntimeResult<VMData> {
        let res = match literal {
            AstLiteral::Boolean(b) => VMData::new_bool(b.value),
            AstLiteral::Float(f) => VMData::new_f64(f.value),
            AstLiteral::Integer(i) => VMData::new_i64(i.value),
            AstLiteral::UnsignedIntegerer(u) => VMData::new_u64(u.value),
            AstLiteral::String(s) => {
                println!("String: {}", s.value);
                let res = self.object_map.put(Object::String(s.value.to_string()));
                match res {
                    Ok(i) => {
                        let ptr = VMData::new_string(i);
                        println!("String: {}", ptr);
                        ptr
                    },
                    Err(_) => {
                        panic!("Out of memory for a new string");
                    }
                }
            }
            AstLiteral::List(l) => {
                let mut v = Vec::new();
                for i in l.items {
                    v.push(self.visit_expression(i)?);
                }
                let res = self.object_map.put(Object::List(v));
                match res {
                    //TODO: Fix this (TypeID is hardcoded)
                    Ok(i) => VMData::new_list(367, i),
                    Err(_) => {
                        panic!("Out of memory for a new list");
                    }
                }
            }
        };
        Ok(res)
    }
    fn visit_binary_expression(
        &mut self,
        expression: &'run AstBinaryOpExpr,
    ) -> RuntimeResult<VMData> {
        let lhs = self.visit_expression(&expression.lhs)?;
        let rhs = self.visit_expression(&expression.rhs)?;
        let res = match expression.op {
            AstBinaryOp::Add => match (lhs.tag, rhs.tag) {
                (VMData::TAG_STR, VMData::TAG_STR) => {
                    let s1 = self.object_map.get(lhs.as_object());
                    let s2 = self.object_map.get(rhs.as_object());
                    let res = self.object_map.put(Object::String(format!("{}{}", s1, s2)));
                    match res {
                        Ok(i) => VMData::new_string(i),
                        Err(_) => {
                            panic!("Out of memory for a new string");
                        }
                    }
                }
                _ => lhs + rhs,
            },
            AstBinaryOp::Sub => lhs - rhs,
            AstBinaryOp::Mul => lhs * rhs,
            AstBinaryOp::Div => lhs / rhs,
            AstBinaryOp::Mod => lhs % rhs,
            AstBinaryOp::Eq => VMData::new_bool(lhs == rhs),
            AstBinaryOp::NEq => VMData::new_bool(lhs != rhs),
            AstBinaryOp::Lt => VMData::new_bool(lhs < rhs),
            AstBinaryOp::Lte => VMData::new_bool(lhs <= rhs),
            AstBinaryOp::Gt => VMData::new_bool(lhs > rhs),
            AstBinaryOp::Gte => VMData::new_bool(lhs >= rhs),
            AstBinaryOp::And => VMData::new_bool(lhs.as_bool() && rhs.as_bool()),
            AstBinaryOp::Or => VMData::new_bool(lhs.as_bool() || rhs.as_bool()),
            _ => return Err(RuntimeError::InvalidOperation),
        };

        Ok(res)
    }

    fn visit_function_call(&mut self, function_call: &'run AstCallExpr) -> RuntimeResult<VMData> {
        match function_call.callee {
            AstExpr::Identifier(i) => {
                for arg in function_call.args{
                    let arg = self.visit_expression(arg)?;
                    self.stack.push(arg)?;
                }
                if self.func_map.contains_key(i) {
                    let f = self.func_map.get(i.name).unwrap();
                    self.visit_function_expression(f)
                } else {
                    for (name, f) in &self.extern_fn {
                        if name == &i.name {
                            return f(VMState {
                                stack: &mut self.stack,
                                object_map: &mut self.object_map,
                                consts: &self.consts,
                                //Only give the current varmap
                                varmap: &self.varmap.last().unwrap(),
                                funcmap: &self.func_map,
                            });
                        }
                    }
                    panic!("Function {} not found", i.name);
                }
            }
            _ => self.visit_expression(function_call.callee),
        }
    }

    fn visit_field_access_expression(
        &mut self,
        _field_access_expression: &'run AstFieldAccessExpr,
    ) -> RuntimeResult<VMData> {
        unimplemented!("Field access not implemented")
    }

    fn visit_function_expression(&mut self, function: &'run AstFunction) -> RuntimeResult<VMData> {
        self.visit_block_expression(function.body)
    }

    fn visit_identifier(&mut self, identifier: &'run AstIdentifier) -> RuntimeResult<VMData> {
        if let Some(v) = self.find_variable(identifier.name) {
            Ok(*v)
        } else {
            return Err(RuntimeError::NullReference);
        }
    }
    fn visit_if_else_node(&mut self, if_else_node: &'run AstIfElseExpr) -> RuntimeResult<VMData> {
        if self.visit_expression(&if_else_node.condition)?.as_bool() {
            self.visit_block_expression(&if_else_node.body)
        } else if let Some(else_body) = &if_else_node.else_body {
            self.visit_block_expression(else_body)
        } else {
            Ok(VMData::new_unit())
        }
    }
    fn visit_index_expression(
        &mut self,
        index_expression: &'run AstIndexingExpr,
    ) -> RuntimeResult<VMData> {
        let list = self.visit_expression(index_expression.target)?;
        let index = self.visit_expression(&index_expression.index)?;
        if list.tag > 256 {
            let list = self.object_map.get(list.as_object());
            match list {
                Object::List(l) => Ok(l[index.as_u64() as usize]),
                _ => panic!("Not a list"),
            }
        } else {
            panic!("Not a list");
        }
    }
    fn visit_match_expression(
        &mut self,
        match_expression: &'run AstMatchExpr,
    ) -> RuntimeResult<VMData> {
        todo!("Match expression not implemented")
    }
    fn visit_unary_expression(
        &mut self,
        expression: &'run AstUnaryOpExpr,
    ) -> RuntimeResult<VMData> {
        let val = self.visit_expression(&expression.expr)?;
        if let Some(op) = &expression.op {
            let res = match op {
                AstUnaryOp::Not => VMData::new_bool(!val.as_bool()),
                AstUnaryOp::Neg => match val.tag {
                    VMData::TAG_I64 => VMData::new_i64(-val.as_i64()),
                    VMData::TAG_FLOAT => VMData::new_f64(-val.as_f64()),
                    _ => panic!("Illegal operation"),
                },
                AstUnaryOp::AsRef => todo!("AsRef not implemented"),
                AstUnaryOp::Deref => todo!("Deref not implemented"),
            };
            Ok(res)
        } else {
            Ok(val)
        }
    }
    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &'run AstLetExpr,
    ) -> RuntimeResult<VMData> {
        todo!("Variable declaration not implemented")
    }
}
