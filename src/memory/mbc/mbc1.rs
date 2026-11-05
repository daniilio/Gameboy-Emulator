use std::fs;
use std::io;
use std::thread;

use crate::memory::mbc::mbc::MemoryBankController;

// MBC1 control registers (write-only)
const RAM_ENABLE_BEGIN:       u16 = 0x0000;
const RAM_ENABLE_END:         u16 = 0x1FFF;
const ROM_BANK_NUMBER_BEGIN:  u16 = 0x2000;
const ROM_BANK_NUMBER_END:    u16 = 0x3FFF;
const RAM_BANK_NUMBER_BEGIN:  u16 = 0x4000;
const RAM_BANK_NUMBER_END:    u16 = 0x5FFF;
const BANK_MODE_SELECT_BEGIN: u16 = 0x6000;
const BANK_MODE_SELECT_END:   u16 = 0x7FFF;

pub struct MemoryBankController1 {
    game_name: String,
    save_thread: Option<JoinHandle<()>>,

    // MBC control registers
    ram_enabled: bool,
    bank_mode: bool,
    rom_bank_select: u8,
    upper_bank_select: u8,
    
    // Header data
    battery_backed: bool,

    // External
    cartridge_rom: Vec<u8>,
    cartridge_ram: Arc<Mutex<Vec<u8>>>,
}

impl MemoryBankController1 {
    pub fn new(
        game_name: String, 
        shutdown: &Arc<AtomicBool>,
        battery_backed: Bool,
        cartridge_rom: Vec<u8>, 
        cartridge_ram: Vec<u8>
    ) -> Self
    {
        let cartridge_ram = Arc::new(Mutex::new(cartridge_ram));
        
        // Spin up battery backed RAM thread
        let thread_shutdown = Arc::clone(shutdown);
        let save_thread = thread::spawn(|| {
            while !thread_shutdown.load(Ordering::Relaxed) {
                thread::sleep(SAVE_INTERVAL);
                if let Err(err) = self.persist_ram() {
                    eprintln!("Failed to persist cartridge RAM: {}", err);
                }
            }
        });

        let mut mbc = Self {
            game_name,
            save_thread,

            ram_enabled: false,
            bank_mode: false,
            rom_bank_select: 0x01,
            upper_bank_select: 0x00,

            battery_backed,

            cartridge_rom,
            cartridge_ram,
        };

        mbc
    }

    /// Returns a bitmask for the ROM bank number, based on the cartridge
    /// ROM size.
    /// Note that a non-power-of-two ROM size may include bank numbers 
    /// that do not correspond to a valid bank.
    fn rom_bank_mask(&self) -> usize 
    {
        let mut bank_count = self.cartridge_rom.len() / ROM_BANK_SIZE;
        if self.cartridge_rom.len() % ROM_BANK_SIZE != 0 {
            bank_count += 1;
        }
        bank_count.next_power_of_two() - 1
    }
}

impl MemoryBankController for MemoryBankController1 {
    // TODO: Parse the cartridge header
    // TODO: Add support for 1 MiB multi-game compilation carts
    
    fn read_byte(&self, addr: u16) -> u8
    {
        match addr {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                let cart_addr: usize;
                if self.bank_mode {
                    let mut rom_bank = (self.upper_bank_select as usize) << 5;
                    rom_bank &= self.rom_bank_mask();

                    cart_addr = (rom_bank << 14) | (addr as usize);
                } else {
                    cart_addr = (addr as usize);
                }
                
                if cart_addr < self.cartridge_rom.len() {
                    self.cartridge_rom[cart_addr]
                } else {
                    0xFF
                }
            }
            ROM_BANK_SWAP_BEGIN ..= ROM_BANK_SWAP_END => {
                let mut rom_bank = (self.rom_bank_select as usize);
                rom_bank |= (self.upper_bank_select as usize) << 5;
                rom_bank &= self.rom_bank_mask();
                
                let mut cart_addr = (rom_bank << 14);
                // Cartridge ROM has 14-bit address bus
                cart_addr |= (addr as usize) & 0x3FFF;
                
                if cart_addr < self.cartridge_rom.len() {
                    self.cartridge_rom[cart_addr]
                } else {
                    0xFF
                }
            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                
                let mut cart_addr: usize = 0;
                if self.bank_mode {
                    cart_addr = (self.upper_bank_select as usize) << 13;
                }
                // Cartridge RAM has 13-bit address bus
                cart_addr |= (addr as usize) & 0x1FFF;
                
                let ram = self.cartridge_ram.lock().unwrap();
                if cart_addr < ram.len() {
                    ram[cart_addr]
                } else {
                    0xFF
                }
            }
            _ => 0xFF
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8)
    {
        match addr {
            RAM_ENABLE_BEGIN ..= RAM_ENABLE_END => {
                let value = value & 0x0F;
                self.ram_enabled = value == 0x0A;
            }
            ROM_BANK_NUMBER_BEGIN ..= ROM_BANK_NUMBER_END => {
                // Mask 5 bits to check if specified bank id is 0
                let mut value = value & 0x1F;
                if value == 0x00 {
                    value = 0x01;
                }
                self.rom_bank_select = value;
            }
            RAM_BANK_NUMBER_BEGIN ..= RAM_BANK_NUMBER_END => {
                self.upper_bank_select = value & 0x03;
            }
            BANK_MODE_SELECT_BEGIN ..= BANK_MODE_SELECT_END => {
                self.bank_mode = (value & 0x1) == 1;
            }
            CARTRIDGE_RAM_BEGIN ..= CARTRIDGE_RAM_END => {
                if !self.ram_enabled {
                    return;
                }
                
                let mut cart_addr: usize = 0;
                if self.bank_mode {
                    cart_addr = (self.upper_bank_select as usize) << 13;
                }
                // Cartridge RAM has 13-bit address bus
                cart_addr |= (addr as usize) & 0x1FFF;
                
                let mut ram = self.cartridge_ram.lock().unwrap();
                if cart_addr < ram.len() {
                    ram[cart_addr] = value;
                }
            }
            _ => {}
        }
    }
}
