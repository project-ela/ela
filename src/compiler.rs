use crate::{
    backend::{gen_x86, regalloc},
    common::cli::CompilerConfig,
    frontend::{
        lexer::{self, SourceFile},
        parser, symbol_pass,
    },
    middleend::{irgen, optimize::constant_folding},
};
use anyhow::Result;
use std::fs;

pub fn compile_to_file(config: CompilerConfig) -> Result<()> {
    let source = SourceFile {
        filename: config.input_file.to_owned(),
        content: fs::read_to_string(&config.input_file)?,
    };
    let output = compile(source, &config)?;
    fs::write(&config.output_file, output)?;
    Ok(())
}

pub fn compile(source: SourceFile, config: &CompilerConfig) -> Result<String> {
    let tokens = lexer::tokenize(source)?;
    if config.dump_token {
        println!("{:#?}", tokens);
    }

    let mut program = parser::parse(tokens)?;
    symbol_pass::apply(&mut program)?;
    if config.dump_ast {
        println!("{:#?}", program);
    }

    if config.optimize {
        program = constant_folding::optimize(program);
    }

    let program = irgen::generate(program)?;
    let program = regalloc::alloc_register(program)?;
    if config.dump_ir {
        println!("{}", program.dump());
    }

    let output = gen_x86::generate(program, config.tse)?;
    Ok(output)
}
