// Memory map
pub const ROM_BANK_0_BEGIN:       u16 = 0x0000;
pub const ROM_BANK_0_END:         u16 = 0x3FFF;
pub const ROM_BANK_0_SIZE:        usize = (ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1) as usize;

pub const ROM_BANK_SWAP_BEGIN:    u16 = 0x4000;
pub const ROM_BANK_SWAP_END:      u16 = 0x7FFF;
pub const ROM_BANK_SWAP_SIZE:     usize = (ROM_BANK_SWAP_END - ROM_BANK_SWAP_BEGIN + 1) as usize;

pub const VRAM_BEGIN:             u16 = 0x8000;
pub const VRAM_END:               u16 = 0x9FFF;
pub const VRAM_SIZE:              usize = (VRAM_END - VRAM_BEGIN + 1) as usize;

pub const CARTRIDGE_RAM_BEGIN:    u16 = 0xA000;
pub const CARTRIDGE_RAM_END:      u16 = 0xBFFF;
pub const CARTRIDGE_RAM_SIZE:     usize = (CARTRIDGE_RAM_END - CARTRIDGE_RAM_BEGIN + 1) as usize;

pub const WRAM_BEGIN:             u16 = 0xC000;
pub const WRAM_END:               u16 = 0xDFFF;
pub const WRAM_SIZE:              usize = (WRAM_END - WRAM_BEGIN + 1) as usize;

pub const ECHO_RAM_BEGIN:         u16 = 0xE000;
pub const ECHO_RAM_END:           u16 = 0xFDFF;
pub const ECHO_RAM_SIZE:          usize = (ECHO_RAM_END - ECHO_RAM_BEGIN + 1) as usize;

pub const OAM_BEGIN:              u16 = 0xFE00;
pub const OAM_END:                u16 = 0xFE9F;
pub const OAM_SIZE:               usize = (OAM_END - OAM_BEGIN + 1) as usize;

pub const INVALID_BEGIN:          u16 = 0xFEA0;
pub const INVALID_END:            u16 = 0xFEFF;
pub const INVALID_SIZE:           usize = (INVALID_END - INVALID_BEGIN + 1) as usize;

pub const IO_REGISTERS_BEGIN:     u16 = 0xFF00;
pub const IO_REGISTERS_END:       u16 = 0xFF7F;
pub const IO_REGISTERS_SIZE:      usize = (IO_REGISTERS_END - IO_REGISTERS_BEGIN + 1) as usize;

pub const HRAM_BEGIN:             u16 = 0xFF80;
pub const HRAM_END:               u16 = 0xFFFE;
pub const HRAM_SIZE:              usize = (HRAM_END - HRAM_BEGIN + 1) as usize;

pub const IE_REGISTER_BEGIN:      u16 = 0xFFFF;
pub const IE_REGISTER_END:        u16 = 0xFFFF;
pub const IE_REGISTER_SIZE:       usize = (IE_REGISTER_END - IE_REGISTER_BEGIN + 1) as usize;
