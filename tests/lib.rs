use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

use siderow::{
    arch::x86::{self, asm},
    ssa,
};

#[test]
fn do_test() {
    let mut module = ssa::Module::new();

    // ---

    let mut func_main = ssa::Function::new("main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);

    let entry_block = builder.add_block();
    builder.set_block(entry_block);

    let one = ssa::Value::new_i32(1);
    let two = builder.add(one, one);
    builder.ret(two);

    module.add_function(func_main);

    // ---

    println!("{}", module.dump());
    let assembly = x86::instsel::translate(module);
    println!("{}", assembly.stringify());

    // ---
    run(assembly, 1).unwrap();
}

fn run(assembly: asm::Assembly, expected: i32) -> io::Result<()> {
    let mut file = File::create("./tmp.s")?;
    file.write_all(assembly.stringify().as_bytes())?;

    let status = Command::new("gcc")
        .arg("./tmp.s")
        .arg("-o")
        .arg("./tmp")
        .status()?;

    if !status.success() {
        println!("==> failed to compile");
        panic!();
    }

    println!("==> finished compiling successfully");

    let status = Command::new("./tmp").status()?;
    let status_code = status.code().unwrap();
    println!("==> exited with {}, expected {}", status_code, expected);

    assert_eq!(status_code, expected);

    Ok(())
}
