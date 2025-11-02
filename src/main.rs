mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;
mod gpu;

use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::cpu::CPU;
use crate::gpu::GPU;
use crate::gpu::PIXELS;

struct Chip8 {
    cpu: CPU,
    tx :Sender<[u8; PIXELS]>
}

impl Chip8 {
    pub fn new(cpu: CPU, tx :Sender<[u8; PIXELS]>) -> Self {
        Self {
            cpu,
            tx
        }
    }

    pub fn start(&mut self) {
        // Target ~60 Hz for timers / display refresh.
        // 1_000_000 / 60 ≈ 16_666.67
        let interval = Duration::from_micros(16_666);
        let mut last_time = Instant::now();

        loop {
            // Execute one CPU instruction (or whatever `step()` encapsulates).
            self.cpu.step();

            // Handle drawing if the emulator signalled it.
            if self.cpu.draw_flag() {
                let frame = self.cpu.display;
                let _ = self.tx.send(frame);
            }

            // Update timers (CHIP-8 spec: they decrement at 60 Hz when > 0).
            if self.cpu.delay_timer > 0 {
                self.cpu.delay_timer -= 1;
            }

            if self.cpu.sound_timer > 0 {
                self.cpu.sound_timer -= 1;
                if self.cpu.sound_timer == 0 {
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
}


fn main() {
    println!("Chip8 Emulator Starting...");

    let (tx, rx) = mpsc::channel::<[u8; PIXELS]>();
    let cpu = CPU::new();
    let mut gpu: GPU = GPU::new(rx);

    let _handle = thread::spawn(move || {
        let mut chip8 = Chip8::new(cpu, tx);
        chip8.start();
    });

    gpu.start();

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
    let _ = _handle.join().unwrap();
}
