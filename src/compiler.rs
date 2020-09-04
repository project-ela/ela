use crate::{
    frontend::{lexer, parser, pass::symbol_pass},
    middleend::tacgen,
};
use std::fs;

pub fn compile_to_file(input_file: String, output_file: String) -> Result<(), String> {
    match fs::read_to_string(input_file) {
        Ok(source) => {
            compile(source)?;
            Ok(())
            // let output = compile(source);
            // match fs::write(output_file, output) {
            //     Ok(_) => Ok(()),
            //     Err(err) => Err(format!("failed to compile: {}", err)),
            // }
        }
        Err(err) => Err(format!("failed to compile: {}", err)),
    }
}

pub fn compile(source: String) -> Result<(), String> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::parse(tokens)?;
    symbol_pass::apply(&program)?;
    let program = tacgen::generate(program)?;
    println!("{:?}", program);
    Ok(())
}
