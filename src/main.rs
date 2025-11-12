mod gpu;
mod config;
mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;
#[allow(dead_code)]
mod cpu_debugger;
mod clock;
mod cpu_core;

use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::cpu_core::CpuCore;
use crate::gpu::Gpu;
// use crate::cpu_debugger::CpuDebugger;

fn main() {
    let mut cpu = Cpu::new();
    let mut core = CpuCore::new();
    let mut gpu = Gpu::new("0xID8");
    let mut clock = Clock::new();
    // let mut cpu_debugger = CpuDebugger::new();

    core
        .load_rom(&mut cpu, "tests/test_opcode.ch8")
        .expect("File not found or not readable");

    if !cpu.panic {
        gpu.panic(&mut cpu);
    }

    while cpu.running {
        clock.start();
        core.tick(&mut cpu);
        gpu.tick(&mut cpu);
        // could be used in a thread to avoid using CPU time to draw cpu status
        // cpu_debugger.tick(&mut cpu).unwrap();
        clock.stop_and_wait();
    }

    // cpu_debugger.quit().unwrap();
}
