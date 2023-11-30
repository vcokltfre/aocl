use crate::vm::{VMValue, VM};

pub fn array_new(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    Ok(Some(VMValue::Array(args)))
}

pub fn array_get(
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

    let index = match &args[1] {
        VMValue::Int(int) => int,
        _ => return Err(format!("expected int, got {}", args[1].name())),
    };

    if *index < 0 || *index as usize >= array.len() {
        return Err(format!("index out of bounds: {}", index));
    }

    Ok(Some(array[*index as usize].clone()))
}

pub fn array_pop(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    if array.len() == 0 {
        return Err(format!("cannot pop from empty array"));
    }

    let new_array = array[..array.len() - 1].to_vec();

    vm.variables
        .insert(idts[0].clone().unwrap(), VMValue::Array(new_array));

    Ok(Some(array[array.len() - 1].clone()))
}

pub fn array_popat(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let index = match &args[1] {
        VMValue::Int(int) => int,
        _ => return Err(format!("expected int, got {}", args[1].name())),
    };

    if *index < 0 || *index as usize >= array.len() {
        return Err(format!("index out of bounds: {}", index));
    }

    let mut new_array = array.clone();
    new_array.remove(*index as usize);

    vm.variables
        .insert(idts[0].clone().unwrap(), VMValue::Array(new_array));

    Ok(Some(array[*index as usize].clone()))
}

pub fn array_push(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let value = args[1].clone();

    let mut new_array = array.clone();
    new_array.push(value);

    vm.variables
        .insert(idts[0].clone().unwrap(), VMValue::Array(new_array));

    Ok(None)
}

pub fn array_index(
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

    let value = args[1].clone();

    let mut index = -1;

    for (i, v) in array.iter().enumerate() {
        if v == &value {
            index = i as i64;
            break;
        }
    }

    Ok(Some(VMValue::Int(index)))
}

pub fn array_reverse(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let mut new_array = array.clone();
    new_array.reverse();

    vm.variables
        .insert(idts[0].clone().unwrap(), VMValue::Array(new_array));

    Ok(None)
}

pub fn array_sort(
    vm: &mut VM,
    idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    let array = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let mut new_array = array.clone();
    new_array.sort();

    vm.variables
        .insert(idts[0].clone().unwrap(), VMValue::Array(new_array));

    Ok(None)
}

pub fn array_len(
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

    Ok(Some(VMValue::Int(array.len() as i64)))
}

pub fn array_clone(
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

    Ok(Some(VMValue::Array(array.clone())))
}

pub fn array_is(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", args.len()));
    }

    let array1 = match &args[0] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[0].name())),
    };

    let array2 = match &args[1] {
        VMValue::Array(array) => array,
        _ => return Err(format!("expected array, got {}", args[1].name())),
    };

    Ok(Some(VMValue::Bool(array1 == array2)))
}

pub fn register(vm: &mut VM) {
    vm.register("array".to_string(), "new".to_string(), array_new);
    vm.register("array".to_string(), "get".to_string(), array_get);
    vm.register("array".to_string(), "pop".to_string(), array_pop);
    vm.register("array".to_string(), "popat".to_string(), array_popat);
    vm.register("array".to_string(), "push".to_string(), array_push);
    vm.register("array".to_string(), "index".to_string(), array_index);
    vm.register("array".to_string(), "reverse".to_string(), array_reverse);
    vm.register("array".to_string(), "sort".to_string(), array_sort);
    vm.register("array".to_string(), "len".to_string(), array_len);
    vm.register("array".to_string(), "clone".to_string(), array_clone);
    vm.register("array".to_string(), "is".to_string(), array_is);
}
