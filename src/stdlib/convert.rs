use crate::vm::{VMValue, VM};

pub fn convert_atoi(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let string = match &args[0] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let int = match string.parse::<i64>() {
        Ok(int) => int,
        Err(err) => return Err(format!("failed to parse int: {}", err)),
    };

    Ok(Some(VMValue::Int(int)))
}

pub fn convert_itoa(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let int = match &args[0] {
        VMValue::Int(int) => int,
        _ => return Err(format!("expected int, got {}", args[0].name())),
    };

    Ok(Some(VMValue::String(int.to_string())))
}

pub fn convert_atof(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let string = match &args[0] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let float = match string.parse::<f64>() {
        Ok(float) => float,
        Err(err) => return Err(format!("failed to parse float: {}", err)),
    };

    Ok(Some(VMValue::Float(float)))
}

pub fn convert_ftoa(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let float = match &args[0] {
        VMValue::Float(float) => float,
        _ => return Err(format!("expected float, got {}", args[0].name())),
    };

    Ok(Some(VMValue::String(float.to_string())))
}

pub fn register(vm: &mut VM) {
    vm.register("convert".to_string(), "atoi".to_string(), convert_atoi);
    vm.register("convert".to_string(), "itoa".to_string(), convert_itoa);
    vm.register("convert".to_string(), "atof".to_string(), convert_atof);
    vm.register("convert".to_string(), "ftoa".to_string(), convert_ftoa);
}
