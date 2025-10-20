mod chip8;

use std::process::exit;
use std::{fs, io};

const MEMORY_SIZE: usize = 4096;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const START_ADDRESS: usize = 0x200;

struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    display: [u8; WIDTH * HEIGHT],
}

fn main() {
    println!("Chip8 Emulator Starting...");
    println!("Memory available: {}", MEMORY_SIZE);
    println!("Initializing CPU");
    let mut chip8 = Chip8::new();

    chip8
        .load_rom("rom.ch8")
        .expect("File not found or not readable");

    loop {
        chip8.cycle();

        if chip8.draw_flag() {
            chip8.draw_display();
        }
    }
}

impl Chip8 {
    fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            display: [0; WIDTH * HEIGHT],
        }
    }

    pub fn cycle(&self) {
        println!("emulator cycle");
    }
    pub fn draw_flag(&self) -> bool {
        true
    }

    pub fn draw_display(&self) {
        println!("draw display");
    }

    fn load_rom(&mut self, filename: &str) -> Result<(), io::Error> {
        let rom_data = fs::read(filename)?;

        for (i, &byte) in rom_data.iter().enumerate() {
            if START_ADDRESS + i >= MEMORY_SIZE {
                eprintln!(
                    "Buffer overflow.\nThe file is overflowing the available memory\nExiting"
                );
                exit(1);
            }

            self.memory[START_ADDRESS + i] = byte;
        }

        Ok(())
    }
}
