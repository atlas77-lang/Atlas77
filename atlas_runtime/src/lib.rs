pub mod value;
pub mod visitor;
pub mod vm_state;
pub mod stdlib;
use std::collections::HashMap;

use crate::visitor::{Program, Visitor};
use atlas_frontend::parser::ast::*;
use atlas_memory::object_map::{Memory, Object, Structure};
use atlas_memory::stack::Stack;
use atlas_memory::vm_data::VMData;
use internment::Intern;

#[derive(Debug, Clone)]
pub struct VarMap {
    pub map: HashMap<Intern<String>, VMData>,
}
impl VarMap {
    pub fn new() -> Self {
        VarMap {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: Intern<String>, value: VMData) {
        self.map.insert(name, value);
    }
    pub fn get(&self, name: &Intern<String>) -> Option<&VMData> {
        let value = self.map.get(name);
        value
    }
}
pub type CallBack = fn(vm_state::VMState) -> Result<VMData, ()>;

pub struct Runtime {
    pub varmap: Vec<VarMap>, //usize is the parent scope
    pub stack: Stack,
    pub func_map: Vec<(String, FunctionExpression)>,
    pub extern_fn: Vec<(String, CallBack)>,
    pub consts: HashMap<Intern<String>, VMData>,
    pub object_map: Memory,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            varmap: vec![VarMap::new()],
            stack: Stack::new(),
            func_map: Vec::new(),
            extern_fn: Vec::new(),
            consts: {
                let mut h = HashMap::new();
                h.insert(
                    Intern::new("pi".to_string()),
                    VMData::new_f64(std::f64::consts::PI),
                );
                h.insert(
                    Intern::new("e".to_string()),
                    VMData::new_f64(std::f64::consts::E),
                );
                h
            },
            object_map: Memory::new(4096),
        }
    }
    pub fn add_extern_fn(&mut self, name: &str, f: CallBack) {
        self.extern_fn.push((name.to_string(), f));
    }

    fn find_variable(&self, name: Intern<String>) -> Option<&VMData> {
        if let Some(v) = self.consts.get(&Intern::new(name.to_string())) {
            return Some(v);
        }
        if let Some(v) = self.varmap.last().unwrap().get(&name) {
            Some(v)
        } else {
            if let Some(v) = self.varmap[0].get(&name) {
                Some(v)
            } else {
                None
            }
        }
    }
}

impl Visitor for Runtime {
    fn visit(&mut self, program: &Program) -> VMData {
        for expr in program {
            if let Expression::VariableDeclaration(v) = expr.as_ref() {
                self.visit_variable_declaration(v);
            }
        }

        if let Some(func) = self.func_map.iter().find(|(k, _)| k == "main") {
            return self.visit_function_expression(&func.1.clone());
        }
        VMData::new_unit()
    }
    fn visit_expression(&mut self, expression: &Expression) -> VMData {
        match expression {
            Expression::BinaryExpression(e) => self.visit_binary_expression(e),
            Expression::DoExpression(e) => self.visit_do_expression(e),
            Expression::FunctionCall(e) => self.visit_function_call(e),
            Expression::FunctionExpression(e) => self.visit_function_expression(e),
            Expression::Identifier(e) => self.visit_identifier(e),
            Expression::IfElseNode(e) => self.visit_if_else_node(e),
            Expression::IndexExpression(e) => self.visit_index_expression(e),
            Expression::MatchExpression(e) => self.visit_match_expression(e),
            Expression::UnaryExpression(e) => self.visit_unary_expression(e),
            Expression::VariableDeclaration(e) => self.visit_variable_declaration(e),
            Expression::NewObjectExpression(e) => self.visit_new_object_expression(e),
            Expression::FieldAccessExpression(e) => self.visit_field_access_expression(e),
            Expression::Literal(e) => match e {
                Literal::Bool(b) => VMData::new_bool(*b),
                Literal::Float(f) => VMData::new_f64(*f),
                Literal::Integer(i) => VMData::new_i64(*i),
                Literal::String(s) => {
                    let res = self.object_map.put(Object::String(s.to_string()));
                    match res {
                        Ok(i) => VMData::new_string(i),
                        Err(_) => {
                            panic!("Out of memory for a new string");
                        }
                    }
                }
                Literal::List(l) => {
                    let mut v = Vec::new();
                    for i in l {
                        v.push(self.visit_expression(i));
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
            },
            _ => unimplemented!("Expression not implemented"),
        }
    }
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> VMData {
        let lhs = self.visit_expression(&expression.left);
        let rhs = self.visit_expression(&expression.right);
        match expression.operator {
            BinaryOperator::OpAdd => match (lhs.tag, rhs.tag) {
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
            BinaryOperator::OpSub => lhs - rhs,
            BinaryOperator::OpMul => lhs * rhs,
            BinaryOperator::OpDiv => lhs / rhs,
            BinaryOperator::OpMod => lhs % rhs,
            BinaryOperator::OpEq => VMData::new_bool(lhs == rhs),
            BinaryOperator::OpNe => VMData::new_bool(lhs != rhs),
            BinaryOperator::OpLt => VMData::new_bool(lhs < rhs),
            BinaryOperator::OpLe => VMData::new_bool(lhs <= rhs),
            BinaryOperator::OpGt => VMData::new_bool(lhs > rhs),
            BinaryOperator::OpGe => VMData::new_bool(lhs >= rhs),
            BinaryOperator::OpAnd => VMData::new_bool(lhs.as_bool() && rhs.as_bool()),
            BinaryOperator::OpOr => VMData::new_bool(lhs.as_bool() || rhs.as_bool())
        }
    }
    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> VMData {
        let mut res = VMData::new_unit();
        for expr in &do_expression.body {
            res = self.visit_expression(expr);
        }
        res
    }
    fn visit_function_call(&mut self, function_call: &FunctionCall) -> VMData {
        let func = self
            .func_map
            .iter()
            .find(|(k, _)| k == function_call.name.as_str())
            .cloned();
        if let Some(f) = func {
            let mut args = Vec::new();
            for arg in &function_call.args {
                args.push(self.visit_expression(arg));
            }
            let mut new_varmap = VarMap::new();
            for (i, arg) in f.1.args.iter().enumerate() {
                new_varmap.insert(arg.0, args[i]);
            }
            self.varmap.push(new_varmap);
            let res = self.visit_expression(&f.1.body);
            self.varmap.pop();
            res
        } else {
            let func = self
                .extern_fn
                .iter()
                .find(|f| f.0 == function_call.name.as_str())
                .cloned();
            if let Some(f) = func {
                let mut args = Vec::new();
                for arg in &function_call.args {
                    args.push(self.visit_expression(arg));
                }
                args.iter().for_each(|arg| self.stack.push(*arg));
                let vm_state =
                    vm_state::VMState::new(&mut self.stack, &mut self.object_map, &self.consts);
                let res = f.1(vm_state).unwrap();
                res
            }
            else {
                if let Some(v) = self.find_variable(function_call.name) {
                    if let VMData::TAG_FN_PTR = v.tag {
                        let func = self.func_map[v.as_fn_ptr()].clone();
                        let mut new_varmap = VarMap::new();
                        for (i, arg) in func.1.args.iter().enumerate() {
                            new_varmap.insert(arg.0, self.visit_expression(&function_call.args[i]));
                        }
                        self.varmap.push(new_varmap);
                        let res = self.visit_expression(&func.1.body);
                        self.varmap.pop();
                        res
                    } else {
                        panic!("You can't call on type {:?}", v);
                    }
                } else {
                    panic!("Function {} not found", function_call.name);
                }
            }
        }
    }

    fn visit_field_access_expression(&mut self, field_access_expression: &FieldAccessExpression) -> VMData {
        let obj_ptr = self.find_variable(field_access_expression.name).unwrap();
        let obj = self.object_map.get(obj_ptr.as_object());
        match obj {
            Object::Structure(s) => s.fields[field_access_expression.field],
            _ => panic!("Not a structure"),
        }
    }

    fn visit_new_object_expression(&mut self, new_object_expression: &NewObjectExpression) -> VMData {
        let mut fields = Vec::new();
        for expr in &new_object_expression.fields {
            fields.push(self.visit_expression(expr));
        }
        let res = self.object_map.put(Object::new(Structure { fields }));
        match res {
            Ok(i) => VMData::new_object(300, i), //TODO: Fix this (TypeID is hardcoded)
            Err(_) => {
                panic!("Out of memory for a new object");
            }
        }
    }

    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> VMData {
        self.visit_expression(&function_expression.body)
    }

    fn visit_identifier(&mut self, identifier: &IdentifierNode) -> VMData {
        if let Some(v) = self.find_variable(identifier.name) {
            *v
        } else {
            panic!("Variable {} not found", identifier.name);
        }
    }
    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> VMData {
        if self.visit_expression(&if_else_node.condition).as_bool() {
            self.visit_expression(&if_else_node.if_body)
        } else if let Some(else_body) = &if_else_node.else_body {
            self.visit_expression(else_body)
        } else {
            VMData::new_unit()
        }
    }
    fn visit_index_expression(&mut self, index_expression: &IndexExpression) -> VMData {
        let list = *self.find_variable(index_expression.name).unwrap();
        let index = self.visit_expression(&index_expression.index);
        if list.tag > 256 {
            let list = self.object_map.get(list.as_object());
            match list {
                Object::List(l) => l[index.as_u64() as usize],
                _ => panic!("Not a list"),
            }
        } else {
            panic!("Not a list");
        }
    }
    fn visit_match_expression(&mut self, match_expression: &MatchExpression) -> VMData {
        let expr = self.visit_expression(&match_expression.expr);
        for arm in &match_expression.arms {
            if self.visit_expression(&arm.pattern) == expr {
                return self.visit_expression(&arm.body);
            }
        }
        if let Some(d) = &match_expression.default {
            return self.visit_expression(d);
        }
        panic!("No match found");
    }
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> VMData {
        let val = self.visit_expression(&expression.expression);
        if let Some(op) = &expression.operator {
            match op {
                UnaryOperator::OpNot => VMData::new_bool(!val.as_bool()),
                UnaryOperator::OpSub => match val.tag {
                    VMData::TAG_I64 => VMData::new_i64(-val.as_i64()),
                    VMData::TAG_FLOAT => VMData::new_f64(-val.as_f64()),
                    _ => panic!("Illegal operation"),
                }
            }
        } else {
            val
        }
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> VMData {
        let mut val = VMData::new_unit();
        if let Some(v) = variable_declaration.value.clone() {
            match v.as_ref() {
                Expression::FunctionExpression(f) => {
                    self.func_map
                        .push((variable_declaration.name.to_string(), f.clone()));
                }
                _ => {
                    val = self.visit_expression(&v);
                    self.varmap
                        .last_mut()
                        .unwrap()
                        .insert(variable_declaration.name, val);
                }
            }
        }
        val
    }
}
