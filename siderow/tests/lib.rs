use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

use siderow::{
    arch::x86::{self, regalloc},
    ssa::{self, parser},
};

#[test]
fn test_all() {
    let path = "tests/testcases/";
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }

        test_file(&entry.path())
    }
}

fn test_file(path: &Path) {
    println!("processing: {:?}", path);

    let input = fs::read_to_string(path).expect("cannot read file");
    let mut input_lines = input.lines();
    let first_line = input_lines.next().unwrap();
    let program = input_lines.collect::<Vec<&str>>().join("\n");

    let module = parser::parse(&program);

    let expected = first_line.strip_prefix("// ").unwrap().parse().unwrap();
    exec(module, expected).expect("cannot execute program");
}

fn exec(module: ssa::Module, expected: i32) -> io::Result<()> {
    let mut assembly = x86::instsel::translate(module);
    regalloc::allocate(&mut assembly);

    let mut file = File::create("./tmp.s")?;
    file.write_all(assembly.stringify().as_bytes())?;

    let cc = env::var("CC").unwrap_or(String::from("gcc"));
    let status = Command::new(cc)
        .arg("./tmp.s")
        .arg("-o")
        .arg("./tmp")
        .status()?;

    if !status.success() {
        panic!("failed to compile");
    }

    let status = Command::new("./tmp").status()?;
    let actual = status.code().unwrap();

    assert_eq!(actual, expected);

    Ok(())
}
