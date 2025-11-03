mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;
mod gpu;
mod chip8;
mod config;
mod debug_cli;

use std::sync::{mpsc, Arc};
use std::thread;

use crate::chip8::Chip8;
use crate::cpu::Cpu;
use crate::gpu::Gpu;
use crate::config::VRAM;
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

