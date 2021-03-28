use siderow::ssa;

#[test]
fn do_test() {
    let mut module = ssa::Module::new();

    let global_piyo = ssa::Global::new("piyo", ssa::Type::I32);
    let global_piyo = module.add_global(global_piyo);

    let func_fuga = module.add_function(func_fuga());
    module.add_function(func_hoge(&module, func_fuga, global_piyo));

    println!("{}", module.dump());
}

fn func_hoge(
    module: &ssa::Module,
    func_fuga: ssa::FunctionId,
    global_piyo: ssa::GlobalId,
) -> ssa::Function {
    let mut function = ssa::Function::new("hoge", ssa::Type::Void, vec![]);
    let mut builder = ssa::FunctionBuilder::new(&mut function);

    let entry_block = builder.add_block();
    builder.set_block(entry_block);

    let one = ssa::Value::Constant(ssa::Constant::I32(1));
    let one = builder.call(module, func_fuga, vec![one]);

    let mem = builder.alloc(ssa::Type::I32);
    builder.store(mem, one);
    let one = builder.load(mem);

    let two = builder.add(one, one);

    let block1 = builder.add_block();
    builder.br(block1);
    builder.set_block(block1);

    let piyo = ssa::Value::new_global(module, global_piyo);
    let three = builder.add(two, piyo);
    let cond = builder.eq(two, three);

    let block2 = builder.add_block();
    let block3 = builder.add_block();
    builder.cond_br(cond, block2, block3);

    builder.set_block(block2);
    builder.ret(cond);

    builder.set_block(block3);
    builder.ret(cond);

    function
}

fn func_fuga() -> ssa::Function {
    let mut function = ssa::Function::new("fuga", ssa::Type::I32, vec![ssa::Type::I32]);
    let mut builder = ssa::FunctionBuilder::new(&mut function);

    let entry_block = builder.add_block();
    builder.set_block(entry_block);

    let param = ssa::Value::new_param(builder.function(), 0);
    builder.ret(param);

    function
}
