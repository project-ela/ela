use crate::backend::codegen::gen_x86::generate;
use crate::frontend::lexer::tokenize;
use crate::frontend::parser::parse;
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
