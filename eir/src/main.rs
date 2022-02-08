use eir::emulator::Emulator;
use std::env;

extern crate eir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !matches!(args.len(), 2 | 3) {
        println!("Usage: eir <file>");
        return;
    }

    let file = args[1].as_str();
    let dump = args.get(2).map_or(false, |arg| arg == "--dump");

    let mut emu = Emulator::new(0x7C00, 0x7c00);
    emu.dump_state = dump;
    emu.load_elf(file);
    emu.run()
}
