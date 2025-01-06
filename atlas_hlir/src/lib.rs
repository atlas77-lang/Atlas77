use std::collections::HashMap;

use atlas_frontend::parser::ast::{
    AbstractSyntaxTree, BinaryExpression, DoExpression, Expression, FieldAccessExpression,
    FunctionCall, FunctionExpression, IdentifierNode, IfElseNode, IndexExpression, Literal,
    MatchArm, MatchExpression, NewObjectExpression, StructDeclaration, Type, UnaryExpression,
    VariableDeclaration,
};
use errors::HlirError;
use internment::Intern;
use node::{
    HlirBinaryExp, HlirDoExpr, HlirExpr, HlirFieldAccess, HlirFunCall, HlirFunExpr, HlirIdentifier,
    HlirIfElseExpr, HlirIndexExpr, HlirLiteral, HlirMatchArm, HlirMatchExpr, HlirNewObjExpr,
    HlirStructDecl, HlirTree, HlirUnary, HlirVarDecl, IdentID, TypeID,
};

pub mod node;
//pub mod ir_builder;
pub mod data_type;
//pub mod context;
pub mod errors;

static mut VARIABLE_ID: u64 = 0;
static mut TYPE_ID: u64 = 256; // 0-255 are reserved for primitive types
fn get_variable_id() -> u64 {
    unsafe {
        VARIABLE_ID += 1;
        VARIABLE_ID
    }
}
fn get_type_id() -> u64 {
    unsafe {
        TYPE_ID += 1;
        TYPE_ID
    }
}

pub fn translate(ast: &AbstractSyntaxTree) -> HlirTree {
    let mut builder = HlirBuilder {
        type_ids: HashMap::new(),
        variable_ids: HashMap::new(),
    };
    builder.translate_program(ast)
}

pub struct HlirBuilder {
    type_ids: HashMap<Type, TypeID>,
    variable_ids: HashMap<Intern<String>, IdentID>,
}

impl HlirBuilder {
    pub fn translate_program(&mut self, ast: &AbstractSyntaxTree) -> HlirTree {
        let mut program = HlirTree::default();
        for expr in ast {
            program.push(self.translate_expr(expr.clone()).unwrap());
        }
        program
    }
    fn translate_expr(&mut self, expr: Expression) -> Result<HlirExpr, HlirError> {
        match expr {
            Expression::Literal(l) => Ok(HlirExpr::HlirLiteral(self.translate_literal(l)?)),
            Expression::VariableDeclaration(v) => {
                Ok(HlirExpr::HlirVarDecl(self.translate_variable(v)?))
            }
            Expression::BinaryExpression(b) => {
                Ok(HlirExpr::HlirBinaryExp(self.translate_binary_expr(b)?))
            }
            Expression::UnaryExpression(u) => {
                Ok(HlirExpr::HlirUnary(self.translate_unary_expr(u)?))
            }
            Expression::FunctionCall(f) => {
                Ok(HlirExpr::HlirFunCall(self.translate_function_call(f)?))
            }
            Expression::StructDeclaration(s) => {
                Ok(HlirExpr::HlirStructDecl(self.translate_structure(s)?))
            }
            Expression::Identifier(i) => Ok(HlirExpr::HlirIdent(self.translate_identifier(i)?)),
            Expression::IfElseNode(i) => Ok(HlirExpr::HlirIfElseExpr(self.translate_if_else(i)?)),
            Expression::FunctionExpression(f) => {
                Ok(HlirExpr::HlirFunExpr(self.translate_function(f)?))
            }
            Expression::DoExpression(d) => Ok(HlirExpr::HlirDoExpr(self.translate_do(d)?)),
            Expression::MatchExpression(m) => Ok(HlirExpr::HlirMatchExpr(self.translate_match(m)?)),
            Expression::IndexExpression(i) => Ok(HlirExpr::HlirIndexExpr(self.translate_index(i)?)),
            Expression::FieldAccessExpression(f) => {
                Ok(HlirExpr::HlirFieldAccess(self.translate_field_access(f)?))
            }
            Expression::NewObjectExpression(n) => {
                Ok(HlirExpr::HlirNewObjExpr(self.translate_new_object(n)?))
            }
        }
    }

    fn translate_field_access(
        &mut self,
        f: FieldAccessExpression,
    ) -> Result<HlirFieldAccess, HlirError> {
        let name = self.translate_name(f.name)?;
        let field = f.field;
        Ok(HlirFieldAccess {
            name,
            field,
            span: f.span,
        })
    }

    fn translate_index(&mut self, i: IndexExpression) -> Result<HlirIndexExpr, HlirError> {
        let name = self.translate_name(i.name)?;
        let index = Box::new(self.translate_expr(*i.index)?);
        Ok(HlirIndexExpr {
            name,
            index,
            span: i.span,
        })
    }

    fn translate_match(&mut self, m: MatchExpression) -> Result<HlirMatchExpr, HlirError> {
        let expr = Box::new(self.translate_expr(*m.expr)?);
        let mut arms = Vec::new();
        for arm in m.arms {
            arms.push(self.translate_match_arm(arm)?);
        }
        Ok(HlirMatchExpr {
            expr,
            arms,
            default: match m.default {
                Some(d) => Some(Box::new(self.translate_expr(*d)?)),
                None => None,
            },
            span: m.span,
        })
    }

    fn translate_match_arm(&mut self, arm: MatchArm) -> Result<HlirMatchArm, HlirError> {
        let pattern = Box::new(self.translate_expr(*arm.pattern)?);
        let body = Box::new(self.translate_expr(*arm.body)?);
        Ok(HlirMatchArm {
            pattern,
            body,
            span: arm.span,
        })
    }

    fn translate_do(&mut self, d: DoExpression) -> Result<HlirDoExpr, HlirError> {
        let mut body = Vec::new();
        for e in d.body {
            body.push(self.translate_expr(*e)?);
        }
        Ok(HlirDoExpr { body, span: d.span })
    }

    fn translate_new_object(
        &mut self,
        n: NewObjectExpression,
    ) -> Result<HlirNewObjExpr, HlirError> {
        let name = self.translate_name(n.name)?;
        let mut fields = Vec::new();
        for field in n.fields {
            fields.push(self.translate_expr(field)?);
        }
        Ok(HlirNewObjExpr {
            name,
            fields,
            span: n.span,
        })
    }

    fn translate_function(&mut self, f: FunctionExpression) -> Result<HlirFunExpr, HlirError> {
        let mut args = Vec::new();
        for (arg_name, arg_t) in f.args {
            let arg_name = self.translate_name(arg_name)?;
            let arg_t = self.translate_type(arg_t)?;
            args.push((arg_name, arg_t));
        }
        let body = self.translate_expr(*f.body)?;
        Ok(HlirFunExpr {
            args,
            body: Box::new(body),
            span: f.span,
        })
    }

    fn translate_if_else(&mut self, i: IfElseNode) -> Result<HlirIfElseExpr, HlirError> {
        let condition = self.translate_expr(*i.condition)?;
        let if_block = self.translate_expr(*i.if_body)?;
        let else_block = match i.else_body {
            Some(e) => Some(Box::new(self.translate_expr(*e)?)),
            None => None,
        };
        Ok(HlirIfElseExpr {
            condition: Box::new(condition),
            if_body: Box::new(if_block),
            else_body: else_block,
            span: i.span,
        })
    }

    fn translate_identifier(&mut self, i: IdentifierNode) -> Result<HlirIdentifier, HlirError> {
        Ok(HlirIdentifier {
            name: self.translate_name(i.name)?,
            span: i.span,
        })
    }

    fn translate_structure(&mut self, s: StructDeclaration) -> Result<HlirStructDecl, HlirError> {
        let name = self.translate_name(s.name)?;
        //Storing the structure as a type to be able to reference it later
        let _ = self.translate_type(Type::NonPrimitive(s.name))?;
        let mut fields = Vec::new();
        for field in s.fields {
            fields.push(self.translate_type(field)?);
        }
        Ok(HlirStructDecl {
            name,
            fields,
            span: s.span,
        })
    }

    fn translate_function_call(&mut self, f: FunctionCall) -> Result<HlirFunCall, HlirError> {
        let name = self.translate_name(f.name)?;
        let mut args = Vec::new();
        for arg in f.args {
            args.push(self.translate_expr(*arg)?);
        }
        Ok(HlirFunCall {
            name,
            args,
            span: f.span,
        })
    }

    fn translate_unary_expr(&mut self, un_expr: UnaryExpression) -> Result<HlirUnary, HlirError> {
        let expr = self.translate_expr(*un_expr.expression)?;
        Ok(HlirUnary {
            expression: Box::new(expr),
            operator: un_expr.operator.map(|op| (&op).into()),
            span: un_expr.span,
        })
    }

    fn translate_binary_expr(
        &mut self,
        bin_expr: BinaryExpression,
    ) -> Result<HlirBinaryExp, HlirError> {
        let left = self.translate_expr(*bin_expr.left)?;
        let right = self.translate_expr(*bin_expr.right)?;
        Ok(HlirBinaryExp {
            left: Box::new(left),
            right: Box::new(right),
            operator: (&bin_expr.operator).into(),
            span: bin_expr.span,
        })
    }

    fn translate_name(&mut self, name: Intern<String>) -> Result<u64, HlirError> {
        if let Some(id) = self.variable_ids.get(&name) {
            Ok(*id)
        } else {
            let id = get_variable_id();
            self.variable_ids.insert(name, id);
            Ok(id)
        }
    }

    fn translate_type(&mut self, ty: Type) -> Result<TypeID, HlirError> {
        if let Some(id) = self.type_ids.get(&ty) {
            Ok(*id)
        } else {
            let id = get_type_id();
            self.type_ids.insert(ty, id);
            Ok(id)
        }
    }

    fn translate_variable(&mut self, var: VariableDeclaration) -> Result<HlirVarDecl, HlirError> {
        let name = self.translate_name(var.name)?;
        let t = self.translate_type(var.t)?;
        match var.value {
            Some(v) => Ok(HlirVarDecl {
                name,
                t,
                value: Box::new(self.translate_expr(*v)?),
                span: var.span,
            }),
            _ => Ok(HlirVarDecl {
                name,
                t,
                value: Box::new(HlirExpr::HlirLiteral(HlirLiteral::Unit)),
                span: var.span,
            }),
        }
    }

    fn translate_literal(&mut self, literal: Literal) -> Result<HlirLiteral, HlirError> {
        match literal {
            Literal::Integer(i) => Ok(HlirLiteral::Int(i)),
            Literal::Float(f) => Ok(HlirLiteral::Float(f)),
            Literal::String(s) => Ok(HlirLiteral::String(s)),
            Literal::Bool(b) => Ok(HlirLiteral::Bool(b)),
            Literal::Unit => Ok(HlirLiteral::Unit),
            Literal::List(l) => {
                let mut list = Vec::new();
                for expr in l {
                    list.push(self.translate_expr(expr)?);
                }
                Ok(HlirLiteral::List(list))
            }
        }
    }
}
