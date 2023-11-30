use crate::vm::{VMValue, VM};

pub fn file_read(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let path = match &args[0] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => return Err(format!("failed to read file: {}", err)),
    };

    Ok(Some(VMValue::String(contents)))
}

pub fn file_write(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let path = match &args[0] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    let contents = match &args[1] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[1].name())),
    };

    match std::fs::write(path, contents) {
        Ok(_) => Ok(None),
        Err(err) => Err(format!("failed to write file: {}", err)),
    }
}

pub fn file_exists(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let path = match &args[0] {
        VMValue::String(string) => string,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    Ok(Some(VMValue::Bool(std::path::Path::new(path).exists())))
}

pub fn register(vm: &mut VM) {
    vm.register("file".to_string(), "read".to_string(), file_read);
    vm.register("file".to_string(), "write".to_string(), file_write);
    vm.register("file".to_string(), "exists".to_string(), file_exists);
}
