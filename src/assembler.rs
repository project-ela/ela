use crate::{backend::gen_code, common::error::Error, frontend::lexer};
use crate::{backend::gen_elf, frontend::parser};
use std::{error, fs};

pub fn assemble_to_file(
    input_file: String,
    output_file: String,
) -> Result<(), Box<dyn error::Error>> {
    let source = fs::read_to_string(input_file)?;
    let output = assemble(source)?;
    fs::write(output_file, output)?;
    Ok(())
}

pub fn assemble_raw_to_file(
    input_file: String,
    output_file: String,
) -> Result<(), Box<dyn error::Error>> {
    let source = fs::read_to_string(input_file)?;
    let output = assemble_raw(source)?;
    fs::write(output_file, output)?;
    Ok(())
}

pub fn assemble(source: String) -> Result<Vec<u8>, Error> {
    lexer::tokenize(source)
        .and_then(parser::parse)
        .and_then(gen_code::generate)
        .and_then(gen_elf::generate)
        .map(|elf| elf.to_bytes())
}

pub fn assemble_raw(source: String) -> Result<Vec<u8>, Error> {
    lexer::tokenize(source)
        .and_then(parser::parse)
        .and_then(gen_code::generate)
        .map(|data| data.program)
}
