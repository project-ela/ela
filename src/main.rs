use verdandi::{common::cli, compiler::compile_to_file};

extern crate verdandi;

fn main() {
    match cli::parse_arguments() {
        Ok(config) => {
            if let Err(err) = compile_to_file(config) {
                println!("{}", err);
                std::process::exit(1);
            }
        }
        Err(_) => println!("Usage: verdandi <input_file> <output_file>"),
    }
}
