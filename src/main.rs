use std::fs;
use std::io::Read;
use std::fs::File;

const CARTRIDGE_SIZE: usize = 0x200000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("ERROR: No game ROM provided");
        return;
    }

    let mut cartridge_memory = [0u8; CARTRIDGE_SIZE];

    let game_path = &args[1];
    let mut file = File::open(&game_path).expect("ERROR: Failed to open game ROM");
    
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
