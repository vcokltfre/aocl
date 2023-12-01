use std::io::BufRead;

use super::VM;

pub fn debugger(vm: &mut VM) {
    let stdin = std::io::stdin();

    println!("debugger started");

    for line in stdin.lock().lines() {
        let command = line.unwrap();
        let args = command.split(" ").collect::<Vec<&str>>();

        if args.len() == 0 {
            continue;
        }

        match args[0] {
            "var" => {
                if args.len() != 2 {
                    println!("expected 1 argument, got {}", args.len() - 1);
                    continue;
                }

                let identifier = args[1];

                if let Some(value) = vm.variables.get(identifier) {
                    println!("{} = {}", identifier, value);
                } else {
                    println!("variable not found: {}", identifier);
                }
            }
            "vars" => {
                if args.len() == 1 {
                    for (identifier, value) in &vm.variables {
                        println!("{} = {}", identifier, value);
                    }
                } else if args.len() == 2 {
                    let query = args[1];

                    for (identifier, value) in &vm.variables {
                        if !identifier.contains(query) {
                            continue;
                        }
                        println!("{} = {}", identifier, value);
                    }
                } else {
                    println!("expected 1 or 2 arguments, got {}", args.len() - 1);
                }
            }
            "goto" => {
                if args.len() != 2 {
                    println!("expected 1 argument, got {}", args.len() - 1);
                    continue;
                }

                let label = args[1];

                match vm.gotos.get(label) {
                    Some(index) => {
                        vm.index = *index;
                    }
                    None => {
                        println!("label not found: {}", label);
                    }
                }
            }
            "labels" => {
                if args.len() == 1 {
                    for (label, index) in &vm.gotos {
                        println!("{} = {}", label, index);
                    }
                } else if args.len() == 2 {
                    let query = args[1];

                    for (label, index) in &vm.gotos {
                        if !label.contains(query) {
                            continue;
                        }
                        println!("{} = {}", label, index);
                    }
                } else {
                    println!("expected 1 or 2 arguments, got {}", args.len() - 1);
                }
            }
            "continue" => {
                break;
            }
            _ => {
                println!("unknown command: {}", args[0]);
            }
        }
    }
}
