use sigrun::{common::cli, compiler::compile_to_file};

extern crate sigrun;

fn main() {
    match cli::parse_arguments() {
        Ok(config) => {
            if let Err(err) = compile_to_file(config) {
                println!("failed to compile:\n{}", err);
                std::process::exit(1);
            }
        }
        Err(_) => println!("Usage: sigrun <input_file> <output_file>"),
    }
}
