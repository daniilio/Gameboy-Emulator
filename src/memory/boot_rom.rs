// Boot ROM
const BOOT_PATH: PathBuf = PathBuf::from("/boot_rom/");
const BOOT_ROM_NAME: &str = "dmg_boot.bin";

pub struct BootRom {
    boot_rom: [u8; BOOT_ROM_SIZE],
}

impl BootRom {
    pub fn new() -> Self
    {
        // TODO: Complete constructor which must load in boot ROM
    }

    pub fn read_byte(&self, addr: u16) -> u8
    {
        match addr {
            BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                self.boot_rom[addr as usize]
            }
            _ => 0xFF
        }
    }

    pub fn write_byte(&mut self, _: u16, _: u8) {}

    /// Load the boot ROM into memory.
    fn load_boot_rom() -> io::Result<[u8; BOOT_ROM_SIZE]>
    {
        let rom_path = BOOT_PATH.join(BOOT_ROM_NAME);        
        let mut rom_file = File::open(rom_path)?;

        let mut rom = [0x00u8; BOOT_ROM_SIZE];
        rom_file.read_exact(&mut rom)?;

        Ok(rom)
    }
}