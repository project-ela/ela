use std::env;

use herja::linker;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: herja <input_file>... <output_file>");
        std::process::exit(0);
    }

    let args_num = args.len() - 1;
    let input_files = args[1..args_num].to_vec();
    let output_file = args[args_num].clone();

    if let Err(err) = linker::link(input_files, output_file) {
        println!("Failed to link: {}", err);
        std::process::exit(1);
    }
}
