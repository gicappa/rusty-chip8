use std::process::exit;
use std::{fs, io};

pub(crate) const MEMORY_SIZE: usize = 4096;
pub(crate) const WIDTH: usize = 64;
pub(crate) const HEIGHT: usize = 32;
pub(crate) const START_ADDRESS: usize = 0x200;

pub struct Chip8 {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    // pub keypad: [u8; 16],
    pub display: [u8; WIDTH * HEIGHT],
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
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            // keypad: [0; 16],
            display: [0; WIDTH * HEIGHT],
        }
    }

    pub fn reset_memory(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        for (i, &byte) in font.iter().enumerate() {
            self.memory[i] = byte;
        }

        self.memory[80..512].fill(0);
    }

    /// Executing a step of the emulator cycle
    /// - reading next operation
    /// - decoding  and executing upon next operation
    ///
    pub fn step(&mut self) {
        let opcode = self.read_opcode();

        self.decode_op(opcode);
    }

    fn read_opcode(&self) -> u16 {
        let Chip8 { memory, pc, .. } = self;
        let _pc = *pc as usize;

        let hi = memory[_pc] as u16;
        let lo = memory[_pc + 1] as u16;

        hi << 8 | lo
    }

    /// Chip 8 - Instruction set
    ///            00E0 - CLS
    ///            00EE - RET
    ///            0nnn - SYS addr
    ///            1nnn - JP addr
    ///            2nnn - CALL addr
    ///            3xkk - SE Vx, byte
    ///            4xkk - SNE Vx, byte
    ///            5xy0 - SE Vx, Vy
    ///            6xkk - LD Vx, byte
    ///            7xkk - ADD Vx, byte
    ///            8xy0 - LD Vx, Vy
    ///            8xy1 - OR Vx, Vy
    ///            8xy2 - AND Vx, Vy
    ///            8xy3 - XOR Vx, Vy
    ///            8xy4 - ADD Vx, Vy
    ///            8xy5 - SUB Vx, Vy
    ///            8xy6 - SHR Vx {, Vy}
    ///            8xy7 - SUBN Vx, Vy
    ///            8xyE - SHL Vx {, Vy}
    ///            9xy0 - SNE Vx, Vy
    ///            Annn - LD I, addr
    ///            Bnnn - JP V0, addr
    ///            Cxkk - RND Vx, byte
    ///            Dxyn - DRW Vx, Vy, nibble
    ///            Ex9E - SKP Vx
    ///            ExA1 - SKNP Vx
    ///            Fx07 - LD Vx, DT
    ///            Fx0A - LD Vx, K
    ///            Fx15 - LD DT, Vx
    ///            Fx18 - LD ST, Vx
    ///            Fx1E - ADD I, Vx
    ///            Fx29 - LD F, Vx
    ///            Fx33 - LD B, Vx
    ///            Fx55 - LD [I], Vx
    ///            Fx65 - LD Vx, [I]
    pub(super) fn decode_op(&mut self, opcode: u16) {
        match opcode {
            // 0x1nnn - Jump
            0x1000..=0x1FFF => self.op_1nnn(opcode),

            // 0x2nnn - Call subroutine
            0x2000..=0x2FFF => self.op_2nnn(opcode),

            // 0x3xkk - Skip if Vx == kk
            0x3000..=0x3FFF => self.op_3xkk(opcode),
            
            // 0x8xy0-0x8xyE - Arithmetic/logic operations
            code => match code & 0xF00F {
                0x8000 => self.op_8xy0(opcode),
                0x8001 => self.op_8xy1(opcode),
                0x8002 => self.op_8xy2(opcode),
                0x8003 => self.op_8xy3(opcode),
                0x8004 => self.op_8xy4(opcode),
                0x8005 => self.op_8xy5(opcode),
                0x8006 => self.op_8xy6(opcode),
                0x8007 => self.op_8xy7(opcode),
                0x8008 => self.op_8xy8(opcode),
                _ => println!("Not matching"),
            },
        }
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
    fn load_rom_test() {
        let mut chip = Chip8::new();

        chip.load_rom("tests/fixtures/test_opcode.ch8")
            .expect("Error loading fixture files");

        assert!(
            chip.memory[START_ADDRESS..]
                .starts_with(&[0x12, 0x4e, 0xea, 0xac, 0xaa, 0xea, 0xce, 0xaa])
        );
    }

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
    #[test]
    fn reset_memory() {
        let mut chip = Chip8::new();

        chip.reset_memory();

        assert!(chip.memory.iter().nth(80).iter().all(|&&b| b == 0));
        assert_eq!(chip.memory[0], 0xf0);
        assert_eq!(chip.memory[1], 0x90);
        assert_eq!(chip.memory[77], 0xF0);
        assert_eq!(chip.memory[79], 0x80);
    }
}
