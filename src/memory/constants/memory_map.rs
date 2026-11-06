// Memory map
const ROM_BANK_0_BEGIN:       u16 = 0x0000;
const ROM_BANK_0_END:         u16 = 0x3FFF;
const ROM_BANK_0_SIZE:        usize = (ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1) as usize;

const ROM_BANK_SWAP_BEGIN:    u16 = 0x4000;
const ROM_BANK_SWAP_END:      u16 = 0x7FFF;
const ROM_BANK_SWAP_SIZE:     usize = (ROM_BANK_SWAP_END - ROM_BANK_SWAP_BEGIN + 1) as usize;

const VRAM_BEGIN:             u16 = 0x8000;
const VRAM_END:               u16 = 0x9FFF;
const VRAM_SIZE:              usize = (VRAM_END - VRAM_BEGIN + 1) as usize;

const CARTRIDGE_RAM_BEGIN:    u16 = 0xA000;
const CARTRIDGE_RAM_END:      u16 = 0xBFFF;
const CARTRIDGE_RAM_SIZE:     usize = (CARTRIDGE_RAM_END - CARTRIDGE_RAM_BEGIN + 1) as usize;

const WRAM_BEGIN:             u16 = 0xC000;
const WRAM_END:               u16 = 0xDFFF;
const WRAM_SIZE:              usize = (WRAM_END - WRAM_BEGIN + 1) as usize;

const ECHO_RAM_BEGIN:         u16 = 0xE000;
const ECHO_RAM_END:           u16 = 0xFDFF;
const ECHO_RAM_SIZE:          usize = (ECHO_RAM_END - ECHO_RAM_BEGIN + 1) as usize;

const OAM_BEGIN:              u16 = 0xFE00;
const OAM_END:                u16 = 0xFE9F;
const OAM_SIZE:               usize = (OAM_END - OAM_BEGIN + 1) as usize;

const INVALID_BEGIN:          u16 = 0xFEA0;
const INVALID_END:            u16 = 0xFEFF;
const INVALID_SIZE:           usize = (INVALID_END - INVALID_BEGIN + 1) as usize;

const IO_REGISTERS_BEGIN:     u16 = 0xFF00;
const IO_REGISTERS_END:       u16 = 0xFF7F;
const IO_REGISTERS_SIZE:      usize = (IO_REGISTERS_END - IO_REGISTERS_BEGIN + 1) as usize;

const HRAM_BEGIN:             u16 = 0xFF80;
const HRAM_END:               u16 = 0xFFFE;
const HRAM_SIZE:              usize = (HRAM_END - HRAM_BEGIN + 1) as usize;

const IE_REGISTER_BEGIN:      u16 = 0xFFFF;
const IE_REGISTER_END:        u16 = 0xFFFF;
const IE_REGISTER_SIZE:       usize = (IE_REGISTER_END - IE_REGISTER_BEGIN + 1) as usize;
