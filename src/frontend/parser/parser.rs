use crate::errors::Error;
use crate::frontend::lexer::{Token, TokenType};

use super::{BinOp, Compare, Statement, Value};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

type ParseResult = Result<Statement, Error>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn get_statement(&self) -> Result<Vec<Token>, Error> {
        let next_end = self.tokens[self.current..]
            .iter()
            .position(|t| t.token_type == TokenType::EOS);

        if let Some(end) = next_end {
            Ok(self.tokens[self.current..=self.current + end].to_vec())
        } else {
            Err(self.tokens[self.current].error("Incomplete statement".to_string()))
        }
    }

    pub fn require(&self, n: usize) -> Result<Vec<Token>, Error> {
        let stmt = self.get_statement()?;
        if stmt.len() != n {
            return Err(self.tokens[self.current].error("Incomplete statement".to_string()));
        }

        Ok(stmt)
    }

    pub fn parse_import(&mut self) -> ParseResult {
        let tokens = self.require(3)?;

        if !tokens[1].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        self.current += 3;

        let name = match tokens[1].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        Ok(Statement::Import(name))
    }

    pub fn parse_assign_literal(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if tokens[1].token_type != TokenType::Equals {
            return Err(tokens[1].error(format!("Expected '=', found {:?}", tokens[1].token_type)));
        }

        if !tokens[2].is_value() {
            return Err(tokens[1].error(format!(
                "Expected literal, found {:?}",
                tokens[1].token_type
            )));
        }

        self.current += 4;

        let name = match tokens[0].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        let value = match tokens[2].token_type.clone() {
            TokenType::Bool(b) => Value::Bool(b),
            TokenType::Int(i) => Value::Int(i),
            TokenType::Float(f) => Value::Float(f),
            TokenType::String(s) => Value::String(s),
            TokenType::Identifier(name) => Value::Identifier(name),
            _ => unreachable!(),
        };

        Ok(Statement::AssignLiteral(name, value))
    }

    pub fn parse_assign_binop(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if !tokens[2].is_value() {
            return Err(
                tokens[1].error(format!("Expected value, found {:?}", tokens[1].token_type))
            );
        }

        if !tokens[4].is_value() {
            return Err(
                tokens[1].error(format!("Expected value, found {:?}", tokens[1].token_type))
            );
        }

        self.current += 6;

        let name = match tokens[0].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        let lhs = match tokens[2].token_type.clone() {
            TokenType::Bool(b) => Value::Bool(b),
            TokenType::Int(i) => Value::Int(i),
            TokenType::Float(f) => Value::Float(f),
            TokenType::String(s) => Value::String(s),
            TokenType::Identifier(name) => Value::Identifier(name),
            _ => unreachable!(),
        };

        let rhs = match tokens[4].token_type.clone() {
            TokenType::Bool(b) => Value::Bool(b),
            TokenType::Int(i) => Value::Int(i),
            TokenType::Float(f) => Value::Float(f),
            TokenType::String(s) => Value::String(s),
            TokenType::Identifier(name) => Value::Identifier(name),
            _ => unreachable!(),
        };

        let binop = match tokens[3].token_type.clone() {
            TokenType::Plus => BinOp::Add(lhs, rhs),
            TokenType::Minus => BinOp::Sub(lhs, rhs),
            TokenType::Star => BinOp::Mul(lhs, rhs),
            TokenType::Slash => BinOp::Div(lhs, rhs),
            TokenType::Percent => BinOp::Mod(lhs, rhs),
            _ => unreachable!(),
        };

        Ok(Statement::AssignBinOp(name, binop))
    }

    pub fn parse_assign_call(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if tokens[2].token_type != TokenType::At {
            return Err(tokens[1].error(format!("Expected '@', found {:?}", tokens[1].token_type)));
        }

        if !tokens[3].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        if tokens[4].token_type != TokenType::Colon {
            return Err(tokens[1].error(format!("Expected ':', found {:?}", tokens[1].token_type)));
        }

        if !tokens[5].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        let argl = tokens.len() - 7;

        let mut values = Vec::<Value>::new();

        for i in 0..argl {
            if !tokens[6 + i].is_value() {
                return Err(
                    tokens[1].error(format!("Expected value, found {:?}", tokens[1].token_type))
                );
            }

            values.push(match tokens[6 + i].token_type.clone() {
                TokenType::Bool(b) => Value::Bool(b),
                TokenType::Int(i) => Value::Int(i),
                TokenType::Float(f) => Value::Float(f),
                TokenType::String(s) => Value::String(s),
                TokenType::Identifier(name) => Value::Identifier(name),
                _ => unreachable!(),
            });
        }

        self.current += tokens.len();

        let name = match tokens[0].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        let module_name = match tokens[3].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        let function_name = match tokens[5].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        Ok(Statement::AssignCall(
            name,
            crate::frontend::parser::CallTarget {
                module: module_name,
                function: function_name,
            },
            values,
        ))
    }

    pub fn parse_assign(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if tokens[1].token_type != TokenType::Equals {
            return Err(tokens[1].error(format!("Expected '=', found {:?}", tokens[1].token_type)));
        }

        let binop = tokens[3].is_binop();

        if tokens.len() == 4 {
            return self.parse_assign_literal();
        } else if tokens.len() == 6 && binop {
            return self.parse_assign_binop();
        } else if tokens.len() >= 7 && !binop {
            return self.parse_assign_call();
        }

        Err(tokens[1].error("Invalid assignment".to_string()))
    }

    pub fn parse_goto_def(&mut self) -> ParseResult {
        let tokens = self.require(3)?;

        if !tokens[1].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        self.current += 3;

        let name = match tokens[1].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        Ok(Statement::GotoDef(name))
    }

    pub fn parse_goto_always(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if !tokens[1].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        self.current += 3;

        let name = match tokens[1].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        Ok(Statement::Goto(name))
    }

    pub fn parse_goto_if(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if !tokens[1].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        if tokens[2].token_type != TokenType::If {
            return Err(tokens[2].error(format!("Expected 'if', found {:?}", tokens[2].token_type)));
        }

        if !tokens[3].is_value() {
            return Err(
                tokens[3].error(format!("Expected value, found {:?}", tokens[3].token_type))
            );
        }

        if !tokens[4].is_compare() {
            return Err(tokens[4].error(format!(
                "Expected comparison, found {:?}",
                tokens[4].token_type
            )));
        }

        if !tokens[5].is_value() {
            return Err(
                tokens[5].error(format!("Expected value, found {:?}", tokens[5].token_type))
            );
        }

        self.current += 7;

        let goto_name = match tokens[1].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        let lhs = match tokens[3].token_type.clone() {
            TokenType::Bool(b) => Value::Bool(b),
            TokenType::Int(i) => Value::Int(i),
            TokenType::Float(f) => Value::Float(f),
            TokenType::String(s) => Value::String(s),
            TokenType::Identifier(name) => Value::Identifier(name),
            _ => unreachable!(),
        };

        let rhs = match tokens[5].token_type.clone() {
            TokenType::Bool(b) => Value::Bool(b),
            TokenType::Int(i) => Value::Int(i),
            TokenType::Float(f) => Value::Float(f),
            TokenType::String(s) => Value::String(s),
            TokenType::Identifier(name) => Value::Identifier(name),
            _ => unreachable!(),
        };

        let compare = match tokens[4].token_type.clone() {
            TokenType::EqualsEquals => Compare::Equals(lhs, rhs),
            TokenType::BangEquals => Compare::NotEquals(lhs, rhs),
            TokenType::Less => Compare::LessThan(lhs, rhs),
            TokenType::Greater => Compare::GreaterThan(lhs, rhs),
            TokenType::LessEquals => Compare::LessThanEquals(lhs, rhs),
            TokenType::GreaterEquals => Compare::GreaterThanEquals(lhs, rhs),
            _ => unreachable!(),
        };

        Ok(Statement::GotoIf(goto_name, compare))
    }

    pub fn parse_goto(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if !tokens[1].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        if tokens.len() == 3 {
            return self.parse_goto_always();
        } else if tokens.len() == 7 {
            return self.parse_goto_if();
        }

        Err(tokens[1].error("Invalid goto statement".to_string()))
    }

    pub fn parse_call(&mut self) -> ParseResult {
        let tokens = self.get_statement()?;

        if !tokens[1].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        if tokens[2].token_type != TokenType::Colon {
            return Err(tokens[1].error(format!("Expected ':', found {:?}", tokens[1].token_type)));
        }

        if !tokens[3].is_identifier() {
            return Err(tokens[1].error(format!(
                "Expected identifier, found {:?}",
                tokens[1].token_type
            )));
        }

        let argl = tokens.len() - 5;
        let mut values = Vec::<Value>::new();

        for i in 0..argl {
            if !tokens[4 + i].is_value() {
                return Err(
                    tokens[1].error(format!("Expected value, found {:?}", tokens[1].token_type))
                );
            }

            values.push(match tokens[4 + i].token_type.clone() {
                TokenType::Bool(b) => Value::Bool(b),
                TokenType::Int(i) => Value::Int(i),
                TokenType::Float(f) => Value::Float(f),
                TokenType::String(s) => Value::String(s),
                TokenType::Identifier(name) => Value::Identifier(name),
                _ => unreachable!(),
            });
        }

        self.current += tokens.len();

        let module_name = match tokens[1].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        let function_name = match tokens[1].token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => unreachable!(),
        };

        return Ok(Statement::Call(
            crate::frontend::parser::CallTarget {
                module: module_name,
                function: function_name,
            },
            values,
        ));
    }

    pub fn parse_statement(&mut self) -> ParseResult {
        let token = self.tokens[self.current].clone();

        match token.token_type {
            TokenType::Import => self.parse_import(),
            TokenType::Tilde => self.parse_goto_def(),
            TokenType::At => self.parse_call(),
            TokenType::Goto => self.parse_goto(),
            TokenType::Identifier(_) => self.parse_assign(),

            _ => Err(token.error(format!("Unexpected token"))),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        loop {
            if self.current >= self.tokens.len() {
                break;
            }

            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }
}