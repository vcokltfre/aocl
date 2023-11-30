use crate::vm::{VMValue, VM};

pub fn math_sum(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let mut sum = 0;

    for part in array {
        let int = match part {
            VMValue::Int(int) => int,
            _ => return Err(format!("expected int in array, got {}", part.name())),
        };

        sum += int;
    }

    Ok(Some(VMValue::Int(sum)))
}

pub fn register(vm: &mut VM) {
    vm.register("math".to_string(), "sum".to_string(), math_sum);
}
