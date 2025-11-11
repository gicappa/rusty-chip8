use crate::config::{VRAM, WXH};

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
    pub _wait_for_key: bool,
    pub keypad: [bool; 16],
    pub vram: VRAM,
    pub draw_flag: bool,
    pub running: bool,
    pub panic: bool,
}

pub(crate) const FONT_ADDR: u16 = 0x00;
pub(crate) const FONT_SIZE: u16 = 5;
pub(crate) const FONT: [u8; 80] = [
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
            keypad: [false; 16],
            _wait_for_key: false,
            vram: [false; WXH],
            draw_flag: true,
            running: true,
            panic: false,
        };

        s.reset_memory();

        s
    }

    pub(crate) fn reset_memory(&mut self) {
        for (i, &byte) in FONT.iter().enumerate() {
            self.mem[usize::from(FONT_ADDR) + i] = byte;
        }

        self.mem[80..512].fill(0);
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::{Cpu, FONT};

    #[test]
    fn test_reset_memory() {
        let mut cpu = Cpu::new();

        cpu.reset_memory();

        assert!(cpu.mem.iter().nth(80).iter().all(|&&b| b == 0));
        assert_eq!(cpu.mem[0], 0xf0);
        assert_eq!(cpu.mem[1], 0x90);
        assert_eq!(cpu.mem[77], 0xF0);
        assert_eq!(cpu.mem[79], 0x80);

        for (i, byte) in FONT.iter().enumerate() {
            println!("{:08b} - {}", &byte, i);
        }
    }
}
