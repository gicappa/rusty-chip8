mod gpu;
mod config;
mod cpu;
mod cpu_opcode;
#[allow(dead_code)]
mod cpu_debugger;
mod clock;
mod cpu_core;
mod video_input;

use clap::Parser;
use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::cpu_core::CpuCore;
use crate::cpu_debugger::CpuDebugger;
// use crate::gpu::Gpu;
use crate::video_input::VideoInput;

#[derive(Parser, Debug)]
#[command(
    name = "oxide",
    version,
    about = "A Chip8 Emulator written in Rust"
)]
struct Args {
    rom_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut cpu = Cpu::new();
    let mut core = CpuCore::new();
    // let mut gpu = Gpu::new("0xID8");
    let mut clock = Clock::new();
    let mut cpu_debugger = CpuDebugger::new();
    let _ = VideoInput::start();

    args.rom_file.map(|r| {
        core
            .load_rom(&mut cpu, &r)
            .expect(&format!("File {} not found or not readable", &r));
    });

    if !cpu.panic {
        // gpu.panic(&mut cpu);
    }

    while cpu.running {
        clock.start();
        core.tick(&mut cpu);
        // gpu.tick(&mut cpu);
        // could be used in a thread to avoid using CPU time to draw cpu status
        cpu_debugger.tick(&mut cpu).unwrap();
        clock.stop_and_wait();
    }

    cpu_debugger.quit().unwrap();
}
