use std::process::exit;
use std::{fs, io};

pub(crate) const MEMORY_SIZE: usize = 4096;
pub(crate) const WIDTH: usize = 64;
pub(crate) const HEIGHT: usize = 32;
pub(crate) const START_ADDRESS: usize = 0x200;


pub struct Chip8 {
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

impl Chip8 {
    pub fn new() -> Self {
        println!("Initializing CPU");
        println!("Memory available: {}", MEMORY_SIZE);

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

    pub fn load_rom(&mut self, filename: &str) -> Result<(), io::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_initializes_state() {
        let chip = Chip8::new();
        
        assert_eq!(chip.pc, START_ADDRESS as u16);
        assert!(chip.memory.iter().all(|&b| b == 0));
        assert!(chip.v.iter().all(|&r| r == 0));
        assert_eq!(chip.i, 0);
        assert_eq!(chip.sp, 0);
        assert!(chip.stack.iter().all(|&s| s == 0));
        assert!(chip.display.iter().all(|&p| p == 0));
    }
}