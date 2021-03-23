use anyhow::Result;
use std::fs;

use crate::{
    backend::{gen_x86, regalloc},
    common::cli::CompilerConfig,
    frontend::{self, lexer::SourceFile},
    middleend::irgen,
};

pub fn compile_to_file(config: CompilerConfig) -> Result<()> {
    let source = SourceFile {
        filename: config.input_file.to_owned(),
        content: fs::read_to_string(&config.input_file)?,
    };
    let output = compile(source, &config)?;
    fs::write(&config.output_file, output)?;
    Ok(())
}

pub fn compile(source: SourceFile, _config: &CompilerConfig) -> Result<String> {
    let tokens = frontend::lexer::tokenize(source)?;
    let program = frontend::parser::parse(tokens)?;
    let _symtab = frontend::type_check::apply(&program)?;
    frontend::sema_check::apply(&program)?;

    let program = irgen::generate(program)?;
    let program = regalloc::alloc_register(program)?;
    let output = gen_x86::generate(program, false)?;
    Ok(output)

    // let program = middleend::ssa::translate(&program)?;
    // middleend::optimize::apply(&mut program)?;

    // let code = backend::x86::translate(&program)?;
    // backend::x86::regalloc::apply(&mut code)?;
    // backend::x86::optimize::apply(&mut code)?;
    // backend::x86::generator::generate(code)?;
}
