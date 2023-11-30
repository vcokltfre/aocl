use std::fs;

use stdlib::{array, file, test};

mod errors;
mod frontend;
mod stdlib;
mod vm;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let data = fs::read_to_string(args[1].clone()).unwrap() + "\n";

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

    let mut vm = vm::VM::new(program.unwrap());

    array::register(&mut vm);
    file::register(&mut vm);
    test::register(&mut vm);

    if let Err(e) = vm.run() {
        println!("{}", e);
        return;
    }
}
