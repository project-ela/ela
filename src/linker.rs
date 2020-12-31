use std::fs;

use elfen::elf::Elf;

pub fn link_to_files(input_files: Vec<String>, output_file: String) -> Result<(), String> {
    let input_elfs = input_files
        .into_iter()
        .map(|path| Elf::read_from_file(&path))
        .collect();

    let output_elf = link(input_elfs)?;

    let elf_bytes = output_elf.to_bytes();
    fs::write(output_file, elf_bytes).unwrap();

    Ok(())
}

pub fn link(input_elfs: Vec<Elf>) -> Result<Elf, String> {
    let linker = Linker::new(input_elfs);
    let output_elf = linker.link()?;
    Ok(output_elf)
}

struct Linker {
    input_elfs: Vec<Elf>,
    output_elf: Elf,
}

impl Linker {
    fn new(input_elfs: Vec<Elf>) -> Self {
        Self {
            input_elfs,
            output_elf: Elf::default(),
        }
    }

    fn link(mut self) -> Result<Elf, String> {
        Ok(self.output_elf)
    }
}
