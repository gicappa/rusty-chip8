mod cpu;
mod cpu_op_01;
mod cpu_op_02;
mod cpu_op_03;
mod cpu_op_04;
mod gpu;
mod chip8;

use std::sync::mpsc;
use std::thread;

use crate::chip8::Chip8;
use crate::cpu::CPU;
use crate::gpu::{GPU,PIXELS};


fn main() {
    println!("Chip8 Emulator Starting...");

    let (tx, rx) = mpsc::channel::<[u8; PIXELS]>();
    let cpu = CPU::new();
    let mut gpu: GPU = GPU::new(rx);

    let _handle = thread::spawn(move || {
        let mut chip8 = Chip8::new(cpu, tx);
        chip8.start();
    });

    gpu.start();

    // chip8
    //     .load_rom("rom.ch8")
    //     .expect("File not found or not readable");
    let _ = _handle.join().unwrap();
}
