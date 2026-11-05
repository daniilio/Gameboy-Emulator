use std::io;
use std::path::PathBuf;

use crate::memory::memory_bus::{
    ROM_BANK_0_BEGIN, 
    ROM_BANK_0_END, 
    ROM_BANK_SWAP_BEGIN, 
    ROM_BANK_SWAP_END
};
use crate::helpers::helpers::read_file;

const BOOT_ROM_BEGIN: u16 = 0x0000;
const BOOT_ROM_END:   u16 = 0x00FF;

// Memory-mapped register
const BOOT_ROM_DISABLE: u16 = 0xFF50;

struct ExternalBus {
    boot_rom_disable: u8,
    mbc: MemoryBankController,
    
    // Cartridge header
    battery_cartridge: bool,
    ram_cartridge: bool,
    cartridge_type: u8,

    cartridge_rom: [u8; (ROM_BANK_SWAP_END - ROM_BANK_0_BEGIN + 1) as usize],
    cartridge_ram: [u8; (CARTRIDGE_RAM_END - CARTRIDGE_RAM_BEGIN + 1) as usize],
    boot_rom: [u8; (BOOT_ROM_END - BOOT_ROM_BEGIN + 1) as usize],
}

impl ExternalBus {
    pub fn load_boot_rom(&mut self, file_path: PathBuf) -> io::Result<usize>
    {
        read_file(file_path, &mut self.boot_rom)
    }

    pub fn load_cartridge(&mut self, file_path: Option<PathBuf>) -> io::Result<usize>
    {
        if file_path.is_none() {
            eprintln!("No cartridge file path provided: {}", err);
            Ok(0);
        }

        let file_path = Some(file_path);
        let cartridge_rom 
        read_file(file_path, &mut self.cartridge_rom)?;

        // 

        Ok(0)
    }

    fn read_rom(&self, addr: u16) -> u8
    {
        match addr {
            BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                if self.is_boot_rom_disabled() {
                    self.cartridge_rom[(addr - ROM_BANK_0_BEGIN) as usize]
                } else {
                    self.boot_rom[(addr - BOOT_ROM_BEGIN) as usize]
                }
            }
            BOOT_ROM_DISABLE => {
                self.boot_rom_disable
            }
            _ => {
                self.cartridge_rom[addr as usize]
            }
        }
    }

    fn write_rom(&mut self, data: u8, addr: u16)
    {
        match addr {
            BOOT_ROM_DISABLE => {
                if !self.is_boot_rom_disabled() {
                    self.boot_rom_disable = data;
                }
            }
            _ => {}
        }
    }

    fn is_boot_rom_disabled(&self) -> bool
    {
        self.boot_rom_disable & 1 == 1
    }
}
