#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Identifier(String),
}

impl Value {
    pub fn rewrite(&self) -> String {
        match self {
            Value::String(string) => {
                format!(
                    "\"{}\"",
                    string
                        .replace("\\", "\\\\")
                        .replace("\"", "\\\"")
                        .replace("\n", "\\n")
                        .replace("\t", "\\t")
                        .replace("\r", "\\r")
                        .replace("\0", "\\0")
                )
            }
            Value::Int(int) => int.to_string(),
            Value::Float(float) => float.to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::Identifier(identifier) => identifier.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
}

impl BinOp {
    pub fn rewrite(&self) -> String {
        match self {
            BinOp::Add(left, right) => format!("{} + {}", left.rewrite(), right.rewrite()),
            BinOp::Sub(left, right) => format!("{} - {}", left.rewrite(), right.rewrite()),
            BinOp::Mul(left, right) => format!("{} * {}", left.rewrite(), right.rewrite()),
            BinOp::Div(left, right) => format!("{} / {}", left.rewrite(), right.rewrite()),
            BinOp::Mod(left, right) => format!("{} % {}", left.rewrite(), right.rewrite()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Compare {
    Equals(Value, Value),
    NotEquals(Value, Value),
    LessThan(Value, Value),
    GreaterThan(Value, Value),
    LessThanEquals(Value, Value),
    GreaterThanEquals(Value, Value),
}

impl Compare {
    pub fn rewrite(&self) -> String {
        match self {
            Compare::Equals(left, right) => format!("{} == {}", left.rewrite(), right.rewrite()),
            Compare::NotEquals(left, right) => {
                format!("{} != {}", left.rewrite(), right.rewrite())
            }
            Compare::LessThan(left, right) => {
                format!("{} < {}", left.rewrite(), right.rewrite())
            }
            Compare::GreaterThan(left, right) => {
                format!("{} > {}", left.rewrite(), right.rewrite())
            }
            Compare::LessThanEquals(left, right) => {
                format!("{} <= {}", left.rewrite(), right.rewrite())
            }
            Compare::GreaterThanEquals(left, right) => {
                format!("{} >= {}", left.rewrite(), right.rewrite())
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallTarget {
    pub module: String,
    pub function: String,
}

impl CallTarget {
    pub fn rewrite(&self) -> String {
        format!("@{}:{}", self.module, self.function)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Import(String),
    AssignLiteral(String, Value),
    AssignBinOp(String, BinOp),
    AssignCall(String, CallTarget, Vec<Value>),
    GotoDef(String),
    Goto(String),
    GotoIf(String, Compare),
    Call(CallTarget, Vec<Value>),
}

impl Statement {
    pub fn rewrite(&self) -> String {
        match self {
            Statement::Import(module) => format!("import {}", module),
            Statement::AssignLiteral(identifier, value) => {
                format!("{} = {}", identifier, value.rewrite())
            }
            Statement::AssignBinOp(identifier, binop) => {
                format!("{} = {}", identifier, binop.rewrite())
            }
            Statement::AssignCall(identifier, call_target, args) => {
                format!(
                    "{} = {} {}",
                    identifier,
                    call_target.rewrite(),
                    args.iter()
                        .map(|arg| arg.rewrite())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Statement::GotoDef(identifier) => format!("~{}", identifier),
            Statement::Goto(identifier) => format!("goto {}", identifier),
            Statement::GotoIf(identifier, compare) => {
                format!("goto {} if {}", identifier, compare.rewrite())
            }
            Statement::Call(call_target, args) => {
                format!(
                    "{} {}",
                    call_target.rewrite(),
                    args.iter()
                        .map(|arg| arg.rewrite())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        }
    }
}
