use crate::vm::{VMValue, VM};

pub fn io_print(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    print!("{}", args[0]);

    Ok(None)
}

pub fn io_println(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    println!("{}", args[0]);

    Ok(None)
}

pub fn io_sprint(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }

    Ok(Some(VMValue::String(args[0].to_string())))
}

pub fn io_read(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    _args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    Ok(Some(VMValue::String(input)))
}

pub fn io_readln(
    _vm: &mut VM,
    _idts: Vec<Option<String>>,
    _args: Vec<VMValue>,
) -> Result<Option<VMValue>, String> {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    Ok(Some(VMValue::String(input)))
}

pub fn register(vm: &mut VM) {
    vm.register("io".to_string(), "print".to_string(), io_print);
    vm.register("io".to_string(), "println".to_string(), io_println);
    vm.register("io".to_string(), "sprint".to_string(), io_sprint);
    vm.register("io".to_string(), "read".to_string(), io_read);
    vm.register("io".to_string(), "readln".to_string(), io_readln);
}
