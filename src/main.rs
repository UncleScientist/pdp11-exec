use std::{fs::File, io::BufReader};

use crate::{
    aout::Memory,
    instructions::{Instruction, Opcode},
};

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
    let mut offset = 0;
    for _ in 0..10 {
        let instr = Instruction::new(offset, &memory);
        let words = instr.size();
        print!("{offset:06o}: ");
        print!("{:06o}", memory.get_word_at(offset));
        if words > 2 {
            print!(" {:06o}", memory.get_word_at(offset + 2));
            if words > 4 {
                print!(" {:06o}", memory.get_word_at(offset + 4));
            } else {
                print!("      ");
            }
        } else {
            print!("             ");
        }
        println!(" | {instr}");
        offset += words;
    }
}

fn open_file(filename: &str) -> std::io::Result<Memory> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    Memory::new(&mut reader)
}
