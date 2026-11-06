use std::thread;
use std::time::{Duration, Instant};
use std::fs;
use std::io::Read;
use std::fs::File;

use crate::cpu::CPU;

const CYCLES_PER_FRAME: u8 = 69905;
const FPS: u32 = 60;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS as u64);

/// Runs the emulator loop.
pub fn run(cpu: &mut CPU) 
{
    loop {
        let frame_start = Instant::now();
        
        // Perform CYCLES_PER_FRAME cycles of execution
        step_frame(&mut cpu);

        // Ensure we render the frame only after FRAME_TIME has elapsed
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_TIME {
            thread::sleep(FRAME_TIME - elapsed);
        }

        // Render the screen
        RenderScreen();
    }
}

/// Performs instruction, timer, graphics, and interrupt execution in sync for 
/// one frame (i.e. Performs CYCLES_PER_FRAME cycles for each type of execution).
///
/// This function should be called at the same frame rate as the gameboy (60 fps).
fn step_frame(cpu: &mut CPU) 
{
    let mut cycle_count = 0;
    while cycle_count < CYCLES_PER_FRAME {
        let cycles = cpu.execute();
        UpdateTimers(cycles);
        UpdateGraphics(cycles);
        DoInterrupts();
        cycle_count += cycles;
    }
}
