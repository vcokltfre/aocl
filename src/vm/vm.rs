use std::collections::HashMap;

use crate::{
    errors::Error,
    frontend::parser::{BinOp, CallTarget, Compare, Statement, StatementContext, Value},
};

use super::{debugger, VMValue};

pub type VMFunc = fn(&mut VM, Vec<Option<String>>, Vec<VMValue>) -> Result<Option<VMValue>, String>;

pub struct VM {
    pub statements: Vec<Statement>,
    pub funcs: HashMap<String, VMFunc>,
    pub gotos: HashMap<String, usize>,
    pub variables: HashMap<String, VMValue>,
    pub index: usize,
    pub call_stack: Vec<usize>,
    pub stack: Vec<VMValue>,
    pub breakpoint: bool,
}

impl VM {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            funcs: HashMap::new(),
            gotos: HashMap::new(),
            variables: HashMap::new(),
            index: 0,
            call_stack: Vec::new(),
            stack: Vec::new(),
            breakpoint: false,
        }
    }

    pub fn register(&mut self, module: String, name: String, call: VMFunc) {
        self.funcs.insert(format!("{}:{}", module, name), call);
    }

    pub fn call(
        &mut self,
        module: String,
        name: String,
        idts: Vec<Option<String>>,
        args: Vec<VMValue>,
    ) -> Result<Option<VMValue>, String> {
        let func = self.funcs.get(&format!("{}:{}", module, name));

        if let Some(func) = func {
            match func(self, idts, args) {
                Ok(value) => return Ok(value),
                Err(e) => return Err(e),
            }
        } else {
            return Err(format!("function not found: {}:{}", module, name));
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.register(
            "vm".to_string(),
            "debug".to_string(),
            |_self, _idents, args| {
                println!("{:?}", args);
                Ok(None)
            },
        );

        for (i, statement) in self.statements.iter().enumerate() {
            if let StatementContext::GotoDef(identifier) = statement.context.clone() {
                self.gotos.insert(identifier.clone(), i);
            }
        }

        while self.index < self.statements.len() {
            self.step()?;
        }

        Ok(())
    }

    fn step(&mut self) -> Result<(), Error> {
        let statement = self.statements[self.index].clone();

        let res = match statement.clone().context {
            StatementContext::AssignLiteral(identifier, value) => {
                self.op_assign_literal(identifier, value)
            }
            StatementContext::AssignBinOp(identifier, binop) => {
                self.op_assign_binop(identifier, binop)
            }
            StatementContext::AssignCall(identifier, call_target, args) => {
                self.op_assign_call(identifier, call_target, args)
            }
            StatementContext::GotoDef(_) => Ok(()),
            StatementContext::Goto(identifier) => {
                if let Some(index) = self.gotos.get(&identifier) {
                    self.index = *index;
                } else {
                    return Err(statement.error(format!("goto not found: {}", identifier)));
                }

                Ok(())
            }
            StatementContext::GotoIf(identifier, compare) => self.op_goto_if(identifier, compare),
            StatementContext::Call(call_target, args) => self.op_call(call_target, args),
            StatementContext::CallLabel(label) => self.op_call_label(label),
            StatementContext::Ret => self.op_ret(),
            StatementContext::EOS => Err("unexpected end of statement".to_string()),
        };

        if let Err(e) = res {
            return Err(statement.error(e));
        }

        self.index += 1;

        if self.breakpoint {
            self.breakpoint = false;

            debugger::debugger(self);
        }

        Ok(())
    }

    fn op_assign_literal(&mut self, identifier: String, value: Value) -> Result<(), String> {
        match value {
            Value::Identifier(id) => {
                if let Some(value) = self.variables.get(&id) {
                    self.variables.insert(identifier, value.clone());
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            }
            _ => {
                self.variables.insert(identifier, VMValue::from(value));
            },
        }

        Ok(())
    }

    fn op_assign_binop(&mut self, identifier: String, binop: BinOp) -> Result<(), String> {
        let (raw_lhs, raw_rhs) = match binop.clone() {
            BinOp::Add(lhs, rhs) => (lhs, rhs),
            BinOp::Sub(lhs, rhs) => (lhs, rhs),
            BinOp::Mul(lhs, rhs) => (lhs, rhs),
            BinOp::Div(lhs, rhs) => (lhs, rhs),
            BinOp::Mod(lhs, rhs) => (lhs, rhs),
        };

        let lhs = match raw_lhs {
            Value::Identifier(identifier) => {
                if let Some(value) = self.variables.get(&identifier) {
                    value.clone()
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            }
            _ => VMValue::from(raw_lhs),
        };

        let rhs = match raw_rhs {
            Value::Identifier(identifier) => {
                if let Some(value) = self.variables.get(&identifier) {
                    value.clone()
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            }
            _ => VMValue::from(raw_rhs),
        };

        let value = match binop {
            BinOp::Add(_, _) => lhs.add(&rhs)?,
            BinOp::Sub(_, _) => lhs.sub(&rhs)?,
            BinOp::Mul(_, _) => lhs.mul(&rhs)?,
            BinOp::Div(_, _) => lhs.div(&rhs)?,
            BinOp::Mod(_, _) => lhs.mod_(&rhs)?,
        };

        self.variables.insert(identifier, value);

        Ok(())
    }

    fn op_assign_call(
        &mut self,
        identifier: String,
        call_target: CallTarget,
        args: Vec<Value>,
    ) -> Result<(), String> {
        let mut vmargs = Vec::<VMValue>::new();
        let mut idts = Vec::new();

        for value in args {
            if let Value::Identifier(identifier) = value.clone() {
                if let Some(value) = self.variables.get(&identifier) {
                    vmargs.push(value.clone());
                    idts.push(Some(identifier));
                    continue;
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            } else {
                idts.push(None);
            }

            vmargs.push(VMValue::from(value));
        }

        let value = self.call(
            call_target.module.clone(),
            call_target.function.clone(),
            idts,
            vmargs,
        )?;

        if let Some(value) = value {
            self.variables.insert(identifier, value);
        } else {
            return Err(format!(
                "function did not return a value: @{}:{}",
                call_target.module, call_target.function
            ));
        }

        Ok(())
    }

    fn op_goto_if(&mut self, identifier: String, compare: Compare) -> Result<(), String> {
        let (raw_lhs, raw_rhs) = match compare.clone() {
            Compare::Equals(lhs, rhs) => (lhs, rhs),
            Compare::NotEquals(lhs, rhs) => (lhs, rhs),
            Compare::LessThanEquals(lhs, rhs) => (lhs, rhs),
            Compare::GreaterThanEquals(lhs, rhs) => (lhs, rhs),
            Compare::LessThan(lhs, rhs) => (lhs, rhs),
            Compare::GreaterThan(lhs, rhs) => (lhs, rhs),
        };

        let lhs = match raw_lhs {
            Value::Identifier(identifier) => {
                if let Some(value) = self.variables.get(&identifier) {
                    value.clone()
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            }
            _ => VMValue::from(raw_lhs),
        };

        let rhs = match raw_rhs {
            Value::Identifier(identifier) => {
                if let Some(value) = self.variables.get(&identifier) {
                    value.clone()
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            }
            _ => VMValue::from(raw_rhs),
        };

        let value = match compare {
            Compare::Equals(_, _) => lhs.equals(&rhs)?,
            Compare::NotEquals(_, _) => lhs.not_equals(&rhs)?,
            Compare::LessThanEquals(_, _) => lhs.less_equals(&rhs)?,
            Compare::GreaterThanEquals(_, _) => lhs.greater_equals(&rhs)?,
            Compare::LessThan(_, _) => lhs.less(&rhs)?,
            Compare::GreaterThan(_, _) => lhs.greater(&rhs)?,
        };

        if let VMValue::Bool(jump) = value {
            if jump {
                if let Some(index) = self.gotos.get(&identifier) {
                    self.index = *index;
                } else {
                    return Err(format!("goto not found: {}", identifier));
                }
            }
        }

        Ok(())
    }

    fn op_call(&mut self, target: CallTarget, values: Vec<Value>) -> Result<(), String> {
        let mut args = Vec::new();
        let mut idts = Vec::new();

        for value in values {
            if let Value::Identifier(identifier) = value.clone() {
                if let Some(value) = self.variables.get(&identifier) {
                    args.push(value.clone());
                    idts.push(Some(identifier));
                    continue;
                } else {
                    return Err(format!("variable not found: {}", identifier));
                }
            } else {
                idts.push(None);
            }

            args.push(VMValue::from(value));
        }

        self.call(target.module, target.function, idts, args)?;

        Ok(())
    }

    fn op_call_label(&mut self, label: String) -> Result<(), String> {
        self.call_stack.push(self.index);
        self.index = *self.gotos.get(&label).unwrap();

        Ok(())
    }

    fn op_ret(&mut self) -> Result<(), String> {
        if let Some(index) = self.call_stack.pop() {
            self.index = index;
        } else {
            return Err("call stack is empty".to_string());
        }

        Ok(())
    }
}
