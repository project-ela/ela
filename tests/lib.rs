use siderow::ssa;

#[test]
fn do_test() {
    let mut function = ssa::Function::new("hoge");

    let one = ssa::Value::Immediate(1);
    let two = function.add_instruction(ssa::Instruction::Add(one, one));
    let three = function.add_instruction(ssa::Instruction::Add(ssa::Value::Instruction(two), one));
    function.add_instruction(ssa::Instruction::Ret(ssa::Value::Instruction(three)));

    println!("{:?}", function);
}
