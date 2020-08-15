use std::env;
use verdandi::token::Token;
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
    match tokenize(source) {
        Err(err) => Err(err),
        Ok(tokens) => match tokens.get(0).unwrap() {
            Token::IntLiteral { value } => {
                println!(".intel_syntax noprefix");
                println!(".global main");
                println!("main:");
                println!("  mov eax, {}", value);
                println!("  ret");
                Ok(())
            }
            _ => Err("unexpected token".to_string()),
        },
    }
}
