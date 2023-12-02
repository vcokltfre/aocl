use crate::vm::{VMValue, VM};

// @iter:iterate "name" "varname" start end
pub fn iter_iterate(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 3 {
        return Err(format!("expected 3 arguments, got {}", args.len()));
    }

    let varname = match args[0].clone() {
        VMValue::String(varname) => varname,
        _ => return Err(format!("expected string, got {}", args[1])),
    };

    let start = match args[1] {
        VMValue::Int(start) => start,
        _ => return Err(format!("expected int, got {}", args[2])),
    };

    let mut end = match args[2] {
        VMValue::Int(end) => end,
        _ => return Err(format!("expected int, got {}", args[3])),
    };

    end -= 1;

    if start > end {
        return Err(format!("start must be less than end"));
    }

    vm.variables.insert(varname.clone(), VMValue::Int(start));
    vm.variables.insert(
        format!("@internal:iter:end:{}", varname.clone()),
        VMValue::Int(end),
    );

    vm.gotos
        .insert(format!("@internal:iter:{}", varname), vm.index);

    Ok(None)
}

pub fn iter_end(
    vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let varname = match args[0].clone() {
        VMValue::String(iter) => iter,
        _ => return Err(format!("expected string, got {}", args[0])),
    };

    let current = match vm.variables.get(&varname.clone()) {
        Some(VMValue::Int(current)) => current,
        _ => return Err("invalid iter for current".to_string()),
    };

    let end = match vm.variables.get(&format!("@internal:iter:end:{}", varname)) {
        Some(VMValue::Int(end)) => end,
        _ => return Err("invalid iter for end".to_string()),
    };

    if current >= end {
        vm.gotos.remove(&format!("@internal:iter:{}", varname));
        vm.variables
            .remove(&format!("@internal:iter:end:{}", varname));
        vm.variables.remove(&varname);
        return Ok(None);
    } else {
        vm.variables
            .insert(varname.clone(), VMValue::Int(current + 1));
        vm.index = vm.gotos[&format!("@internal:iter:{}", varname)];
    }

    Ok(None)
}

pub fn register(vm: &mut VM) {
    vm.register("iter".to_string(), "iterate".to_string(), iter_iterate);
    vm.register("iter".to_string(), "end".to_string(), iter_end);
}
