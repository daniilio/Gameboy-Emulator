// ROM size codes
const ROM_32_KIB: u8 = 0x00;
const ROM_64_KIB: u8 = 0x01;
const ROM_128_KIB: u8 = 0x02;
const ROM_256_KIB: u8 = 0x03;
const ROM_512_KIB: u8 = 0x04;
const ROM_1_MIB: u8 = 0x05;
const ROM_2_MIB: u8 = 0x06;
const ROM_4_MIB: u8 = 0x07;
const ROM_8_MIB: u8 = 0x08;
const ROM_1_1_MIB: u8 = 0x52;
const ROM_1_2_MIB: u8 = 0x53;
const ROM_1_5_MIB: u8 = 0x54;

pub fn get_rom_size(rom_code: u8) -> usize {
    match rom_code {
        ROM_32_KIB => 32 * 1024,
        ROM_64_KIB => 64 * 1024,
        ROM_128_KIB => 128 * 1024,
        ROM_256_KIB => 256 * 1024,
        ROM_512_KIB => 512 * 1024,
        ROM_1_MIB => 1024 * 1024,
        ROM_2_MIB => 2 * 1024 * 1024,
        ROM_4_MIB => 4 * 1024 * 1024,
        ROM_8_MIB => 8 * 1024 * 1024,
        ROM_1_1_MIB => 72 * 16 * 1024,
        ROM_1_2_MIB => 80 * 16 * 1024,
        ROM_1_5_MIB => 96 * 16 * 1024,
        _ => panic!("Unsupported ROM size code: {rom_code:#04X}"),
    }
}
