use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

use siderow::{
    arch::x86::{self, asm, regalloc},
    ssa,
};

#[test]
fn do_test() {
    let mut module = ssa::Module::new();

    // ---

    let mut func_main = ssa::Function::new(&module, "main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);
    let block_0 = builder.new_block();
    let block_1 = builder.new_block();

    builder.set_block(block_0);
    let one = ssa::Value::new_i32(1);
    let v0 = builder.add(one, one);
    let v1 = builder.add(v0, v0);
    let _v2 = builder.add(v0, v0);
    builder.br(block_1);

    builder.set_block(block_1);
    let _v3 = builder.add(v0, v0);
    builder.ret(v1);

    module.add_function(func_main);

    // ---

    println!("{}", module.dump());
    ssa::pass::cf::apply(&mut module);
    ssa::pass::dce::apply(&mut module);
    println!("{}", module.dump());

    // ---

    let mut assembly = x86::instsel::translate(module);
    regalloc::allocate(&mut assembly);
    println!("{}", assembly.stringify());

    // ---

    run(assembly, 4).unwrap();
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
