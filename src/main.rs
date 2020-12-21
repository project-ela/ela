use skuld::assembler;
use std::env;

extern crate skuld;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !matches!(args.len(), 3 | 4) {
        show_usage();
    }

    let input_file = args[1].to_string();
    let output_file = args[2].to_string();
    let output_raw = args.get(3).map_or(false, |arg| arg == "--raw");

    let err = if output_raw {
        assembler::assemble_raw_to_file(input_file, output_file)
    } else {
        assembler::assemble_to_file(input_file, output_file)
    };

    if let Err(err) = err {
        println!("Failed to assemble: {}", err);
        std::process::exit(1);
    }
}

fn show_usage() {
    println!("usage: skuld <input_file> <output_file> [--raw]");
    std::process::exit(0);
}
