// Boot ROM overlaps with ROM bank 0
const BOOT_ROM_BEGIN: u16 = 0x0000;
const BOOT_ROM_END:   u16 = 0x00FF;
const BOOT_ROM_SIZE:  u16 = (BOOT_ROM_END - BOOT_ROM_BEGIN + 1) as usize;

// This area of WRAM is echoed
const WRAM_ECHO_BEGIN: u16 = 0xC000;
const WRAM_ECHO_END:   u16 = 0xDDFF;

// Boot ROM Disable register
const BOOT_ROM_DISABLE: u16 = 0xFF50;

// OAM DMA (direct-memory access) register
const OAM_DMA: u16 = 0xFF46;

pub struct MemoryBus {
    // Memory bus registers
    boot_rom_disable: u8,
    oam_dma: u8,

    // Addressable components
    boot_rom: BootROM,
    cartridge: Cartridge,
    ppu: PictureProcessingUnit,
    wram: [u8; WRAM_SIZE],
    io_registers: IORegisters,
    hram: [u8; HRAM_SIZE],
    interrupt_controller: InterruptController,
}

impl MemoryBus {
    pub fn new() -> Self
    {
        // TODO: Complete constructor
        Self {
            boot_rom_disable: 0x00,

        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 
    {
        match addr {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                match addr {
                    BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                        if self.boot_rom_disable == 0x00 {
                            self.boot_rom.read_byte(addr)
                        } else {
                            self.cartridge.read_byte(addr)
                        }
                    }
                    _ => {
                        self.cartridge.read_byte(addr)
                    }
                }
            }
            ROM_BANK_SWAP_BEGIN ..= ROM_BANK_SWAP_END => {
                self.cartridge.read_byte(addr)
            }
            VRAM_BEGIN ..= VRAM_END => {
                self.ppu.read_byte(addr)
            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {
                self.cartridge.read_byte(addr)
            }
            WRAM_BEGIN ..= WRAM_END => {
                self.wram[(addr - WRAM_BEGIN) as usize]
            }
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => {
                self.wram[(addr - ECHO_RAM_BEGIN) as usize]
            }
            OAM_BEGIN ..= OAM_END => {
                self.ppu.read_byte(addr)
            }
            INVALID_BEGIN ..= INVALID_END => {
                0xFF
            }
            IO_REGISTERS_BEGIN ..= IO_REGISTERS_END => {
                match addr {
                    OAM_DMA => {
                        self.oam_dma
                    }
                    _ => {
                        self.io_registers.read_byte(addr)
                    }
                }
            }
            HRAM_BEGIN ..= HRAM_END => {
                self.hram[(addr - HRAM_BEGIN) as usize]
            }
            IE_REGISTERS_BEGIN ..= IE_REGISTERS_END => {
                self.interrupt_controller.read_byte(addr)
            }
            _ => {
                0xFF
            }
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 
    {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr.wrapping_add(1));

        let mut data: u16 = 0;
        set_msb(&mut data, msb);
        set_lsb(&mut data, lsb);
        
        data
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) 
    {
        match addr {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                match addr {
                    BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                        // TODO: Current code might be wrong here, perhaps should be:
                        // self.cartridge.write_byte(addr, value);
                        if self.boot_rom_disable == 0x01 {
                            self.cartridge.write_byte(addr, value);
                        }
                    }
                    _ => {
                        self.cartridge.write_byte(addr, value);
                    }
                }
            }
            ROM_BANK_SWAP_BEGIN ..= ROM_BANK_SWAP_END => {
                self.cartridge.write_byte(addr, value);
            }
            VRAM_BEGIN ..= VRAM_END => {
                self.ppu.write_byte(addr, value);
            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {
                self.cartridge.write_byte(addr, value);
            }
            WRAM_BEGIN ..= WRAM_END => {
                self.wram[(addr - WRAM_BEGIN) as usize] = value;
            }
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => {
                self.wram[(addr - ECHO_RAM_BEGIN) as usize] = value;
            }
            OAM_BEGIN ..= OAM_END => {
                self.ppu.write_byte(addr, value);
            }
            INVALID_BEGIN ..= INVALID_END => {}
            IO_REGISTERS_BEGIN ..= IO_REGISTERS_END => {
                match addr {
                    BOOT_ROM_DISABLE => {
                        if self.boot_rom_disable == 0x00 {
                            self.boot_rom_disable = value & 0x01;
                        }
                    }
                    OAM_DMA => {
                        self.oam_dma = value;
                        let src_addr = (value as u16) << 8;
                        self.dma_write(src_addr);
                    }
                    _ => {
                        self.io_registers.write_byte(addr, value);
                    }
                }
            }
            HRAM_BEGIN ..= HRAM_END => {
                self.hram[(addr - HRAM_BEGIN) as usize] = value;
            }
            IE_REGISTERS_BEGIN ..= IE_REGISTERS_END => {
                self.interrupt_controller.write_byte(addr, value);
            }
            _ => {}
        }
    }

    pub fn write_word(&mut self, addr: u16, data: u16) 
    {
        let lsb = get_lsb(data);
        let msb = get_msb(data);

        self.write_byte(addr, lsb);
        self.write_byte(addr.wrapping_add(1), msb);
    }

    fn dma_write(&mut self, src_addr: u16)
    {
        // TODO: OAM_DMA should consume like 160 cycles and also lock out parts of memory
        let mut cur_addr = src_addr;
        let mut dst_addr = OAM_BEGIN;

        while dst_addr <= OAM_END {
            let data = self.read_byte(cur_addr);
            self.write_byte(dst_addr, data);

            cur_addr = cur_addr.wrapping_add(1);
            dst_addr = dst_addr.wrapping_add(1);
        }
    }
}
