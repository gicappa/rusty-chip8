use crate::config::{H, VRAM, W};
use std::process::exit;
use std::{fs, io};

pub(crate) const MEMORY_SIZE: usize = 4096;
pub(crate) const START_ADDRESS: usize = 0x200;

pub struct Cpu {
    pub mem: [u8; MEMORY_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub _keypad: [u8; 16],
    pub vram: VRAM,
    pub(super) draw_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut s = Self {
            mem: [0; MEMORY_SIZE],
            v: [0; 16],
            i: 0,
            pc: START_ADDRESS as u16,
            sp: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            _keypad: [0; 16],
            vram: [false; W * H],
            draw_flag: false,
        };

        s.reset_memory();

        s
    }

    /// Execute one CPU cycle
    pub fn clk(&mut self) {
        self.draw_flag = false;

        let opcode = self.fetch_opcode();

        self.decode_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0 {
                // Placeholder for actual audio.
                println!("beep!");
            }
        }

        self.pc += 2;
    }

    fn fetch_opcode(&self) -> u16 {
        let Cpu { mem: memory, pc, .. } = self;
        let _pc = *pc as usize;

        let hi = memory[_pc] as u16;
        let lo = memory[_pc + 1] as u16;

        hi << 8 | lo
    }

    /// Chip 8 - Instruction set
    pub(super) fn decode_opcode(&mut self, opcode: u16) {
        match opcode {
            // 0x00e0 - CLS Clear display
            0x00e0 => self.op_00e0(opcode),
            // 0x00ee - RET Return from a subroutine.
            0x00ee => self.op_00ee(opcode),
            // 0x0nnn - SYS Ignored (old SYS addr)
            0x0000..=0x0FFF => self.op_0nnn(opcode),
            // 0x1nnn - JP addr
            0x1000..=0x1FFF => self.op_1nnn(opcode),
            // 0x2nnn - CALL addr
            0x2000..=0x2FFF => self.op_2nnn(opcode),
            // 0x3xkk - Skip next instruction if Vx = kk.
            0x3000..=0x3FFF => self.op_3xkk(opcode),
            // 0x4xkk - Skip next instruction if Vx != kk.
            0x4000..=0x4FFF => self.op_4xkk(opcode),
            // 0x5xy0 - Skip next instruction if Vx = Vy.
            code if code & 0xF00F == 0x5000 => self.op_5xy0(opcode),
            // 6xkk - Set Vx = kk.
            0x6000..=0x6FFF => self.op_6xkk(opcode),
            // 7xkk - Set Vx = Vx + kk.
            0x7000..=0x7FFF => self.op_7xkk(opcode),
            // 0x9xy0 - Skip next instruction if Vx != Vy.
            code if code & 0xF00F == 0x9000 => self.op_9xy0(opcode),
            // 0xAnnn - The value of register I is set to nnn.
            0xA000..=0xAFFF => self.op_annn(opcode),
            // 0xBnnn - Jump to location nnn + V0.
            0xB000..=0xBFFF => self.op_bnnn(opcode),
            // 0xcxkk - Set Vx = random byte AND kk.
            0xC000..=0xCFFF => self.op_cxkk(opcode),
            // 0xdxyn - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            0xD000..=0xDFFF => self.op_dxyn(opcode),
            // 0xex9e - Checks the keyboard
            code if code & 0xF0FF == 0xE09E => self.op_ex9e(opcode),
            // 0xexa1 - Checks the keyboard
            code if code & 0xF0FF == 0xE0A1 => self.op_exa1(opcode),
            code if code & 0xF0FF == 0xF007 => self.op_fx07(opcode),
            code if code & 0xF0FF == 0xF00A => self.op_fx0a(opcode),
            code if code & 0xF0FF == 0xF015 => self.op_fx15(opcode),
            code if code & 0xF0FF == 0xF018 => self.op_fx18(opcode),
            code if code & 0xF0FF == 0xF01E => self.op_fx1e(opcode),
            code if code & 0xF0FF == 0xF029 => self.op_fx29(opcode),
            code if code & 0xF0FF == 0xF033 => self.op_fx33(opcode),
            // 0x8xy0-0x8xyE - Arithmetic/logic operations
            code => match code & 0xF00F {
                // 0x8xy0 - Set Vx = Vy.
                0x8000 => self.op_8xy0(opcode),
                // 0x8xy1 - Set Vx = Vy.
                0x8001 => self.op_8xy1(opcode),
                // 0x8xy2 - Set Vx = Vy.
                0x8002 => self.op_8xy2(opcode),
                // 0x8xy3 - Set Vx = Vy.
                0x8003 => self.op_8xy3(opcode),
                // 0x8xy4 - Set Vx = Vy.
                0x8004 => self.op_8xy4(opcode),
                // 0x8xy5 - Set Vx = Vy.
                0x8005 => self.op_8xy5(opcode),
                // 0x8xy6 - Set Vx = Vy.
                0x8006 => self.op_8xy6(opcode),
                // 0x8xy7 - Set Vx = Vy.
                0x8007 => self.op_8xy7(opcode),
                // 0x8xye - Set Vx = Vy.
                0x800e => self.op_8xye(opcode),
                _ => println!("Not matching"),
            },
        }
    }

    fn reset_memory(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0,
            0x20, 0x60, 0x20, 0x20, 0x70,
            0xF0, 0x10, 0xF0, 0x80, 0xF0,
            0xF0, 0x10, 0xF0, 0x10, 0xF0,
            0x90, 0x90, 0xF0, 0x10, 0x10,
            0xF0, 0x80, 0xF0, 0x10, 0xF0,
            0xF0, 0x80, 0xF0, 0x90, 0xF0,
            0xF0, 0x10, 0x20, 0x40, 0x40,
            0xF0, 0x90, 0xF0, 0x90, 0xF0,
            0xF0, 0x90, 0xF0, 0x10, 0xF0,
            0xF0, 0x90, 0xF0, 0x90, 0x90,
            0xE0, 0x90, 0xE0, 0x90, 0xE0,
            0xF0, 0x80, 0x80, 0x80, 0xF0,
            0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0,
            0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        for (i, &byte) in font.iter().enumerate() {
            self.mem[i] = byte;
        }

        self.mem[80..512].fill(0);

        // TODO delete this
        for x in 0..W * H {
            self.vram[x] = x.is_multiple_of(2)
        }
    }

    pub fn draw_flag(&self) -> bool {
        self.draw_flag
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        if key < 16 {
            self._keypad[key as usize] = if pressed { 1 } else { 0 };
        }
    }

    #[allow(dead_code)]
    pub fn load_rom(&mut self, filename: &str) -> Result<(), io::Error> {
        let rom_data = fs::read(filename)?;

        for (i, &byte) in rom_data.iter().enumerate() {
            if START_ADDRESS + i >= MEMORY_SIZE {
                eprintln!(
                    "Buffer overflow.\nThe file is overflowing the available memory\nExiting"
                );
                exit(1);
            }

            self.mem[START_ADDRESS + i] = byte;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_rom_test() {
        let mut chip = Cpu::new();

        chip.load_rom("tests/fixtures/test_opcode.ch8")
            .expect("Error loading fixture files");

        assert!(
            chip.mem[START_ADDRESS..]
                .starts_with(&[0x12, 0x4e, 0xea, 0xac, 0xaa, 0xea, 0xce, 0xaa])
        );
    }

    #[test]
    fn new_initializes_state() {
        let chip = Cpu::new();

        assert_eq!(chip.pc, START_ADDRESS as u16);
        assert!(chip.mem.iter().all(|&b| b == 0));
        assert!(chip.v.iter().all(|&r| r == 0));
        assert_eq!(chip.i, 0);
        assert_eq!(chip.sp, 0);
        assert!(chip.stack.iter().all(|&s| s == 0));
        assert!(chip.vram.iter().all(|&p| p == false));
    }
    #[test]
    fn reset_memory() {
        let mut chip = Cpu::new();

        chip.reset_memory();

        assert!(chip.mem.iter().nth(80).iter().all(|&&b| b == 0));
        assert_eq!(chip.mem[0], 0xf0);
        assert_eq!(chip.mem[1], 0x90);
        assert_eq!(chip.mem[77], 0xF0);
        assert_eq!(chip.mem[79], 0x80);
    }
}
