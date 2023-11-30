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

pub fn register(vm: &mut VM) {
    vm.register("std".to_string(), "map".to_string(), std_map);
    vm.register("std".to_string(), "mapdrop".to_string(), std_mapdrop);
}