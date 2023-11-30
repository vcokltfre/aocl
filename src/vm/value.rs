use crate::frontend::parser::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum VMValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Array(Vec<VMValue>),
}

impl VMValue {
    pub fn from(value: Value) -> Self {
        match value {
            Value::Bool(bool) => Self::Bool(bool),
            Value::Int(int) => Self::Int(int),
            Value::Float(float) => Self::Float(float),
            Value::String(string) => Self::String(string),
            Value::Identifier(identifier) => Self::Identifier(identifier),
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

    fn name(&self) -> String {
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