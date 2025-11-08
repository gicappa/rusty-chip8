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
mod cpu_core;

use crate::cpu::Cpu;
use crate::gpu::Gpu;

use crate::clock::Clock;
use crate::cpu_core::CpuCore;

fn main() {
    let mut cpu = Cpu::new();
    let mut core = CpuCore::new(&mut cpu);
    let mut gpu = Gpu::new();
    let mut clock = Clock::new();

    loop {
        clock.start();
        core.clk();
        gpu.clk(
            core.cpu.vram,
            core.draw_flag());
        clock.stop_and_wait();
    }

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
}
