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
// use crate::cpu_debugger::CpuDebugger;
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
    let (tx, rx) = mpsc::channel::<[u8; WXH]>();

    let mut app = VideoInput::new(rx);
    let event_loop = EventLoop::new().unwrap();

    std::thread::spawn(move || {
        run_cpu_thread(tx);
    });

    event_loop.run_app(&mut app).expect("TODO: panic message");
}

fn run_cpu_thread(tx: Sender<[u8; WXH]>) {
    let args = Args::parse();

    let mut cpu = Cpu::new();
    let mut core = CpuCore::new_tx(Some(tx));

    match args.rom_file  {
        None =>  cpu.panic(),
        Some(r) => cpu.load_rom(&r)
            .expect(&format!("File {} not found or not readable", &r))
    }

    // let mut cpu_debugger = CpuDebugger::new();
    let mut clock = Clock::new();

    while cpu.running {
        clock.start();
        core.tick(&mut cpu);
        // cpu_debugger.tick(&mut cpu).unwrap();
        clock.stop_and_wait();
    }

    // cpu_debugger.quit().unwrap();
}
