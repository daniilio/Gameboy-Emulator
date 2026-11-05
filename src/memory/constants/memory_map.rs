// Memory map
const ROM_BANK_0_BEGIN:       u16 = 0x0000;
const ROM_BANK_0_END:         u16 = 0x3FFF;

const ROM_BANK_SWAP_BEGIN:    u16 = 0x4000;
const ROM_BANK_SWAP_END:      u16 = 0x7FFF;

const VRAM_BEGIN:             u16 = 0x8000;
const VRAM_END:               u16 = 0x9FFF;

const CARTRIDGE_RAM_BEGIN:    u16 = 0xA000;
const CARTRIDGE_RAM_END:      u16 = 0xBFFF;

const WRAM_BEGIN:             u16 = 0xC000;
const WRAM_END:               u16 = 0xDFFF;

const ECHO_RAM_BEGIN:         u16 = 0xE000;
const ECHO_RAM_END:           u16 = 0xFDFF;

const OAM_BEGIN:              u16 = 0xFE00;
const OAM_END:                u16 = 0xFE9F;

const INVALID_BEGIN:          u16 = 0xFEA0;
const INVALID_END:            u16 = 0xFEFF;

const IO_REGISTERS_BEGIN:     u16 = 0xFF00;
const IO_REGISTERS_END:       u16 = 0xFF7F;

const HRAM_BEGIN:             u16 = 0xFF80;
const HRAM_END:               u16 = 0xFFFE;

const IE_REGISTER_BEGIN:      u16 = 0xFFFF;
const IE_REGISTER_END:        u16 = 0xFFFF;
