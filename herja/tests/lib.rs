extern crate herja;

use elfen::elf::Elf;
use herja::linker;
use std::fs;

#[test]
fn link() {
    let input_files = vec!["tests/testcases/file1.o", "tests/testcases/file2.o"];
    let input_elfs = input_files
        .into_iter()
        .map(|path| Elf::read_from_file(path))
        .collect();
    let output_elf = linker::link(input_elfs).unwrap();
    let actual_bytes = output_elf.to_bytes();
    let expected_bytes = fs::read("tests/testcases/file").unwrap();
    assert_eq!(actual_bytes, expected_bytes);
}
