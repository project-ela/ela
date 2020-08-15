use std::env;
use verdandi::compiler::compile_to_file;

extern crate verdandi;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: verdandi <input_file> <output_file>")
    } else {
        let input_file = args[1].to_string();
        let output_file = args[2].to_string();
        if let Err(err) = compile_to_file(input_file, output_file) {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
