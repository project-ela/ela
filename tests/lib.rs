use siderow::ssa;

#[test]
fn do_test() {
    let mut function = ssa::Function::new("hoge");
    let mut builder = ssa::FunctionBuilder::new(&mut function);

    let entry_block = builder.add_block();
    builder.set_block(entry_block);

    let one = ssa::Value::Immediate(ssa::Immediate::I32(1));

    let mem = builder.alloc(ssa::Type::I32);
    builder.store(mem, one);
    let one = builder.load(mem);

    let two = builder.add(one, one);

    let block1 = builder.add_block();
    builder.br(block1);
    builder.set_block(block1);

    let three = builder.add(two, one);
    let cond = builder.eq(two, three);

    let block2 = builder.add_block();
    let block3 = builder.add_block();
    builder.cond_br(cond, block2, block3);

    builder.set_block(block2);
    builder.ret(cond);

    builder.set_block(block3);
    builder.ret(cond);

    println!("{}", function);
}
