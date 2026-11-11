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

pub fn set_msb(word: u16, value: u8) -> u16 
{
    ((value as u16) << 8) | (word & 0xFF)
}

pub fn set_lsb(word: u16, value: u8) -> u16 
{
    (value as u16) | (word & 0xFF00)
}

pub fn is_bit_set(byte: u8, position: u8) -> u8
{
    ((byte >> position) & 1) != 0
}

pub fn is_bit_set_16bit(word: u16, position: u8) -> bool {
    ((word >> position) & 1) != 0
}

pub fn set_bit(byte: u8, position: u8, value: bool) -> u8
{
    if value {
        byte | (1u8 << position)
    } else {
        byte & !(1u8 << position)
    }
}

pub fn set_bit_16bit(word: u16, position: u8, value: bool) -> u16 {
    if value {
        word | (1u16 << position)
    } else {
        word & !(1u16 << position)
    }
}
