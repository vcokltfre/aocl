use crate::vm::{VMValue, VM};

pub fn std_map(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 3 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let module = match args[0].clone() {
        VMValue::String(module) => module,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let function = match args[1].clone() {
        VMValue::String(function) => function,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    let values = match &args[2] {
        VMValue::Array(values) => values,
        _ => return Err(format!("expected array, got {}", args[2].name())),
    };

    let mut new_array = Vec::new();

    for value in values {
        let mut idts = idts.clone();
        idts.push(Some("value".to_string()));

        let result = vm.call(module.clone(), function.clone(), idts, vec![value.clone()]);

        match result {
            Ok(Some(result)) => new_array.push(result),
            Ok(None) => return Err(format!("expected return value, got None")),
            Err(err) => return Err(err),
        }
    }

    Ok(Some(VMValue::Array(new_array)))
}

// same as map but doesn't care about None values and returns None
pub fn std_mapdrop(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 3 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let module = match args[0].clone() {
        VMValue::String(module) => module,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let function = match args[1].clone() {
        VMValue::String(function) => function,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    let values = match &args[2] {
        VMValue::Array(values) => values,
        _ => return Err(format!("expected array, got {}", args[2].name())),
    };

    for value in values {
        let mut idts = idts.clone();
        idts.push(Some("value".to_string()));

        let result = vm.call(module.clone(), function.clone(), idts, vec![value.clone()]);

        match result {
            Err(err) => return Err(err),
            _ => (),
        }
    }

    Ok(None)
}

pub fn std_filter(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 3 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let module = match args[0].clone() {
        VMValue::String(module) => module,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let function = match args[1].clone() {
        VMValue::String(function) => function,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    let values = match &args[2] {
        VMValue::Array(values) => values,
        _ => return Err(format!("expected array, got {}", args[2].name())),
    };

    let mut new_array = Vec::new();

    for value in values {
        let mut idts = idts.clone();
        idts.push(Some("value".to_string()));

        let result = vm.call(module.clone(), function.clone(), idts, vec![value.clone()]);

        match result {
            Ok(Some(result)) => {
                let boolean = match result {
                    VMValue::Bool(boolean) => boolean,
                    _ => return Err(format!("expected boolean, got {}", result.name())),
                };

                if boolean {
                    new_array.push(value.clone());
                }
            }
            Ok(None) => return Err(format!("expected return value, got None")),
            Err(err) => return Err(err),
        }
    }

    Ok(Some(VMValue::Array(new_array)))
}

pub fn std_getargs(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    _args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    let args: Vec<VMValue> = std::env::args().map(VMValue::String).collect();

    Ok(Some(VMValue::Array(args[1..].to_vec())))
}

pub fn std_getenv(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let key = match args[0].clone() {
        VMValue::String(key) => key,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let value = match std::env::var(key) {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };

    Ok(Some(VMValue::String(value)))
}

pub fn std_setenv(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let key = match args[0].clone() {
        VMValue::String(key) => key,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let value = match args[1].clone() {
        VMValue::String(value) => value,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    std::env::set_var(key, value);

    Ok(None)
}

pub fn std_exit(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() > 1 {
        return Err(format!("expected 0 or 1 arguments, got {}", args.len()));
    }

    let code = match args.len() {
        0 => 0,
        1 => match args[0].clone() {
            VMValue::Int(code) => code as i32,
            _ => return Err(format!("expected int, got {}", args[0].name())),
        },
        _ => unreachable!(),
    };

    std::process::exit(code);
}

pub fn register(vm: &mut VM) {
    vm.register("std".to_string(), "map".to_string(), std_map);
    vm.register("std".to_string(), "mapdrop".to_string(), std_mapdrop);
    vm.register("std".to_string(), "filter".to_string(), std_filter);
    vm.register("std".to_string(), "getargs".to_string(), std_getargs);
    vm.register("std".to_string(), "getenv".to_string(), std_getenv);
    vm.register("std".to_string(), "setenv".to_string(), std_setenv);
    vm.register("std".to_string(), "exit".to_string(), std_exit);
}
