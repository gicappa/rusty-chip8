mod gpu;
mod config;
mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;
#[allow(dead_code)]
mod debug_cli;
mod clock;
mod cpu_engine;

use crate::config::VRAM;
use crate::cpu::Cpu;
use crate::gpu::Gpu;

use crate::clock::Clock;
use crate::cpu_engine::CpuEngine;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<VRAM>();

    let mut cpu = Cpu::new();
    let mut cpu_engine = CpuEngine::new(&mut cpu);
    let mut gpu = Gpu::new(rx);
    let mut clock = Clock::new();

    loop {
        clock.start();
        cpu_engine.clk();

        if cpu_engine.draw_flag() {
            let _ = tx.send(cpu_engine.cpu.vram);
        }

        clock.stop_and_wait();
        gpu.clk();
    }

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
}
