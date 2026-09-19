struct MemoryBus {
    memory: [u8; 0xFFFF],
    cartridge_memory[u8; 0x200000],
}

impl MemoryBus {
    fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.read_vram(addr - VRAM_BEGIN)
            }
            _ => {
                self.memory[addr]
            }
        }
    }

    fn read_word(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr.wrapping_add(1));

        ((msb as u16) << 8) | (lsb as u16)
    }

    fn write_byte(&mut self, data: u8, addr: u16) {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.write_vram(addr - VRAM_BEGIN);
            }
            _ => {
                self.memory[addr] = data;
            }
        }
    }

    fn write_word(&mut self, data: u16, addr: u16) {
        let lsb = ((data & 0xFF00) >> 8) as u8;
        let msb = (data & 0xFF) as u8;

        self.write_byte(lsb, addr);
        self.write_byte(msb, addr.wrapping_add(1));
    }
}
