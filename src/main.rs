use skuld::assembler::assemble_to_file;
use std::env;

extern crate skuld;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: skuld <input_file> <output_file>");
        std::process::exit(0);
    }

    let input_file = args[1].to_string();
    let output_file = args[2].to_string();
    if let Err(err) = assemble_to_file(input_file, output_file) {
        println!("Failed to assemble: {}", err);
        std::process::exit(1);
    }
}
