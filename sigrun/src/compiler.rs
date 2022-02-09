use anyhow::Result;
use siderow::arch::x86;
use std::fs;

use crate::{
    common::cli::CompilerConfig,
    frontend::{self, lexer::SourceFile},
    middleend,
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

pub fn compile(source: SourceFile, config: &CompilerConfig) -> Result<String> {
    let tokens = frontend::lexer::tokenize(source)?;
    if config.dump_token {
        println!("{:?}", tokens);
    }

    let module = frontend::parser::parse(tokens)?;
    if config.dump_ast {
        println!("{:?}", module);
    }

    let mut symtab = frontend::type_check::apply(&module)?;
    frontend::sema_check::apply(&module)?;

    let mut module = middleend::ssagen::translate(module, &mut symtab);
    if config.optimize {
        siderow::ssa::pass::cf::apply(&mut module);
        siderow::ssa::pass::dce::apply(&mut module);
    }
    if config.dump_ir {
        println!("{}", module.dump());
    }

    let mut asm = x86::instsel::translate(module);
    x86::regalloc::allocate(&mut asm);
    Ok(asm.stringify())
}
