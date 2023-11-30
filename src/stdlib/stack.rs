use crate::vm::{VMValue, VM};

pub fn stack_push(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    vm.stack.push(args[0].clone());

    Ok(None)
}

pub fn stack_pop(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    _args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    match vm.stack.pop() {
        Some(value) => Ok(Some(value)),
        None => Err("stack is empty".to_string()),
    }
}

pub fn stack_len(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    _args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    Ok(Some(VMValue::Int(vm.stack.len() as i64)))
}

pub fn register(vm: &mut VM) {
    vm.register("stack".to_string(), "push".to_string(), stack_push);
    vm.register("stack".to_string(), "pop".to_string(), stack_pop);
    vm.register("stack".to_string(), "len".to_string(), stack_len);
}
