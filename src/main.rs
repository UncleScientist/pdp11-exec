use std::{fs::File, io::BufReader};

use crate::aout::Memory;

mod aout;
mod header;
mod instructions;
mod symbol_data;

fn main() {
    if let Ok(memory) = open_file("exe/larn12.0") {
        execute(memory);
    } else {
        eprintln!("Unable to read header from exe file");
    }
}

fn execute(memory: Memory) {
    println!("-- got here --");
    println!("{:x?}", memory.get_instruction_at(0));
}

fn open_file(filename: &str) -> std::io::Result<Memory> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    Memory::new(&mut reader)
}
