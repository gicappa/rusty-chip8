mod gpu;
mod config;
mod cpu;
mod cpu_opcode;
#[allow(dead_code)]
mod cpu_debugger;
mod clock;
mod cpu_core;
mod video_input;

use crate::clock::Clock;
use crate::config::WXH;
use crate::cpu::Cpu;
use crate::cpu_core::CpuCore;
use crate::cpu_debugger::CpuDebugger;
// use crate::gpu::Gpu;
use crate::video_input::VideoInput;
use clap::Parser;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use winit::event_loop::EventLoop;

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
    let (tx, rx) = mpsc::channel::<[u8; WXH]>();

    let mut cpu = Cpu::new();
    let mut core = CpuCore::new();
    let cpu_debugger = CpuDebugger::new();
    let mut app = VideoInput::new(rx);
    let event_loop = EventLoop::new().unwrap();

    args.rom_file.map(|r| {
        core
            .load_rom(&mut cpu, &r)
            .expect(&format!("File {} not found or not readable", &r));
    });

    std::thread::spawn(move || {
        run_cpu_thread(tx, cpu, core, cpu_debugger);
    });


    event_loop.run_app(&mut app).expect("TODO: panic message");
}

fn run_cpu_thread(tx: Sender<[u8; WXH]>,
                  mut cpu: Cpu,
                  mut core: CpuCore,
                  mut cpu_debugger: CpuDebugger) {
    let mut clock = Clock::new();

    while cpu.running {
        clock.start();
        core.tick(&mut cpu);
        cpu_debugger.tick(&mut cpu).unwrap();
        clock.stop_and_wait();

    }

    if !cpu.panic {
        // panic
    }

    cpu_debugger.quit().unwrap();
}
