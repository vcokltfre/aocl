use std::fmt;

use colored::Colorize;

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

        let sidebar_padding = " ".repeat(self.line.to_string().len());
        let arrow = "-->".blue().bold();
        let sidebar = "|".blue().bold();
        let padding = " ".repeat(self.column - 1);
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
