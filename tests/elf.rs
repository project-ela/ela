extern crate skuld;

use std::fs;

use skuld::assembler;

#[test]
fn simple() {
    let source = fs::read_to_string("tests/testcases/simple.s").unwrap();
    let actual_output = assembler::assemble(source).unwrap();
    let expected_output = fs::read("tests/testcases/simple.o").unwrap();

    assert_eq!(actual_output, expected_output);
}
