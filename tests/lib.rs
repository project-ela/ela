use siderow::ssa;

#[test]
fn do_test() {
    let one = ssa::Value::Immediate(1);
    let two = ssa::Instruction::Add(Box::new(one.clone()), Box::new(one));
    println!("{:?}", two);
}
