use std::env;
use urd::emulator::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: urd <file>");
        return;
    }
    let file = args[1].as_str();
    let mut emu = Emulator::new(0x0000, 0x7c00);
    emu.loadFromFile(file);
}
