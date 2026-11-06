// Game ROM and battery-backed RAM
const GAME_PATH: PathBuf = PathBuf::from("/games/roms/");
const SAVES_PATH: PathBuf = PathBuf::from("games/saves/");
const SAVE_FREQ: Duration = Duration::from_millis(10_000);

// Cartridge header addresses
const CARTRIDGE_TYPE: u16 = 0x0147;
const ROM_SIZE: u16 = 0x0148;
const RAM_SIZE: u16 = 0x0149;

pub struct Cartridge {
    // Thread handle for battery-backed RAM saves
    save_handle: Option<JoinHandle<()>>,
    // Thread shutdown signal
    shutdown_signal: Arc<(Mutex<bool>, Condvar)>,

    mbc: Box<dyn MemoryBankController>,
}

impl Cartridge {
    pub fn new(
        game_name: &str,
    ) -> Self
    {
        // Read in the ROM file
        let rom_data = match Self::load_game_rom(&game_name) {
            Ok(rom) => rom,
            Err(err) => {
                panic!("Failed to load ROM: {err:?}");
            }
        };
        let rom_size = get_rom_size(rom_data[ROM_SIZE as usize]);
        assert_eq!(rom_data.len(), rom_size);

        // Read in the RAM save file if it exists
        let mut ram_data = match Self::load_game_ram(&game_name) {
            Ok(ram) => ram,
            Err(err) => {
                panic!("Failed to load RAM: {err:?}");
            }
        };
        
        // Read cartridge type from cartridge header
        let cart_type = rom_data[CARTRIDGE_TYPE as usize];
        let (has_ram, battery_backed) = match cart_type {
            ROM_ONLY => (false, false),
            ROM_RAM => (true, false),
            ROM_RAM_BATTERY => (true, true),
            MBC1 => (false, false),
            MBC1_RAM => (true, false),
            MBC1_RAM_BATTERY => (true, true),
            _ => panic!("Unsupported cartridge type: {cart_type:#04X}"),
        };

        // Initialize a RAM vector if no save file was found
        if has_ram {
            let ram_size = get_ram_size(rom_data[RAM_SIZE as usize]);
            if ram_data.len() == 0 {
                ram_data = vec![0x00u8; ram_size];
            }
            assert_eq!(ram_data.len(), ram_size);
        }
        // Battery backed RAM and MBC touch cartridge RAM
        let cart_ram = Arc::new(Mutex::new(ram_data));

        // Construct the correct MBC
        let mbc = match cart_type {
            ROM_ONLY | ROM_RAM | ROM_RAM_BATTERY => {
                Box::new(ROMOnly::new(
                    rom_data,
                    Arc::clone(&cart_ram),
                ))
            }
            MBC1 | MBC1_RAM | MBC1_RAM_BATTERY => {
                Box::new(MemoryBankController1::new(
                    rom_data,
                    Arc::clone(&cart_ram),
                ))
            }
            _ => {
                panic!("Unsupported cartridge type: {cart_type:#04X}");
            }
        };

        // Initialize shutdown signal
        let shutdown_signal = Arc::new((Mutex::new(false), Condvar::new()));

        // Start RAM save thread if battery backed
        let save_handle = if battery_backed {
            match Self::start_save_ram(&game_name, Arc::clone(&shutdown_signal), Arc::clone(&cart_ram)) {
                Ok(handle) => {
                    Some(handle)
                }
                Err(err) => {
                    eprintln!("Failed to spawn cartridge RAM save thread: {err:?}");
                    None
                }
            }
        } else {
            None
        };

        Self {
            save_handle,
            shutdown_signal,
            mbc,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8
    {
        self.mbc.read_byte(addr)
    }

    pub fn write_byte(&mut self, addr: u16, value: u8)
    {
        self.mbc.write_byte(addr, value);
    }

    /// Loads the game ROM into memory.
    fn load_game_rom(game_name: &str) -> io::Result<Vec<u8>>
    {   
        let rom_path = GAME_PATH.join(format!("{game_name}.rom"));
        let rom_data = fs::read(&rom_path)?;
        Ok(rom_data)
    }

    /// Loads the game RAM into memory if it's save file is present.
    fn load_game_ram(game_name: &str) -> io::Result<Vec<u8>>
    {
        let ram_path = SAVES_PATH.join(format!("{game_name}.sav"));
        let ram_data = if ram_path.try_exists()? {
            fs::read(ram_path)?
        } else {
            Vec::new()
        };
        Ok(ram_data)
    }

    /// Spawns a thread that continuously backs up RAM to save file.
    fn start_save_ram(
        game_name: &str,
        shutdown_signal: Arc<(Mutex<bool>, Condvar)>,
        cartridge_ram: Arc<Mutex<Vec<u8>>>,
    ) -> Result<JoinHandle<()>, io::Error>
    {
        let save_path = SAVES_PATH.join(format!("{game_name}.sav"));

        thread::Builder::new().spawn(move || {
            let (lock, cvar) = &*shutdown_signal;
            loop {
                // Wait SAVE_FREQ ms or shutdown signal
                let shutdown = lock.lock().unwrap();
                let (shutdown, _) = cvar.wait_timeout_while(shutdown, SAVE_FREQ, |shutdown| !*shutdown).unwrap();
                let should_shutdown = *shutdown;
                drop(shutdown);

                // Persist RAM
                let ram = cartridge_ram.lock().unwrap();
                if let Err(err) = fs::write(&save_path, ram.as_slice()) {
                    eprintln!("Failed to persist cartridge RAM: {err:?}");
                }
                drop(ram);
                
                if (should_shutdown) {
                    break;
                }
            }
        })
    }
}

impl Drop for Cartridge {
    /// Cleans up RAM save thread.
    fn drop(&mut self)
    {
        if let Some(save_handle) = self.save_handle.take() {
            // Set shutdown flag to terminate save thread
            let (lock, cvar) = &*self.shutdown_signal;
            let mut shutdown = lock.lock().unwrap();
            *shutdown = true;
            drop(shutdown);
            cvar.notify_one();

            // Clean up save thread
            if let Err(err) = save_handle.join() {
                eprintln!("Save thread panicked: {err:?}");
            }
        }
    }
}
