use std::env;
use verdandi::ast::AST;
use verdandi::parser::parse;
use verdandi::tokenizer::tokenize;

extern crate verdandi;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: verdandi <number>")
    } else {
        let source = args[1].to_string();
        if let Err(err) = compile(source) {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}

fn compile(source: String) -> Result<(), String> {
    match tokenize(source).and_then(|tokens| parse(tokens)) {
        Err(err) => Err(err),
        Ok(ast) => match ast {
            AST::Integer { value } => {
                println!(".intel_syntax noprefix");
                println!(".global main");
                println!("main:");
                println!("  mov eax, {}", value);
                println!("  ret");
                Ok(())
            }
            x => Err(format!("unexpected node: {:?}", x)),
        },
    }
}
