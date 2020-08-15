use std::env;
use verdandi::codegen::generate;
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
    tokenize(source)
        .and_then(|tokens| parse(tokens))
        .and_then(|ast| generate(ast))
}
