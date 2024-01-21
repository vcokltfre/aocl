use std::fmt;

trait Colorable {
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn blue(&self) -> String;
    fn cyan(&self) -> String;
    fn bold(&self) -> String;
}

impl Colorable for String {
    fn red(&self) -> String {
        format!("{}{}{}", "\x1b[31m", self, "\x1b[0m")
    }

    fn green(&self) -> String {
        format!("{}{}{}", "\x1b[32m", self, "\x1b[0m")
    }

    fn blue(&self) -> String {
        format!("{}{}{}", "\x1b[34m", self, "\x1b[0m")
    }

    fn cyan(&self) -> String {
        format!("{}{}{}", "\x1b[36m", self, "\x1b[0m")
    }

    fn bold(&self) -> String {
        format!("{}{}{}", "\x1b[1m", self, "\x1b[0m")
    }
}

impl Colorable for &str {
    fn red(&self) -> String {
        format!("{}{}{}", "\x1b[31m", self, "\x1b[0m")
    }

    fn green(&self) -> String {
        format!("{}{}{}", "\x1b[32m", self, "\x1b[0m")
    }

    fn blue(&self) -> String {
        format!("{}{}{}", "\x1b[34m", self, "\x1b[0m")
    }

    fn cyan(&self) -> String {
        format!("{}{}{}", "\x1b[36m", self, "\x1b[0m")
    }

    fn bold(&self) -> String {
        format!("{}{}{}", "\x1b[1m", self, "\x1b[0m")
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorLocation {
    Lexer,
    Parser,
    Interpreter,
}

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub index: usize,
    pub width: usize,

    pub file: String,
    pub message: String,
    pub context: String,
    pub location: ErrorLocation,
}

impl Error {
    pub fn new(
        line: usize,
        column: usize,
        index: usize,
        width: usize,
        file: String,
        message: String,
        context: String,
        location: ErrorLocation,
    ) -> Self {
        Self {
            line,
            column,
            index,
            width,
            file,
            message,
            context,
            location,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stage = match self.location {
            ErrorLocation::Lexer => "Lexing",
            ErrorLocation::Parser => "Parsing",
            ErrorLocation::Interpreter => "Runtime",
        };

        let mut col = self.column;
        if col == 0 {
            col = 1;
        }

        let sidebar_padding = " ".repeat(self.line.to_string().len());
        let arrow = "-->".blue().bold();
        let sidebar = "|".blue().bold();
        let padding = " ".repeat(col - 1);
        let bar = "^".repeat(self.width).blue().bold();

        write!(
            f,
            "{stage} error:\n{sidebar_padding}{arrow} {} {}:{} ({})\n{sidebar_padding} {sidebar}\n{} {sidebar} {}\n{sidebar_padding} {sidebar} {padding}{bar} {}",
            self.file.cyan(),
            self.line,
            self.column,
            self.index,
            self.line.to_string().blue().bold(),
            self.context.green(),
            self.message.red().bold(),
        )
    }
}
