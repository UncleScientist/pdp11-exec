use std::io::{Read, Seek, SeekFrom};

use crate::{header::Header, symbol_data::SymbolData};

const MEM_SIZE: usize = 64 * 1024;

pub struct Memory {
    header: Header,
    text: [u8; MEM_SIZE],
    ram: [u8; MEM_SIZE],
    symbols: Vec<SymbolData>,
}

impl Memory {
    pub fn new<R: Read + Seek>(file: &mut R) -> std::io::Result<Self> {
        let header = Header::new(file)?;
        let (textoff, textsize) = header.get_text_segment();
        let mut text = [0u8; MEM_SIZE];
        file.seek(SeekFrom::Start(textoff))?;
        file.read_exact(&mut text[0..textsize as usize])?;

        let (dataoff, datasize) = header.get_data_segment();
        let mut ram = [0u8; MEM_SIZE];
        file.seek(SeekFrom::Start(dataoff))?;
        file.read_exact(&mut ram[0..datasize as usize])?;

        Ok(Self {
            header,
            text,
            ram,
            symbols: Vec::new(), // TODO
        })
    }

    pub fn get_word_at(&self, pc: usize) -> u16 {
        Self::get_word(&self.text, pc)
    }

    pub fn get_word(data: &[u8], offset: usize) -> u16 {
        (data[offset + 1] as u16) << 8 | (data[offset] as u16)
    }
}
