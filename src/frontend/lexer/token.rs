#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Literals
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),

    // Identifiers
    Identifier(String), // /[a-zA-Z0-9_]+/

    // Keywords
    Goto, // goto
    If,   // if

    // Single-character tokens
    Colon,   // :
    Equals,  // =
    At,      // @
    Tilde,   // ~
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Less,    // <
    Greater, // >

    // Two-character tokens
    EqualsEquals,  // ==
    BangEquals,    // !=
    LessEquals,    // <=
    GreaterEquals, // >=

    // Meta
    EOS,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,

    pub line: usize,
    pub column: usize,
    pub index: usize,
    pub width: usize,
    pub file: String,

    pub context: String,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        column: usize,
        index: usize,
        width: usize,
        file: String,
        context: String,
    ) -> Self {
        Self {
            token_type,
            line,
            column,
            index,
            width,
            file,
            context,
        }
    }

    pub fn error(&self, message: String) -> crate::errors::Error {
        crate::errors::Error::new(
            self.line,
            self.column,
            self.index,
            self.width,
            self.file.clone(),
            message,
            self.context.clone(),
            crate::errors::ErrorLocation::Parser,
        )
    }

    pub fn is_literal(&self) -> bool {
        match self.token_type {
            TokenType::Bool(_) | TokenType::Int(_) | TokenType::Float(_) | TokenType::String(_) => {
                true
            }
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self.token_type {
            TokenType::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        self.is_literal() || self.is_identifier()
    }

    pub fn is_compare(&self) -> bool {
        match self.token_type {
            TokenType::EqualsEquals
            | TokenType::BangEquals
            | TokenType::LessEquals
            | TokenType::GreaterEquals
            | TokenType::Less
            | TokenType::Greater => true,
            _ => false,
        }
    }

    pub fn is_binop(&self) -> bool {
        match self.token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Percent => true,
            _ => false,
        }
    }
}
