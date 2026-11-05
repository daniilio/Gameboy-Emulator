const game_path: PathBuf = PathBuf::from("/games/roms/");
const saves_path: PathBuf = PathBuf::from("/games/saves/");

struct Cartridge {
    mbc: MemoryBankController,
}

impl Cartridge {
    fn new(
        game_name: &str,
    ) -> Self
    {
        load_game_rom()
    }

    /// Loads the game ROM into memory.
    pub fn load_game_rom(game_name: &str) -> Vec<u8>
    {   
        
        let mut file = File::open(&game_path).expect("ERROR: Failed to open game ROM");
        let mut cartridge_memory = [0u8; CARTRIDGE_SIZE];
        cartridge_rom
        let mut bytes_read: usize = 0;
        while bytes_read < CARTRIDGE_SIZE {
            let b = file.read(&mut cartridge_memory[bytes_read..]).expect("ERROR: Failed to read game ROM");
            if b == 0 {
                // EOF
                break;
            }
            bytes_read += b;
        }
        drop(file);

        cartridge_rom
    }

    pub fn load_game_ram(game_name: )
}
