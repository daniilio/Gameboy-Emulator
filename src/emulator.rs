use std::thread;
use std::time::{Duration, Instant};

const CYCLES_PER_FRAME: u8 = 69905;
const FPS: u32 = 60;
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

impl Emulator {
    fn run() {
        loop {
            let frame_start = Instant::now();
            
            Self::update();

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_TIME {
                thread::sleep(FRAME_TIME - elapsed);
            }
        }
    }
    
    // we need to call this function 60 times every second
    fn update() {
        let mut cycle_count = 0;
        while cycle_count < CYCLES_PER_FRAME {
            let cycles = ExecuteNextOpcode();
            UpdateTimers(cycles);
            UpdateGraphics(cycles);
            DoInterrupts();
            cycle_count += cycles;
        }
        RenderScreen();
    }
}
