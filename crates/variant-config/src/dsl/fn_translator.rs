use super::frontend::*;
use super::utils::get_string_hash;
use super::HashMap;
use super::RandomState;
use super::{BOOL, INT};
use anyhow::bail;
use cranelift::prelude::*;

pub struct ValueWrapper {
    pub value: Value,
    pub r#type: Type,
}

impl ValueWrapper {
    pub fn new(value: Value, r#type: Type) -> ValueWrapper {
        ValueWrapper { value, r#type }
    }
}

pub struct FunctionTranslator<'a> {
    random_state: &'a RandomState,
    builder: &'a mut FunctionBuilder<'a>,
    variables: &'a mut HashMap<String, Variable>,
    entry_block: Block,
}

impl<'a> FunctionTranslator<'a> {
    pub fn new(
        random_state: &'a RandomState,
        builder: &'a mut FunctionBuilder<'a>,
        variables: &'a mut HashMap<String, Variable>,
        entry_block: Block,
    ) -> FunctionTranslator<'a> {
        FunctionTranslator {
            random_state,
            builder,
            variables,
            entry_block,
        }
    }

    pub fn convert_int_to_bool(&mut self, v: ValueWrapper) -> ValueWrapper {
        if v.r#type == BOOL {
            return v;
        } else {
            let return_true = self.builder.ins().bconst(BOOL, true);
            let return_false = self.builder.ins().bconst(BOOL, false);
            return ValueWrapper::new(
                self.builder
                    .ins()
                    .select(v.value, return_true, return_false),
                BOOL,
            );
        }
    }

    fn translate_expr_bool(&mut self, expr: Expr) -> anyhow::Result<ValueWrapper> {
        let ret = self.translate_expr(expr)?;
        Ok(self.convert_int_to_bool(ret))
    }

    fn translate_expr_int(&mut self, expr: Box<Expr>) -> anyhow::Result<ValueWrapper> {
        let v = self.translate_expr(*expr)?;
        if v.r#type != INT {
            bail!(format!("Invalid type {}", v.r#type))
        } else {
            Ok(v)
        }
    }

    pub fn translate_expr(&mut self, expr: Expr) -> anyhow::Result<ValueWrapper> {
        match expr {
            Expr::IntLiteral(literal) => Ok(ValueWrapper::new(
                self.builder.ins().iconst(INT, literal),
                INT,
            )),

            Expr::BoolLiteral(literal) => Ok(ValueWrapper::new(
                self.builder.ins().bconst(BOOL, literal),
                BOOL,
            )),

            Expr::StringLiteral(literal) => {
                let hash = get_string_hash(self.random_state, &literal);
                Ok(ValueWrapper::new(
                    self.builder.ins().iconst(INT, hash),
                    types::R64,
                ))
            }

            Expr::Add(lhs, rhs) => {
                let lhs = self.translate_expr_int(lhs)?;
                let rhs = self.translate_expr_int(rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().iadd(lhs.value, rhs.value),
                    INT,
                ))
            }

            Expr::Sub(lhs, rhs) => {
                let lhs = self.translate_expr_int(lhs)?;
                let rhs = self.translate_expr_int(rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().isub(lhs.value, rhs.value),
                    INT,
                ))
            }

            Expr::Mul(lhs, rhs) => {
                let lhs = self.translate_expr_int(lhs)?;
                let rhs = self.translate_expr_int(rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().imul(lhs.value, rhs.value),
                    INT,
                ))
            }

            Expr::Div(lhs, rhs) => {
                let lhs = self.translate_expr_int(lhs)?;
                let rhs = self.translate_expr_int(rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().sdiv(lhs.value, rhs.value),
                    INT,
                ))
            }

            Expr::Mod(lhs, rhs) => {
                let lhs = self.translate_expr_int(lhs)?;
                let rhs = self.translate_expr_int(rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().srem(lhs.value, rhs.value),
                    INT,
                ))
            }

            Expr::And(lhs, rhs) => {
                let lhs = self.translate_expr_bool(*lhs)?;
                let rhs = self.translate_expr_bool(*rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().band(lhs.value, rhs.value),
                    BOOL,
                ))
            }

            Expr::Or(lhs, rhs) => {
                let lhs = self.translate_expr_bool(*lhs)?;
                let rhs = self.translate_expr_bool(*rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().bor(lhs.value, rhs.value),
                    BOOL,
                ))
            }

            Expr::Eq(lhs, rhs) => self.translate_icmp(IntCC::Equal, lhs, rhs),
            Expr::Ne(lhs, rhs) => self.translate_icmp(IntCC::NotEqual, lhs, rhs),
            Expr::Lt(lhs, rhs) => self.translate_icmp(IntCC::SignedLessThan, lhs, rhs),
            Expr::Le(lhs, rhs) => self.translate_icmp(IntCC::SignedLessThanOrEqual, lhs, rhs),
            Expr::Gt(lhs, rhs) => self.translate_icmp(IntCC::SignedGreaterThan, lhs, rhs),
            Expr::Ge(lhs, rhs) => self.translate_icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
            Expr::Identifier(name) => {
                if let Some(&variable) = self.variables.get(&name) {
                    Ok(ValueWrapper::new(self.builder.use_var(variable), INT))
                } else {
                    let idx = self.variables.len();
                    let var = Variable::new(idx);
                    self.variables.insert(name, var);
                    self.builder.declare_var(var, INT);
                    let val = self.builder.block_params(self.entry_block)[idx];
                    self.builder.def_var(var, val);
                    Ok(ValueWrapper::new(val, INT))
                }
            }
        }
    }

    fn translate_icmp(
        &mut self,
        cmp: IntCC,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    ) -> anyhow::Result<ValueWrapper> {
        match cmp {
            IntCC::Equal | IntCC::NotEqual => {
                let lhs = self.translate_expr(*lhs)?;
                let rhs = self.translate_expr(*rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().icmp(cmp, lhs.value, rhs.value),
                    INT,
                ))
            }
            _ => {
                let lhs = self.translate_expr_int(lhs)?;
                let rhs = self.translate_expr_int(rhs)?;
                Ok(ValueWrapper::new(
                    self.builder.ins().icmp(cmp, lhs.value, rhs.value),
                    INT,
                ))
            }
        }
    }

    pub fn return_and_finalize(&mut self, ret: Value) {
        self.builder.ins().return_(&[ret]);
        self.builder.finalize();
    }
}
