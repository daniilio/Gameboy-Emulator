// Persistent cartridge RAM
const PERSISTENT_RAM_DIR: Path = Path::new("games/saves/");
const SAVE_INTERVAL: Duration = Duration::from_millis(30_000);

// Cartridge header addresses
const CARTRIDGE_TYPE: u16 = 0x0147;
const ROM_SIZE: u16 = 0x0148;
const RAM_SIZE: u16 = 0x0149;

// Bank sizes
const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

pub trait MemoryBankController {
    fn read_byte(&self, addr: u16) -> u8;

    fn write_byte(&mut self, addr: u16, value: u8);

    fn new(
        game_name: String, 
        shutdown: &Arc<AtomicBool>, 
        cartridge_rom: Vec<u8>, 
        cartridge_ram: Vec<u8>,
    ) -> Box<dyn MemoryBankController>
    {
        let battery_backed: bool;

        let cartridge_type = cartridge_rom[CARTRIDGE_TYPE];
        match cartridge_type {
            ROM_ONLY => {
                // TODO:
            }
            MBC1 | MBC1_RAM => {
                battery_backed = false;
                Box::new(MemoryBankController1::new(
                    game_name,
                    shutdown,
                    battery_backed,
                    cartridge_rom,
                    cartridge_ram,
                ))
            }
            MBC1_RAM_BATTERY => {
                battery_backed = true;
                Box::new(MemoryBankController1::new(
                    game_name,
                    shutdown,
                    battery_backed,
                    cartridge_rom,
                    cartridge_ram,
                ))
            }
            _ => {
                panic!("Cartridge type not implemented: {cartridge_type:#04X}");
            }
        }
    }

    /// Spins-up a thread that periodically persists cartridge RAM to memory.
    fn battery_back(&self,)
    {
        let thread_shutdown = Arc::clone(shutdown);
        let save_thread = thread::spawn(|| {
            while !thread_shutdown.load(Ordering::Relaxed) {
                thread::sleep(SAVE_INTERVAL);
                if let Err(err) = self.persist_ram() {
                    eprintln!("Failed to persist cartridge RAM: {}", err);
                }
            }
        });
    }
}

pub trait BatteryBackedRAM {
    /// Spawns a thread that continuously backs up RAM to save file.
    fn start_persist_ram(&mut self, shutdown: &Arc<AtomicBool>)
    {
        if !self.battery_backed {
            return;
        }
        
        let thread_shutdown = Arc::clone(shutdown);
        let cartridge_ram = Arc::clone(&self.cartridge_ram);
        let save_path = PERSISTENT_RAM_DIR.join(format!("{}.sav", self.game_name));

        self.save_thread = thread::spawn(move || {
            while !thread_shutdown.load(Ordering::Relaxed) {
                thread::sleep(SAVE_INTERVAL);
                
                let ram = cartridge_ram.lock().unwrap();
                if let Err(err) = io::write(save_path, &ram) {
                    eprintln!("Failed to persist cartridge RAM: {err}");
                }
            }
        });
    }
}
