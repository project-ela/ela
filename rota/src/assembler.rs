use std::{error, fs};

use crate::{
    backend::{
        gen_code::{self, SectionName},
        gen_elf,
    },
    common::error::Error,
    frontend::{
        lexer::{self, SourceFile},
        parser,
    },
};

pub fn assemble_to_file(
    input_file: String,
    output_file: String,
) -> Result<(), Box<dyn error::Error>> {
    let source = SourceFile {
        filename: input_file.clone(),
        content: fs::read_to_string(input_file)?,
    };
    let output = assemble(source)?;
    fs::write(output_file, output)?;
    Ok(())
}

pub fn assemble_raw_to_file(
    input_file: String,
    output_file: String,
) -> Result<(), Box<dyn error::Error>> {
    let source = SourceFile {
        filename: input_file.clone(),
        content: fs::read_to_string(input_file)?,
    };
    let output = assemble_raw(source)?;
    fs::write(output_file, output)?;
    Ok(())
}

pub fn assemble(source: SourceFile) -> Result<Vec<u8>, Error> {
    lexer::tokenize(source)
        .and_then(parser::parse)
        .and_then(gen_code::generate)
        .and_then(gen_elf::generate)
        .map(|elf| elf.to_bytes())
}

pub fn assemble_raw(source: SourceFile) -> Result<Vec<u8>, Error> {
    let obj = lexer::tokenize(source)
        .and_then(parser::parse)
        .and_then(gen_code::generate)?;

    let text_section = obj
        .sections
        .into_iter()
        .find(|section| section.name == SectionName::Text)
        .unwrap();

    Ok(text_section.data)
}
