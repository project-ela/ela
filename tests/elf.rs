extern crate rota;

use std::fs;

use rota::{assembler, frontend::lexer::SourceFile};

#[test]
fn simple() {
    let filename = "tests/testcases/simple.s";
    let source = SourceFile {
        filename: filename.to_string(),
        content: fs::read_to_string(filename).unwrap(),
    };
    let actual_output = assembler::assemble(source).unwrap();
    let expected_output = fs::read("tests/testcases/simple.o").unwrap();

    assert_eq!(actual_output, expected_output);
}
