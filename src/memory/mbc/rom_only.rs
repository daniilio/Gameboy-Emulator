struct ROMOnly {
    cartridge_rom: Vec<u8>,
    cartridge_ram: Arc<Mutex<Vec<u8>>>,
}

impl MemoryBankController for ROMOnly {
    fn read_byte(&self, addr: u16) -> u8 
    {
        match addr {
            ROM_BANK_0_BEGIN ..= ROM_BANK_SWAP_END => {
                if addr < cartridge_rom.len() {
                    self.cartridge_rom[addr as usize]
                } else {
                    0xFF
                }
            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {
                // Cartridge RAM has 13-bit address bus
                let cart_addr = (addr as usize) & 0x1FFF;

                let ram = self.cartridge_ram.lock().unwrap();
                if cart_addr < ram.len() {
                    self.cartridge_ram[cart_addr]
                } else {
                    0xFF
                }
            }
            _ => {
                0xFF
            }
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8)
    {
        match addr {
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {
                // Cartridge RAM has 13-bit address bus
                let cart_addr = (addr as usize) & 0x1FFF;
                
                let mut ram = self.cartridge_ram.lock().unwrap();
                if cart_addr < ram.len() {
                    ram[cart_addr] = value;
                }
            }
            _ => {}
        }
    }
}
