use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::frontend::parser::Value;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum VMValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Identifier(Rc<RefCell<String>>),
    Array(Rc<RefCell<Vec<VMValue>>>),
}

impl fmt::Display for VMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(bool) => write!(f, "{}", bool),
            Self::Int(int) => write!(f, "{}", int),
            Self::Float(float) => write!(f, "{}", float),
            Self::String(string) => write!(f, "{}", string),
            Self::Identifier(identifier) => write!(f, "{}", identifier.borrow()),
            Self::Array(array) => {
                write!(f, "[")?;

                for (i, value) in array.borrow().iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", value)?;
                }

                write!(f, "]")
            }
        }
    }
}

impl Eq for VMValue {}

impl Ord for VMValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left.cmp(right),
            (Self::Float(left), Self::Float(right)) => left.partial_cmp(right).unwrap(),
            (Self::Int(left), Self::Float(right)) => (*left as f64).partial_cmp(right).unwrap(),
            (Self::Float(left), Self::Int(right)) => left.partial_cmp(&(*right as f64)).unwrap(),
            (Self::String(left), Self::String(right)) => left.cmp(right),
            (Self::Bool(left), Self::Bool(right)) => left.cmp(right),
            _ => panic!("cannot compare {} and {}", self.name(), other.name()),
        }
    }
}

impl VMValue {
    pub fn from(value: Value) -> Self {
        match value {
            Value::Bool(bool) => Self::Bool(bool),
            Value::Int(int) => Self::Int(int),
            Value::Float(float) => Self::Float(float),
            Value::String(string) => Self::String(string),
            Value::Identifier(identifier) => Self::Identifier(Rc::new(RefCell::new(identifier))),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Int(left + right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Float(left + right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Float(*left as f64 + right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Float(left + *right as f64)),
            _ => Err(format!("cannot add {} and {}", self.name(), other.name())),
        }
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Int(left - right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Float(left - right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Float(*left as f64 - right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Float(left - *right as f64)),
            _ => Err(format!(
                "cannot subtract {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Int(left * right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Float(left * right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Float(*left as f64 * right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Float(left * *right as f64)),
            _ => Err(format!(
                "cannot multiply {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Int(left / right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Float(left / right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Float(*left as f64 / right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Float(left / *right as f64)),
            _ => Err(format!(
                "cannot divide {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn mod_(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Int(left % right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Float(left % right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Float(*left as f64 % right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Float(left % *right as f64)),
            _ => Err(format!(
                "cannot modulo {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Bool(left == right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Bool(left == right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Bool(*left as f64 == *right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Bool(left == &(*right as f64))),
            (Self::String(left), Self::String(right)) => Ok(Self::Bool(left == right)),
            (Self::Bool(left), Self::Bool(right)) => Ok(Self::Bool(left == right)),
            _ => Err(format!(
                "cannot compare equality between {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Bool(left != right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Bool(left != right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Bool(*left as f64 != *right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Bool(left != &(*right as f64))),
            (Self::String(left), Self::String(right)) => Ok(Self::Bool(left != right)),
            (Self::Bool(left), Self::Bool(right)) => Ok(Self::Bool(left != right)),
            _ => Err(format!(
                "cannot compare inequality between {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn less(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Bool(left < right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Bool(left < right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Bool((*left as f64) < *right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Bool(left < &(*right as f64))),
            (Self::String(left), Self::String(right)) => Ok(Self::Bool(left < right)),
            _ => Err(format!(
                "cannot compare less than between {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn greater(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Bool(left > right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Bool(left > right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Bool(*left as f64 > *right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Bool(left > &(*right as f64))),
            (Self::String(left), Self::String(right)) => Ok(Self::Bool(left > right)),
            _ => Err(format!(
                "cannot compare greater than between {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn less_equals(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Bool(left <= right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Bool(left <= right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Bool(*left as f64 <= *right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Bool(left <= &(*right as f64))),
            (Self::String(left), Self::String(right)) => Ok(Self::Bool(left <= right)),
            _ => Err(format!(
                "cannot compare less than or equal to between {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn greater_equals(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => Ok(Self::Bool(left >= right)),
            (Self::Float(left), Self::Float(right)) => Ok(Self::Bool(left >= right)),
            (Self::Int(left), Self::Float(right)) => Ok(Self::Bool(*left as f64 >= *right)),
            (Self::Float(left), Self::Int(right)) => Ok(Self::Bool(left >= &(*right as f64))),
            (Self::String(left), Self::String(right)) => Ok(Self::Bool(left >= right)),
            _ => Err(format!(
                "cannot compare greater than or equal to between {} and {}",
                self.name(),
                other.name()
            )),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Identifier(_) => "identifier",
            Self::Array(_) => "array",
        }
        .to_string()
    }
}
