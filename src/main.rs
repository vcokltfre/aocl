use std::fs;

mod errors;
mod frontend;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let data = fs::read_to_string(args[1].clone()).unwrap();

    let mut lexer = frontend::lexer::Lexer::new(args[1].clone(), data);
    let tokens = lexer.tokenise();

    if let Err(e) = tokens {
        println!("{}", e);
        return;
    }

    let mut parser = frontend::parser::Parser::new(tokens.unwrap());
    let program = parser.parse();

    if let Err(e) = program {
        println!("{}", e);
        return;
    }

    for statement in program.unwrap() {
        println!("{}", statement.rewrite());
    }
}
