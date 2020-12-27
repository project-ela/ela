use crate::{backend::gen_code, frontend::lexer};
use crate::{backend::gen_elf, frontend::parser};
use std::fs;

pub fn assemble_to_file(input_file: String, output_file: String) -> Result<(), String> {
    match fs::read_to_string(input_file) {
        Ok(source) => {
            let output = assemble(source)?;
            if let Err(err) = fs::write(output_file, output) {
                Err(format!("{}", err))
            } else {
                Ok(())
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn assemble_raw_to_file(input_file: String, output_file: String) -> Result<(), String> {
    match fs::read_to_string(input_file) {
        Ok(source) => {
            let output = assemble_raw(source)?;
            if let Err(err) = fs::write(output_file, output) {
                Err(format!("{}", err))
            } else {
                Ok(())
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn assemble(source: String) -> Result<Vec<u8>, String> {
    lexer::tokenize(source)
        .and_then(parser::parse)
        .and_then(gen_code::generate)
        .and_then(gen_elf::generate)
        .map(|elf| elf.to_bytes())
}

pub fn assemble_raw(source: String) -> Result<Vec<u8>, String> {
    lexer::tokenize(source)
        .and_then(parser::parse)
        .and_then(gen_code::generate)
        .map(|data| data.program)
}
