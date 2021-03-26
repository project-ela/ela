use siderow::ssa;

#[test]
fn do_test() {
    let mut function = ssa::Function::new("hoge");

    let one = ssa::Value::Immediate(1);
    let two = ssa::Instruction::Add(Box::new(one.clone()), Box::new(one.clone()));
    let three = ssa::Instruction::Add(Box::new(ssa::Value::Instruction(two)), Box::new(one));

    function.add_instruction(three);

    println!("{:?}", function);
}
