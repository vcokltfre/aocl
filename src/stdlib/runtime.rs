use crate::vm::{VMValue, VM};

pub fn runtime_goto(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let label = match args[0].clone() {
        VMValue::String(label) => label,
        _ => return Err(format!("expected string, got {}", args[0].name())),
    };

    match vm.gotos.get(&label) {
        Some(index) => {
            vm.index = *index;
            Ok(None)
        }
        None => Err(format!("label {} not found", label)),
    }
}

pub fn runtime_breakpoint(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    _args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    vm.breakpoint = true;

    Ok(None)
}

pub fn register(vm: &mut VM) {
    vm.register("runtime".to_string(), "gotolabel".to_string(), runtime_goto);
    vm.register(
        "runtime".to_string(),
        "breakpoint".to_string(),
        runtime_breakpoint,
    );
}
