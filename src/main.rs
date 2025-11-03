mod gpu;
mod chip8;
mod config;
#[allow(dead_code)]
mod debug_cli;
mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;

use std::sync::mpsc;
use std::thread;

use crate::chip8::Chip8;
use crate::config::VRAM;
use crate::cpu::Cpu;
use crate::gpu::Gpu;
// use crate::debug_cli::DebugCli;

fn main() {
    let (tx, rx) = mpsc::channel::<VRAM>();

    let cpu = Cpu::new();
    let mut gpu: Gpu = Gpu::new(rx);

    // let debug_cli = DebugCli::new(&cpu);
    // debug_cli.start();

    let handle = thread::spawn(move || {
        let mut chip8 = Chip8::new(cpu, tx);
        chip8.start();
    });

    gpu.start();

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
    let _ = handle.join().unwrap();
}

