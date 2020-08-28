pub mod generator;
pub mod parser;
pub mod tokenizer;

use crate::compiler::generator::gen_x86::generate;
use crate::compiler::parser::parser::parse;
use crate::compiler::tokenizer::tokenizer::tokenize;
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
    tokenize(source)
        .and_then(|tokens| parse(tokens))
        .and_then(|program| generate(program))
}
