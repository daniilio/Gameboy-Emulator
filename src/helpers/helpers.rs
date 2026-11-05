use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn get_lsb(word: u16) -> u8
{
    (word & 0xFF) as u8
}

pub fn get_msb(word: u16) -> u8
{
    ((word & 0xFF00) >> 8) as u8
}

pub fn set_msb(word: &mut u16, value: u8) {
    *word = ((value as u16) << 8) | (*word & 0xFF);
}

pub fn set_lsb(word: &mut u16, value: u8) {
    *word = (value as u16) | (*word & 0xFF00);
}

/// Check if bit at position in word is set
pub fn is_bit_set(word: &u16, position: u8) -> bool {
    ((word >> position) & 1) != 0
}

/// Set bit at position in word
pub fn set_bit(word: &mut u16, position: u8, value: bool) {
    if value {
        *word |= 1u16 << position;
    } else {
        *word &= !(1u16 << position);
    }
}

pub fn read_file(file_path: PathBuf, buffer: &mut [u8]) -> io::Result<usize>
{
    let mut file = File::open(file_path)?;
    let mut bytes_read = 0;
    while bytes_read < buffer.len() {
        let b = file.read(&mut buffer[bytes_read..])?;
        if b == 0 {
            break; // EOF
        }
        bytes_read += b;
    }

    Ok(bytes_read)
}
