use crate::{
    backend::{gen_x86, regalloc},
    common::cli::CompilerConfig,
    frontend::{lexer, parser, pass::symbol_pass},
    middleend::{optimize::constant_folding, tacgen},
};
use std::fs;

pub fn compile_to_file(config: CompilerConfig) -> Result<(), String> {
    match fs::read_to_string(&config.input_file) {
        Ok(source) => {
            let output = compile(source, &config)?;
            match fs::write(&config.output_file, output) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("failed to compile: {}", err)),
            }
        }
        Err(err) => Err(format!("failed to compile: {}", err)),
    }
}

pub fn compile(source: String, config: &CompilerConfig) -> Result<String, String> {
    let tokens = lexer::tokenize(source)?;
    if config.dump_tokens {
        println!("{:#?}", tokens);
    }

    let mut program = parser::parse(tokens)?;
    if config.dump_nodes {
        println!("{:#?}", program);
    }

    symbol_pass::apply(&program)?;
    if config.optimize {
        program = constant_folding::optimize(program);
    }

    let program = tacgen::generate(program)?;
    let program = regalloc::alloc_register(program)?;
    if config.dump_tac {
        println!("{}", program.dump());
    }

    let output = gen_x86::generate(program)?;
    Ok(output)
}
