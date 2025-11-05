use crate::frontend::parser::{BinOp, Compare, Statement, StatementContext, Value};

fn vvm_emit_load_value(v: &Value) -> String {
    match v {
        Value::Int(i) => format!("PUSHI {}", i),
        Value::Float(f) => format!("PUSHF {}", f),
        Value::Identifier(name) => format!("LOAD_IMM {}", name),
        Value::Bool(b) => format!("PUSHB {}", b),
        Value::String(s) => format!("PUSHS {}", s.replace("\n", "\\n")),
    }
}

fn vvm_emit_binop(op: &BinOp) -> Vec<String> {
    let mut lines = Vec::new();

    match op {
        BinOp::Add(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("ADD".into());
            lines
        }
        BinOp::Sub(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("SUB".into());
            lines
        }
        BinOp::Mul(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("MUL".into());
            lines
        }
        BinOp::Div(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("DIV".into());
            lines
        }
        BinOp::Mod(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("MOD".into());
            lines
        }
    }
}

fn vvm_emit_compare(cmp: &Compare) -> Vec<String> {
    let mut lines = Vec::new();

    match cmp {
        Compare::Equals(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("CMPEQ".into());
            lines
        }
        Compare::NotEquals(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("CMPNE".into());
            lines
        }
        Compare::LessThan(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("CMPLT".into());
            lines
        }
        Compare::GreaterThan(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("CMPGT".into());
            lines
        }
        Compare::LessThanEquals(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("CMPLE".into());
            lines
        }
        Compare::GreaterThanEquals(lhs, rhs) => {
            lines.push(vvm_emit_load_value(lhs));
            lines.push(vvm_emit_load_value(rhs));
            lines.push("CMPGE".into());
            lines
        }
    }
}

pub fn vvm_emit(stmts: &Vec<Statement>) -> String {
    let mut lines = Vec::new();

    for stmt in stmts {
        match &stmt.context {
            StatementContext::AssignBinOp(name, op) => {
                let mut ls = vvm_emit_binop(op);
                lines.append(&mut ls);
                lines.push(format!("STORE_IMM {}", name));
            }
            StatementContext::AssignLiteral(name, value) => {
                lines.push(vvm_emit_load_value(value));
                lines.push(format!("STORE_IMM {}", name));
            }
            StatementContext::AssignCall(name, target, args) => {
                for arg in args {
                    lines.push(vvm_emit_load_value(arg));
                }
                lines.push(format!("CALLNATIVE {}:{}", target.module, target.function));
                lines.push(format!("STORE_IMM {}", name));
            }
            StatementContext::Call(target, args) => {
                if target.module == "array" && target.function == "push" {
                    let name = match &args[0] {
                        Value::Identifier(n) => n,
                        _ => panic!("Expected identifier as first argument to array:push"),
                    };
                    for arg in &args[1..] {
                        lines.push(vvm_emit_load_value(arg));
                    }
                    lines.push(format!("LOAD_IMM {}", name));
                    lines.push(format!("CALLNATIVE {}:{}", target.module, target.function));
                    lines.push(format!("STORE_IMM {}", name));
                    continue;
                }
                for arg in args {
                    lines.push(vvm_emit_load_value(arg));
                }
                lines.push(format!("CALLNATIVE {}:{}", target.module, target.function));
            }
            StatementContext::Goto(name) => {
                lines.push(format!("JMP {}", name));
            }
            StatementContext::GotoIf(name, cond) => {
                let mut ls = vvm_emit_compare(cond);
                lines.append(&mut ls);
                lines.push(format!("JMPIF {}", name));
            }
            StatementContext::GotoDef(name) => {
                lines.push(format!("LABEL {}", name));
            }
            StatementContext::Ret => {
                lines.push(format!("RET"));
            }
            _ => {
                lines.push(format!("// Unhandled statement: {:?}", stmt));
            }
        }
    }

    lines.join("\n")
}

pub fn vvm_run(stmts: &Vec<Statement>) {
    let vvm_code = vvm_emit(stmts);
    let program = vvm::Program::from_source(&vvm_code).unwrap();
    let mut vm = vvm::VM::new(program);

    vm.register_native_handler("file:read", |vm| {
        let filename = vm.pop();
        match filename {
            vvm::Value::String(s) => match std::fs::read_to_string(&s) {
                Ok(content) => vm.push(vvm::Value::String(content)),
                Err(_) => {
                    eprintln!("Error: Failed to read file '{}'", s);
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!("Error: file:read expects a string argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("string:split", |vm| {
        let delimiter = vm.pop();
        let string = vm.pop();
        match (string, delimiter) {
            (vvm::Value::String(s), vvm::Value::String(d)) => {
                let parts: Vec<vvm::Value> = s
                    .split(&d)
                    .map(|part| vvm::Value::String(part.to_string()))
                    .collect();
                vm.push(vvm::Value::Array(parts));
            }
            _ => {
                eprintln!("Error: string:split expects two string arguments");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("array:len", |vm| {
        let arr = vm.pop();
        match arr {
            vvm::Value::Array(items) => {
                vm.push(vvm::Value::Int(items.len() as i64));
            }
            _ => {
                eprintln!("Error: array:len expects an array argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("array:new", |vm| {
        vm.push(vvm::Value::Array(Vec::new()));
    });

    vm.register_native_handler("array:get", |vm| {
        let index = vm.pop();
        let arr = vm.pop();
        match (arr, index) {
            (vvm::Value::Array(items), vvm::Value::Int(i)) => {
                if i < 0 || (i as usize) >= items.len() {
                    eprintln!("Error: array:get index out of bounds");
                    std::process::exit(1);
                }
                vm.push(items[i as usize].clone());
            }
            _ => {
                eprintln!("Error: array:get expects an array and an integer index");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("string:notempty", |vm| {
        let s = vm.pop();
        match s {
            vvm::Value::String(str_val) => {
                vm.push(vvm::Value::Bool(!str_val.is_empty()));
            }
            _ => {
                eprintln!("Error: string:notempty expects a string argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("convert:atoi", |vm| {
        let s = vm.pop();
        match s {
            vvm::Value::String(str_val) => match str_val.parse::<i64>() {
                Ok(num) => vm.push(vvm::Value::Int(num)),
                Err(_) => {
                    eprintln!("Error: convert:atoi failed to parse integer");
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!("Error: convert:atoi expects a string argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("std:filter", |vm| {
        let arr = vm.pop();
        let func = vm.pop();
        let module = vm.pop();
        match (module, func, arr) {
            (vvm::Value::String(m), vvm::Value::String(f), vvm::Value::Array(items)) => {
                let mut result = Vec::new();
                for item in items {
                    vm.push(item.clone());
                    vm.call_native(format!("{}:{}", m, f).as_str());
                    let ret = vm.pop();
                    match ret {
                        vvm::Value::Bool(true) => result.push(item),
                        vvm::Value::Bool(false) => (),
                        _ => {
                            eprintln!("Error: std:filter function must return a boolean");
                            std::process::exit(1);
                        }
                    }
                }

                vm.push(vvm::Value::Array(result));
            }
            _ => {
                eprintln!("Error: std:filter expects module name, function name, and array");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("std:map", |vm| {
        let arr = vm.pop();
        let func = vm.pop();
        let module = vm.pop();
        match (module, func, arr) {
            (vvm::Value::String(m), vvm::Value::String(f), vvm::Value::Array(items)) => {
                let mut result = Vec::new();
                for item in items {
                    vm.push(item);
                    vm.call_native(format!("{}:{}", m, f).as_str());
                    let ret = vm.pop();
                    result.push(ret);
                }
                vm.push(vvm::Value::Array(result));
            }
            _ => {
                eprintln!("Error: std:map expects module name, function name, and array");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("math:sum", |vm| {
        let arr = vm.pop();
        match arr {
            vvm::Value::Array(items) => {
                let mut sum = 0i64;
                for item in items {
                    match item {
                        vvm::Value::Int(n) => sum += n,
                        _ => {
                            eprintln!("Error: math:sum expects an array of integers");
                            std::process::exit(1);
                        }
                    }
                }
                vm.push(vvm::Value::Int(sum));
            }
            _ => {
                eprintln!("Error: math:sum expects an array argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("array:push", |vm| {
        let arr = vm.pop();
        let value = vm.pop();
        match arr {
            vvm::Value::Array(mut items) => {
                items.push(value);
                vm.push(vvm::Value::Array(items));
            }
            _ => {
                eprintln!("Error: array:push expects an array argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("math:max", |vm| {
        let arr = vm.pop();
        match arr {
            vvm::Value::Array(items) => {
                if items.is_empty() {
                    eprintln!("Error: math:max expects a non-empty array");
                    std::process::exit(1);
                }
                let mut max_val = match &items[0] {
                    vvm::Value::Int(n) => *n,
                    _ => {
                        eprintln!("Error: math:max expects an array of integers");
                        std::process::exit(1);
                    }
                };
                for item in &items[1..] {
                    match item {
                        vvm::Value::Int(n) => {
                            if *n > max_val {
                                max_val = *n;
                            }
                        }
                        _ => {
                            eprintln!("Error: math:max expects an array of integers");
                            std::process::exit(1);
                        }
                    }
                }
                vm.push(vvm::Value::Int(max_val));
            }
            _ => {
                eprintln!("Error: math:max expects an array argument");
                std::process::exit(1);
            }
        }
    });

    vm.register_native_handler("io:println", |vm| {
        let value = vm.pop();
        match value {
            vvm::Value::Int(n) => println!("{}", n),
            vvm::Value::UInt(u) => println!("{}", u),
            vvm::Value::Float(f) => println!("{}", f),
            vvm::Value::Bool(b) => println!("{}", b),
            vvm::Value::String(s) => println!("{}", s),
            vvm::Value::Array(arr) => {
                let strs: Vec<String> = arr
                    .into_iter()
                    .map(|v| match v {
                        vvm::Value::Int(n) => n.to_string(),
                        vvm::Value::UInt(u) => u.to_string(),
                        vvm::Value::Float(f) => f.to_string(),
                        vvm::Value::Bool(b) => b.to_string(),
                        vvm::Value::String(s) => s,
                        vvm::Value::Array(_) => "[Array]".to_string(),
                    })
                    .collect();
                println!("[{}]", strs.join(", "));
            }
        }
    });

    vm.run();
}
