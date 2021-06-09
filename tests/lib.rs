use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

use siderow::{
    arch::x86::{self, asm, regalloc},
    ssa::{self, parser},
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

    run(module, 4).unwrap();
}

#[test]
fn gep_1() {
    let mut module = ssa::Module::new();
    let mut func_main = ssa::Function::new(&module, "main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);
    let block_0 = builder.new_block();
    builder.set_block(block_0);

    let array_type = module.types.borrow_mut().array_of(ssa::Type::I32, 4);
    let v0 = builder.alloc(array_type);
    let v1 = builder.gep(v0, vec![ssa::Value::new_i32(0), ssa::Value::new_i32(1)]);
    let v2 = builder.gep(v0, vec![ssa::Value::new_i32(0), ssa::Value::new_i32(2)]);
    builder.store(v1, ssa::Value::new_i32(42));
    builder.store(v2, ssa::Value::new_i32(21));
    let v3 = builder.load(v1);
    let v4 = builder.load(v2);
    let v5 = builder.add(v3, v4);
    builder.ret(v5);

    module.add_function(func_main);
    println!("{}", module.dump());

    run(module, 63).unwrap();
}
#[test]
fn gep_2() {
    let mut module = ssa::Module::new();
    let mut func_main = ssa::Function::new(&module, "main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);
    let block_0 = builder.new_block();
    builder.set_block(block_0);

    let array_type = module.types.borrow_mut().array_of(ssa::Type::I32, 4);
    let ptr_type = module.types.borrow_mut().ptr_to(ssa::Type::I32);
    let v0 = builder.alloc(array_type);
    let v1 = builder.alloc(ptr_type);
    builder.store(v1, v0);

    let v2 = builder.load(v1);
    let v3 = builder.gep(v2, vec![ssa::Value::new_i32(1)]);
    let v4 = builder.gep(v2, vec![ssa::Value::new_i32(2)]);
    builder.store(v3, ssa::Value::new_i32(42));
    builder.store(v4, ssa::Value::new_i32(21));
    let v5 = builder.load(v3);
    let v6 = builder.load(v4);
    let v7 = builder.add(v5, v6);
    builder.ret(v7);

    module.add_function(func_main);
    println!("{}", module.dump());

    run(module, 63).unwrap();
}

#[test]
fn gep_3() {
    let mut module = ssa::Module::new();
    let mut func_main = ssa::Function::new(&module, "main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);
    let block_0 = builder.new_block();
    builder.set_block(block_0);

    let arr1 = module.types.borrow_mut().array_of(ssa::Type::I32, 4);
    let arr2 = module.types.borrow_mut().array_of(arr1, 4);
    let v0 = builder.alloc(arr2);

    let zero = ssa::Value::new_i32(0);
    for i in 0..3 {
        for j in 0..3 {
            let v1 = builder.gep(
                v0,
                vec![zero, ssa::Value::new_i32(i), ssa::Value::new_i32(j)],
            );
            builder.store(v1, ssa::Value::new_i32(i * 3 + j));
        }
    }

    let v2 = builder.gep(
        v0,
        vec![zero, ssa::Value::new_i32(1), ssa::Value::new_i32(2)],
    );
    let v3 = builder.load(v2);
    builder.ret(v3);

    module.add_function(func_main);
    println!("{}", module.dump());

    run(module, 5).unwrap();
}

#[test]
fn byte_1() {
    let mut module = ssa::Module::new();
    let mut func_main = ssa::Function::new(&module, "main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);
    let block_0 = builder.new_block();
    builder.set_block(block_0);

    let array_typ = module.types.borrow_mut().array_of(ssa::Type::I1, 4);
    let v0 = builder.alloc(array_typ);
    let v1 = builder.gep(v0, vec![ssa::Value::new_i32(0), ssa::Value::new_i32(1)]);
    builder.store(v1, ssa::Value::new_i1(true));
    let v2 = builder.load(v1);
    builder.ret(v2);

    module.add_function(func_main);
    println!("{}", module.dump());

    run(module, 1).unwrap();
}

#[test]
fn zero_1() {
    let mut module = ssa::Module::new();
    let mut func_main = ssa::Function::new(&module, "main", ssa::Type::I32, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut func_main);
    let block_0 = builder.new_block();
    builder.set_block(block_0);

    let v0 = builder.alloc(ssa::Type::I1);
    let v1 = builder.alloc(ssa::Type::I8);
    let v2 = builder.alloc(ssa::Type::I32);
    let array_typ = module.types.borrow_mut().array_of(ssa::Type::I8, 4);
    let v3 = builder.alloc(array_typ);

    builder.store(v0, ssa::Value::new_zero());
    builder.store(v1, ssa::Value::new_zero());
    builder.store(v2, ssa::Value::new_zero());
    builder.store(v3, ssa::Value::new_zero());

    builder.ret(v2);

    module.add_function(func_main);
    println!("{}", module.dump());

    run(module, 1).unwrap();
}

fn run(module: ssa::Module, expected: i32) -> io::Result<()> {
    let mut assembly = x86::instsel::translate(module);
    regalloc::allocate(&mut assembly);

    let mut file = File::create("./tmp.s")?;
    file.write_all(assembly.stringify().as_bytes())?;

    let status = Command::new("gcc")
        .arg("./tmp.s")
        .arg("-o")
        .arg("./tmp")
        .status()?;

    if !status.success() {
        panic!("failed to compile");
    }

    let status = Command::new("./tmp").status()?;
    let status_code = status.code().unwrap();

    assert_eq!(status_code, expected);

    Ok(())
}

#[test]
fn test_all() {
    let path = "tests/testcases/";
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }

        test_file(&entry.path()).unwrap();
    }
}

fn test_file(path: &Path) -> io::Result<()> {
    println!("Processing: {:?}", path);
    let input = fs::read_to_string(path)?;

    let mut input_lines = input.lines();
    let first_line = input_lines.next().unwrap();
    let program = input_lines.collect::<Vec<&str>>().join("\n");

    let module = parser::parse(&program);

    let expected = first_line.strip_prefix("// ").unwrap().parse().unwrap();
    run(module, expected).unwrap();

    Ok(())
}
