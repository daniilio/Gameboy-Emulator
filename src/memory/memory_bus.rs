use crate::memory::ppu::PPU;
use crate::memory::io_registers::IO_Registers;
use crate::memory::rom::ROM;
use crate::cpu::interrupts::InterruptController;

struct MemoryBus {
    rom: ROM,
    ppu: PPU,
    cartridge_ram: [u8; CARTRIDGE_RAM_END - CARTRIDGE_RAM_BEGIN + 1],
    wram: [u8; WRAM_END - WRAM_BEGIN + 1],
    echo_ram: [u8; ECHO_RAM_END - ECHO_RAM_BEGIN + 1],
    io_registers: IO_REGISTERS,
    high_ram: [u8; HRAM_END - HRAM_BEGIN + 1],
    interrupt_controller: InterruptController,
}

impl MemoryBus {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                match addr {
                    BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                        if io_registers.is_boot_rom_mapped() {
                            self.boot_rom[addr as usize]
                        } else {
                            self.memory[addr as usize]
                        }
                    }
                    _ => {
                        self.memory[addr as usize]
                    }
                }
            }
            ROM_BANK_SWAP_BEGIN ..= ROM_BANK_SWAP_END => {

            }
            VRAM_BEGIN ..= VRAM_END => {
                ppu.read_vram(addr - VRAM_BEGIN)
            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {

            }
            WRAM_BEGIN ..= WRAM_END => {

            }
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => {

            }
            OAM_BEGIN ..= OAM_END => {

            }
            INVALID_BEGIN ..= INVALID_END => {

            }
            IO_REGISTERS_BEGIN ..= IO_REGISTERS_END => {
                self.memory[addr as usize]
            }
            HRAM_BEGIN ..= HRAM_END => {

            }
            IE_REGISTERS_BEGIN ..= IE_REGISTERS_END => {

            }
            _ => {
                self.memory[addr as usize]
            }
        }
    }

    fn read_word(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr.wrapping_add(1));

        let mut data: u16 = 0;
        set_msb(&mut data, msb);
        set_lsb(&mut data, lsb);
        
        data
    }

    fn write_byte(&mut self, data: u8, addr: u16) {
        match addr {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                match addr {
                    BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                        let addr = addr as usize;
                        // Never write to Boot ROM
                        if !io_registers.is_boot_rom_mapped() {
                            self.memory[addr] = data;
                        }
                    }
                    _ => {
                        let addr = addr as usize;
                        self.memory[addr] = data;
                    }
                }
            }
            ROM_BANK_SWAP_BEGIN ..= ROM_BANK_SWAP_END => {

            }
            VRAM_BEGIN ..= VRAM_END => {

            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {

            }
            WRAM_BEGIN ..= WRAM_END => {

            }
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => {

            }
            OAM_BEGIN ..= OAM_END => {

            }
            INVALID_BEGIN ..= INVALID_END => {

            }
            IO_REGISTERS_BEGIN ..= IO_REGISTERS_END => {
                self.io_registers.write_byte(data, addr);
                self.memory[addr as usize] = data;
            }
            HRAM_BEGIN ..= HRAM_END => {

            }
            IE_REGISTERS_BEGIN ..= IE_REGISTERS_END => {

            }
            _ => {
                self.memory[addr as usize] = data;
            }
        };
    }

    fn write_word(&mut self, data: u16, addr: u16) {
        let lsb = get_lsb(data);
        let msb = get_msb(data);

        self.write_byte(lsb, addr);
        self.write_byte(msb, addr.wrapping_add(1));
    }

    pub fn dma_write(&mut self, src_addr: u16)
    {
        // TODO: The OMA DMA controller uses a simplified docoding scheme. This
        // behaviour has not been mimiced in the hopes that it isn't important!
        let mut cur_addr = src_addr;
        let mut dst_addr = OAM_BEGIN;

        while dst_addr <= OAM_END {
            match cur_addr {
                0xA000 ..= 0xFFFF => {

                }
            }
            let data = self.read_byte(cur_addr);
            self.write_byte(data, dst_addr);
            cur_addr = cur_addr.wrapping_add(1);
            dst_addr = dst_addr.wrapping_add(1);
        }
    }
}
