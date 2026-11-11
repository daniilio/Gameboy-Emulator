// TODO: Complete IORegisters: Since we're scattering some of the registers around, we can complete the rest later

pub struct IORegisters {
    io_registers: [u8; IO_REGISTERS_END - IO_REGISTERS_BEGIN + 1],
}

impl IORegisters {
    /// Returns whether the boot ROM register is toggled in bit 0.
    /// This indicates whether Boot ROM is mapped
    pub fn is_boot_rom_mapped(&self) -> bool
    {
        let reg = self.memory_bus.read_byte(BOOT_ROM_REGISTER);
        (reg & 1) != 1
    }

    pub fn write_byte(&mut self, data: u8, addr: u16)
    {
        let addr = addr as usize;
        self.io_registers[addr - IO_REGISTERS_BEGIN] = data;

        let addr = addr as u16;
        match addr {
            OAM_DMA_CONTROL_REGISTER => {
                let mut src_addr: u16 = 0;
                set_msb(&mut src_addr, data);
                self.memory_bus.dma_write(src_addr);
            }
            _ => {
                // TODO: Handle other IO Registers
            }
        }
    }
}
