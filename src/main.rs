//! Entry point for the Rusty CHIP-8 emulator binary.
//!
//! Responsibilities handled here:
//! - Instantiate the `Chip8` VM.
//! - Load a ROM file (currently hard-coded as `rom.ch8`).
//! - Reset / initialise memory state.
//! - Run the main emulation loop: CPU step, timers, drawing, sound.
//! - Pace execution to (approximately) 60 Hz for timers / display.
//!
//! Future improvements that could live here:
//! - Command-line argument parsing for ROM path.
//! - Configurable clock speeds (CPU vs timer frequency).
//! - Proper sound generation instead of `println!("beep!")`.
//! - Graceful exit on key press / window close.

mod chip8;
mod chip8_op8xyz;
mod chip8_op1nnn;

use crate::chip8::Chip8;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Main entry point.
///
/// Flow:
/// 1. Create a new `Chip8` instance.
/// 2. Load the ROM bytes from `rom.ch8` into memory starting at the conventional 0x200 address.
/// 3. Optionally reset memory (currently clears it after loading; you likely want to remove `reset_memory()` if it wipes the ROM region).
/// 4. Enter an infinite loop performing one CPU step per iteration.
/// 5. If the draw flag is set, invoke display rendering.
/// 6. Decrement delay and sound timers at ~60 Hz. When the sound timer hits zero after being positive, print a placeholder beep.
/// 7. Sleep the remainder of the 60 Hz frame period so timers approximate real time.
///
/// Timing:
/// - `interval` is ~16.666 ms (1/60 s) expressed in microseconds.
/// - We measure elapsed time for each loop; if shorter than the desired interval we sleep the difference.
/// - If longer, we skip sleeping (frame is late) and continue immediately.
///
/// NOTE: `reset_memory()` after loading a ROM may erase the just loaded program depending on its implementation. If it clears the entire memory array you should move the call before `load_rom()` or remove it.
fn main() {
    println!("Chip8 Emulator Starting...");
    let mut chip8 = Chip8::new();

    chip8.reset_memory(); // Should this be included in the Chip8::new()?
    chip8
        .load_rom("rom.ch8")
        .expect("File not found or not readable");


    // Target ~60 Hz for timers / display refresh.
    let interval = Duration::from_micros(16_666); // 1_000_000 / 60 ≈ 16_666.67
    let mut last_time = Instant::now();

    loop {
        // Execute one CPU instruction (or whatever `step()` encapsulates).
        chip8.step();

        // Handle drawing if the emulator signalled it.
        if chip8.draw_flag() {
            chip8.draw_display();
        }

        // Update timers (CHIP-8 spec: they decrement at 60 Hz when > 0).
        if chip8.delay_timer > 0 {
            chip8.delay_timer -= 1;
        }

        if chip8.sound_timer > 0 {
            chip8.sound_timer -= 1;
            if chip8.sound_timer == 0 {
                println!("beep!"); // Placeholder for actual audio.
            }
        }

        // Frame pacing: sleep remaining time this frame if we are ahead of schedule.
        let now = Instant::now();
        let elapsed = now - last_time;
        if elapsed < interval {
            sleep(interval - elapsed);
            // Advance last_time by exactly one interval to reduce drift from sleep inaccuracies.
            last_time += interval;
        } else {
            // We're late; resync to now instant to prevent spiral of death.
            last_time = now;
        }
    }
}
