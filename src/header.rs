// https://www.typewritten.org/Manual/UCB/UNIX/2.11BSD/man5/a.out.html

use std::io::{Read, Seek, SeekFrom};

const HEADER: u16 = 16;
const OVERLAY_SIZE: u16 = 2 + 15 * 2;

const MAGIC5: u16 = 0o430;
const MAGIC6: u16 = 0o431;

#[derive(Debug)]
pub struct Header {
    magic: u16,
    text_size: u16,
    data_size: u16,
    bss_size: u16, // Block Started by Symbol
    syms: u16,
    entry: u16,
    _unused: u16,
    flags: u16,
    overlay: OverlayInfo,
}

#[derive(Debug)]
pub struct OverlayInfo {
    max_size: u16,
    sizes: [u16; 15],
}

impl Header {
    pub fn new<R: Read + Seek>(file: &mut R) -> std::io::Result<Self> {
        file.seek(SeekFrom::Start(0))?;

        let mut first_header = [0u8; 16];
        file.read_exact(&mut first_header)?;

        let magic = Self::get_word(&first_header, 0);
        let text_size = Self::get_word(&first_header, 2);
        let data_size = Self::get_word(&first_header, 4);
        let bss_size = Self::get_word(&first_header, 6);
        let syms = Self::get_word(&first_header, 8);
        let entry = Self::get_word(&first_header, 10);
        let _unused = Self::get_word(&first_header, 12);
        let flags = Self::get_word(&first_header, 14);

        let mut second_header = [0u8; 2 * 16];
        file.read_exact(&mut second_header)?;
        let max_size = Self::get_word(&second_header, 0);

        let sizes = (0..15)
            .map(|offset| Self::get_word(&second_header, 2 + 2 * offset))
            .collect::<Vec<_>>();
        let overlay = OverlayInfo {
            max_size,
            sizes: sizes.as_slice().try_into().unwrap(),
        };

        Ok(Self {
            magic,
            text_size,
            data_size,
            bss_size,
            syms,
            entry,
            _unused: 0,
            flags,
            overlay,
        })
    }

    pub fn get_text_offset(&self) -> u64 {
        (if self.magic == MAGIC5 || self.magic == MAGIC6 {
            HEADER + OVERLAY_SIZE
        } else {
            HEADER
        }) as u64
    }

    fn get_word(data: &[u8], offset: usize) -> u16 {
        (data[offset + 1] as u16) << 8 | (data[offset] as u16)
    }

    pub(crate) fn get_text_segment(&self) -> (u64, u16) {
        (self.get_text_offset(), self.text_size)
    }

    pub(crate) fn get_data_segment(&self) -> (u64, u16) {
        let textoff = self.get_text_offset();
        let dataoff = self
            .overlay
            .sizes
            .iter()
            .fold(textoff, |sum, size| sum + *size as u64);

        (dataoff, self.data_size)
    }
}
