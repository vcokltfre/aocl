use crate::vm::{VMValue, VM};

pub fn test_is(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let left = &args[0];
    let right = &args[1];

    let result = match left.equals(right) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    if let VMValue::Bool(result) = result {
        if !result {
            return Err(format!(
                "expected {} to be {}, got {}",
                left.name(),
                right.name(),
                result
            ));
        }
    } else {
        return Err(format!("expected bool, got {}", result.name()));
    }

    Ok(None)
}

pub fn register(vm: &mut VM) {
    vm.register("test".to_string(), "is".to_string(), test_is);
}
