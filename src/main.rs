mod gpu;
mod config;
mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;
#[allow(dead_code)]
mod debug_cli;

use crate::config::VRAM;
use crate::cpu::Cpu;
use crate::gpu::Gpu;

use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
// use crate::debug_cli::DebugCli;

fn main() {
    let (tx, rx) = mpsc::channel::<VRAM>();

    let mut gpu = Gpu::new(rx);

    // let debug_cli = DebugCli::new(&cpu);
    // debug_cli.start();

    let handle = thread::spawn(move || {
        let mut cpu = Cpu::new();

        // let interval = Duration::from_micros(16_666);
        let interval = Duration::from_secs(1);
        let mut last_time = Instant::now();

        loop {
            cpu.clk();

            if cpu.draw_flag() {
                let frame = cpu.vram;
                let _ = tx.send(frame);
            }

            let now = Instant::now();
            let elapsed = now - last_time;
            if elapsed < interval {
                sleep(interval - elapsed);
                last_time += interval;
            } else {
                last_time = now;
            }
        }
    });

    gpu.start();

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
    let _ = handle.join().unwrap();
}
