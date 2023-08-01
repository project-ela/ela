use std::{env, fs, path::Path};

use siderow::{
    arch::aarch64::{self, asm::Printer},
    ssa::parser,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: compile <filepath>");
        return;
    }

    let program = load_file(Path::new(&args[1]));

    let module = parser::parse(&program);
    let mut assembly = aarch64::instsel::translate(module);
    aarch64::regalloc::allocate(&mut assembly);

    let mut asm = String::new();
    assembly.print(&mut asm);
    println!("{}", asm);
}

fn load_file(path: &Path) -> String {
    let input = fs::read_to_string(path).expect("cannot read file");
    let mut input_lines = input.lines();

    let _ = input_lines.next().unwrap();

    input_lines.collect::<Vec<&str>>().join("\n")
}
