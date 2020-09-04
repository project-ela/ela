use crate::{
    backend::{gen_x86, regalloc},
    frontend::{lexer, parser, pass::symbol_pass},
    middleend::{optimize::constant_folding, tacgen},
};
use std::fs;

pub fn compile_to_file(input_file: String, output_file: String) -> Result<(), String> {
    match fs::read_to_string(input_file) {
        Ok(source) => {
            let output = compile(source)?;
            match fs::write(output_file, output) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("failed to compile: {}", err)),
            }
        }
        Err(err) => Err(format!("failed to compile: {}", err)),
    }
}

pub fn compile(source: String) -> Result<String, String> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::parse(tokens)?;
    symbol_pass::apply(&program)?;
    let program = constant_folding::optimize(program);
    let program = tacgen::generate(program)?;
    let program = regalloc::alloc_register(program)?;
    let output = gen_x86::generate(program)?;
    Ok(output)
}
