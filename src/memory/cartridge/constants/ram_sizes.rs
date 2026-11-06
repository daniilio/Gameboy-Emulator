// RAM size codes
const RAM_NONE: u8 = 0x00;
const RAM_UNUSED: u8 = 0x01;
const RAM_8_KIB: u8 = 0x02;
const RAM_32_KIB: u8 = 0x03;
const RAM_128_KIB: u8 = 0x04;
const RAM_64_KIB: u8 = 0x05;

pub fn get_ram_size(ram_code: u8) -> usize {
    match ram_code {
        RAM_NONE => 0,
        RAM_8_KIB => 8 * 1024,
        RAM_32_KIB => 32 * 1024,
        RAM_128_KIB => 128 * 1024,
        RAM_64_KIB => 64 * 1024,
        _ => panic!("Unsupported RAM size code: {ram_code:#04X}"),
    }
}
