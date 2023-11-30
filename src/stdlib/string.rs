use crate::vm::{VMValue, VM};

pub fn string_split(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let string = match &args[0] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let separator = match &args[1] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    let mut array = Vec::new();

    for part in string.split(separator) {
        array.push(VMValue::String(part.to_string()));
    }

    Ok(Some(VMValue::Array(array)))
}

pub fn string_join(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let separator = match &args[1] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    let mut string = String::new();

    for (i, part) in array.iter().enumerate() {
        if i != 0 {
            string += separator;
        }

        let str = match part {
            VMValue::String(s) => s,
            _ => return Err(format!("expected string in array, got {}", part.name())),
        };

        string += str;
    }

    Ok(Some(VMValue::String(string)))
}

pub fn string_notempty(
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

    Ok(Some(VMValue::Bool(!string.is_empty())))
}

pub fn register(vm: &mut VM) {
    vm.register("string".to_string(), "split".to_string(), string_split);
    vm.register("string".to_string(), "join".to_string(), string_join);
    vm.register(
        "string".to_string(),
        "notempty".to_string(),
        string_notempty,
    );
}
