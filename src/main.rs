mod emulator;
mod execution;
mod graphics;
mod instructions;
mod memory;
mod registers;

use std::fs;
use std::io::Read;
use std::fs::File;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering}
}

use crate::emulator::{load_rom, run}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("ERROR: No game ROM provided");
        return;
    }
    let game_path = PathBuf::from(&args[1]);
    load_rom(&game_path);

    let game_name = game_path
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("Invalid game file name");

    // TODO: Cleanup battery backed RAM thread on shutdown
    let shutdown = Arc::new(AtomicBool::new(false));
    

    run();
}
