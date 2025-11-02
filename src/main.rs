mod chip8;
mod chip8_op0nnn;
mod chip8_op8xyz;
mod chip8_opannn;
mod chip8_opfx07;
mod gpu;

use std::sync::mpsc;
use crate::chip8::Chip8;
use crate::chip8::{HEIGHT, WIDTH};
use crate::gpu::GPU;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    println!("Chip8 Emulator Starting...");

    let mut chip8 = Chip8::new();
    let (tx, rx) = mpsc::channel::<[u8; 2048]>();
    let mut gpu: GPU<WIDTH, HEIGHT> = GPU::new(rx);
    let _handle = thread::spawn(move || {
        // Target ~60 Hz for timers / display refresh.
        let interval = Duration::from_micros(16_666); // 1_000_000 / 60 ≈ 16_666.67
        let mut last_time = Instant::now();

        loop {
            // Execute one CPU instruction (or whatever `step()` encapsulates).
            chip8.step();

            // Handle drawing if the emulator signalled it.
            if chip8.draw_flag() {
                let frame = chip8.display;
                let _ = tx.send(frame);
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
    });

    gpu.start();

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
    let _ = _handle.join().unwrap();
}
