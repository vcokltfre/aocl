use super::{Token, TokenType};
use crate::errors::Error;

type TokenisationResult = Result<Token, Error>;

pub struct Lexer {
    pub filename: String,
    pub source: String,
    pub index: u32,
    pub line: u32,
    pub column: u32,
    pub lines: Vec<String>,
}

impl Lexer {
    pub fn new(filename: String, source: String) -> Lexer {
        let lines = source.clone().lines().map(|s| s.to_string()).collect();

        Lexer {
            filename,
            source,
            index: 0,
            line: 1,
            column: 1,
            lines: lines,
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::<Token>::new();
        loop {
            let token = self.get_token()?;
            if token.token_type == TokenType::EOF {
                break;
            }

            if tokens.len() >= 1 && tokens[tokens.len() - 1].token_type == TokenType::EOS {
                if token.token_type == TokenType::EOS {
                    continue;
                }
            }

            tokens.push(token);
        }

        let mut runs = Vec::<Vec<Token>>::new();
        let mut run = Vec::<Token>::new();

        for token in tokens {
            if token.token_type != TokenType::EOS {
                run.push(token);
            } else {
                runs.push(run);
                run = Vec::<Token>::new();
            }
        }

        let mut combined_tokens = Vec::<Token>::new();

        for run in runs {
            if run.len() != 2 {
                combined_tokens.append(&mut run.clone());
                continue;
            }

            if !run[0].clone().is_identifier() {
                combined_tokens.append(&mut run.clone());
                continue;
            }

            if !run[1].is_string() {
                combined_tokens.append(&mut run.clone());
                continue;
            }

            let ident_name = match run[0].clone().token_type {
                TokenType::Identifier(s) => s,
                _ => unreachable!(),
            };

            if ident_name != "import" {
                combined_tokens.append(&mut run.clone());
                continue;
            }

            let filename = match run[1].clone().token_type {
                TokenType::String(s) => s,
                _ => unreachable!(),
            };

            let data = std::fs::read_to_string(filename.clone());
            if data.is_err() {
                return Err(self.error(format!(
                    "Failed to import file '{}': {}",
                    filename,
                    data.err().unwrap()
                )));
            }

            let mut lexer = Lexer::new(filename.clone(), data.unwrap());
            let tokens = lexer.tokenise();

            match tokens {
                Ok(tokens) => {
                    combined_tokens.append(&mut tokens.clone());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        combined_tokens.push(Token::new(
            TokenType::EOS,
            self.line as usize,
            self.column as usize,
            self.index as usize,
            0,
            self.filename.clone(),
            "".to_string(),
        ));

        let mut final_tokens = Vec::<Token>::new();
        let mut skip_eos = false;

        for token in combined_tokens {
            if token.token_type == TokenType::EOS && skip_eos {
                continue;
            } else if token.token_type == TokenType::EOS {
                skip_eos = true;
            } else {
                skip_eos = false;
            }
            final_tokens.push(token);
        }

        Ok(final_tokens)
    }

    fn error(&self, message: String) -> Error {
        Error::new(
            self.line as usize,
            self.column as usize,
            self.index as usize,
            1,
            self.filename.clone(),
            message,
            self.lines[self.line as usize - 1].clone(),
            crate::errors::ErrorLocation::Lexer,
        )
    }

    fn make_token(&self, token_type: TokenType, width: u32) -> Token {
        if self.lines.len() == (self.line as usize - 1) {
            return Token::new(
                token_type,
                self.line as usize,
                self.column as usize - width as usize,
                self.index as usize - width as usize,
                width as usize,
                self.filename.clone(),
                "".to_string(),
            );
        }

        Token::new(
            token_type,
            self.line as usize,
            self.column as usize - width as usize,
            self.index as usize - width as usize,
            width as usize,
            self.filename.clone(),
            self.lines[self.line as usize - 1].clone(),
        )
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.index as usize);
        if c.is_some() {
            self.index += 1;
            self.column += 1;
        }
        c
    }

    fn peek(&self, offset: u32) -> Option<char> {
        self.source.chars().nth((self.index + offset) as usize)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek(0);
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn is_end(&self) -> bool {
        self.index >= self.source.len() as u32
    }

    fn is_boundary(&self) -> bool {
        let c = self.peek(0);
        if c.is_none() {
            return true;
        }

        match c.unwrap() {
            ' ' | '\t' | '\r' | '\n' | '=' | '+' | '-' | '*' | '/' | '%' | '!' | '>' | '<'
            | ':' | '~' | '@' => true,
            _ => false,
        }
    }

    fn get_number(&mut self) -> TokenisationResult {
        let mut number = String::new();
        let mut is_float = false;

        loop {
            let c = self.peek(0);
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                '0'..='9' => {
                    number.push(c.unwrap());
                    self.advance();
                }
                '.' => {
                    let next = self.peek(1);
                    if next.is_some() {
                        if next.unwrap() == '.' {
                            break;
                        }
                    }

                    if is_float {
                        return Err(self.error(
                            "Illegal second decimal point in float literal: '.'".to_string(),
                        ));
                    }
                    number.push(c.unwrap());
                    self.advance();
                    is_float = true;
                }
                _ => {
                    break;
                }
            }
        }

        if !self.is_boundary() {
            let c = self.peek(0).unwrap().to_string();
            return Err(self.error("Unexpected character in numeric literal: ".to_string() + &c));
        }

        if is_float {
            Ok(self.make_token(
                TokenType::Float(number.parse().unwrap()),
                number.len().try_into().unwrap(),
            ))
        } else {
            Ok(self.make_token(
                TokenType::Int(number.parse().unwrap()),
                number.len().try_into().unwrap(),
            ))
        }
    }

    fn get_string(&mut self) -> TokenisationResult {
        let mut value = String::new();

        self.advance();

        let mut escape = false;

        loop {
            let c = self.advance();

            if c.is_none() {
                return Err(self.error("Unterminated string literal".to_string()));
            }

            let c = c.unwrap();

            if escape {
                match c {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '0' => value.push('\0'),
                    '\'' => value.push('\''),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    _ => {
                        return Err(
                            self.error("Invalid escape sequence: \\".to_string() + &c.to_string())
                        )
                    }
                }
                escape = false;
                continue;
            }

            if c == '\\' {
                escape = true;
                continue;
            }

            if c == '"' {
                break;
            }

            value.push(c);
        }

        let value_len = value.len();

        Ok(self.make_token(
            TokenType::String(value),
            (value_len + 2).try_into().unwrap(),
        ))
    }

    fn get_ident(&mut self) -> TokenisationResult {
        let mut ident = String::new();

        loop {
            let c = self.peek(0);
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    ident.push(c.unwrap());
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }

        if !self.is_boundary() {
            let c = self.peek(0).unwrap().to_string();
            return Err(self.error("Unexpected character in identifier: ".to_string() + &c));
        }

        let ident_size = ident.len();

        match ident.as_str() {
            "if" => return Ok(self.make_token(TokenType::If, 2)),
            "goto" => return Ok(self.make_token(TokenType::Goto, 4)),
            "call" => return Ok(self.make_token(TokenType::Call, 4)),
            "ret" => return Ok(self.make_token(TokenType::Ret, 3)),
            "true" => return Ok(self.make_token(TokenType::Bool(true), 4)),
            "false" => return Ok(self.make_token(TokenType::Bool(false), 5)),
            _ => Ok(self.make_token(TokenType::Identifier(ident), ident_size.try_into().unwrap())),
        }
    }

    fn get_single(&mut self) -> TokenisationResult {
        let c = self.peek(0).unwrap();
        self.advance();

        match c {
            '+' => Ok(self.make_token(TokenType::Plus, 1)),
            '-' => Ok(self.make_token(TokenType::Minus, 1)),
            '*' => Ok(self.make_token(TokenType::Star, 1)),
            '/' => Ok(self.make_token(TokenType::Slash, 1)),
            '%' => Ok(self.make_token(TokenType::Percent, 1)),
            '=' => Ok(self.make_token(TokenType::Equals, 1)),
            '<' => Ok(self.make_token(TokenType::Less, 1)),
            '>' => Ok(self.make_token(TokenType::Greater, 1)),
            '@' => Ok(self.make_token(TokenType::At, 1)),
            '~' => Ok(self.make_token(TokenType::Tilde, 1)),
            ':' => Ok(self.make_token(TokenType::Colon, 1)),
            ';' => Ok(self.make_token(TokenType::EOS, 1)),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(self.make_token(TokenType::EOS, 1))
            }

            _ => Err(self.error("Unexpected character: ".to_string() + &c.to_string())),
        }
    }

    fn get_multi(&mut self) -> TokenisationResult {
        let c = self.peek(0).unwrap();
        let next = self.peek(1);

        if next.is_none() {
            return self.get_single();
        }

        let next_c = next.unwrap();

        match (c, next_c) {
            ('=', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::EqualsEquals, 2))
            }
            ('>', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::GreaterEquals, 2))
            }
            ('<', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::LessEquals, 2))
            }
            ('!', '=') => {
                self.advance();
                self.advance();
                Ok(self.make_token(TokenType::BangEquals, 2))
            }

            _ => self.get_single(),
        }
    }

    fn get_token(&mut self) -> TokenisationResult {
        self.skip_whitespace();

        if self.is_end() {
            return Ok(self.make_token(TokenType::EOF, 0));
        }

        let c = self.peek(0).unwrap();

        match c {
            '-' => {
                let c = self.peek(1);
                if c.is_some() && c.unwrap().is_ascii_digit() {
                    self.advance();
                    let mut token = self.get_number()?;
                    match token.token_type {
                        TokenType::Int(i) => token.token_type = TokenType::Int(-i),
                        TokenType::Float(f) => token.token_type = TokenType::Float(-f),
                        _ => {}
                    }
                    return Ok(token);
                }
                self.advance();
                Ok(self.make_token(TokenType::Minus, 1))
            }
            '+' | '*' | '/' | '%' | '!' | '=' | '<' | '>' | '@' | '~' | ':' | '\n' => {
                self.get_multi()
            }
            '0'..='9' => self.get_number(),
            'a'..='z' | 'A'..='Z' => self.get_ident(),
            '#' => {
                loop {
                    let c = self.advance();
                    if c.is_none() {
                        break;
                    }
                    if c.unwrap() == '\n' {
                        break;
                    }
                }
                self.get_token()
            }
            '"' => self.get_string(),
            _ => Err(self.error("Unexpected character: ".to_string() + &c.to_string())),
        }
    }
}
