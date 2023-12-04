use std::{cell::RefCell, rc::Rc};

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

    Ok(Some(VMValue::Array(Rc::new(RefCell::new(array)))))
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

    for (i, part) in array.borrow().iter().enumerate() {
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

pub fn string_contains(
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

    let substring = match &args[1] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    Ok(Some(VMValue::Bool(string.contains(substring))))
}

pub fn string_len(
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

    Ok(Some(VMValue::Int(string.len() as i64)))
}

pub fn string_toarray(
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

    let mut array = Vec::new();

    for c in string.chars() {
        array.push(VMValue::String(c.to_string()));
    }

    Ok(Some(VMValue::Array(Rc::new(RefCell::new(array)))))
}

pub fn register(vm: &mut VM) {
    vm.register("string".to_string(), "split".to_string(), string_split);
    vm.register("string".to_string(), "join".to_string(), string_join);
    vm.register(
        "string".to_string(),
        "notempty".to_string(),
        string_notempty,
    );
    vm.register(
        "string".to_string(),
        "contains".to_string(),
        string_contains,
    );
    vm.register("string".to_string(), "len".to_string(), string_len);
    vm.register("string".to_string(), "toarray".to_string(), string_toarray);
}
