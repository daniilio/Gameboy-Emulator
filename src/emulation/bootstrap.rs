use std::

/// Loads the game ROM into memory.
pub fn load_game_rom(game_path: &PathBuf) 
{   
    let mut file = File::open(&game_path).expect("ERROR: Failed to open game ROM");
    // TODO: We need to map to the actual memory struct
    let mut cartridge_memory = [0u8; CARTRIDGE_SIZE];
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
}

/// Initialize registers.
pub fn init(cpu: &mut CPU)
{
    // TODO: Set cartridge memory to 0xFF
    // TODO: Set all of memory to correct values
    // TODO: Set registers to correct value


    cpu.regs
}
