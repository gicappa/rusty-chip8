use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::config::VRAM;
use crate::cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu,
    tx: Sender<VRAM>,
}

impl Chip8 {
    pub fn new(cpu: Cpu, tx: Sender<VRAM>) -> Self {
        Self {
            cpu,
            tx,
        }
    }

    pub fn start(&mut self) {
        // Target ~60 Hz for timers / display refresh.
        // 1_000_000 / 60 ≈ 16_666.67
        let interval = Duration::from_micros(16_666);
        let mut last_time = Instant::now();

        loop {
            // Execute one CPU cycle
            self.cpu.clk();

            // Handle drawing if the emulator signalled it.
            if self.cpu.draw_flag() {
                let frame = self.cpu.vram;
                let _ = self.tx.send(frame);
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
