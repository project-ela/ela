use eir::emulator::Emulator;
use std::env;

extern crate eir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: eir <file>");
        return;
    }
    let file = args[1].as_str();
    let mut emu = Emulator::new(0x7C00, 0x7c00);
    emu.load_from_file(file);
    emu.run()
}
