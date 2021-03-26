use siderow::ssa;

#[test]
fn do_test() {
    let mut function = ssa::Function::new("hoge");
    let mut builder = ssa::FunctionBuilder::new(&mut function);

    let entry_block = builder.add_block();
    builder.set_block(entry_block);

    let one = ssa::Value::Immediate(ssa::Immediate::I32(1));
    let two = builder.add(one, one);
    let three = builder.add(two, one);
    let cond = builder.eq(two, three);

    builder.ret(cond);

    println!("{}", function);
}
