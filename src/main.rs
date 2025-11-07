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

use crate::config::VRAM;
use crate::cpu::Cpu;
use crate::gpu::Gpu;

use crate::clock::Clock;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use crate::debug_cli::DebugCli;

fn main() {
    let (tx, rx) = mpsc::channel::<VRAM>();
    let (key_tx, key_rx) = mpsc::channel::<u8>();

    let cpu = Arc::new(Mutex::new(Cpu::new()));

    let mut gpu = Gpu::new(rx, key_tx);

    // In a thread, f
    let handle = thread::spawn(move || {
        let cpu_arc_main = Arc::clone(&cpu);
        let cpu_arc_debug_cli = Arc::clone(&cpu);
        let cpu_debug_cli = cpu_arc_debug_cli.lock().unwrap();
        let mut debug_cli = DebugCli::new(&cpu_debug_cli);

        let mut clock = Clock::new();

        loop {

            clock.start();

            let mut cpu_guard = cpu_arc_main.lock().unwrap();

            cpu_guard.clk();

            if cpu_guard.draw_flag() {
                let frame = cpu_guard.vram;
                let _ = tx.send(frame);
            }

            // Check for key input
            while let Ok(key) = key_rx.try_recv() {
                // let mut cpu_guard = cpu_clone.lock().unwrap();
                cpu_guard.set_key(key, true);
            }

            clock.stop_and_wait();
            debug_cli.tick().unwrap();
            drop(cpu_guard);
        }

    });

    {
    }

    gpu.start();

    let _ = handle.join().unwrap();
    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");

}
