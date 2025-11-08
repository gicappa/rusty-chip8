use crate::cpu::Cpu;
use std::process::exit;
use std::{fs, io};

pub(crate) const MEMORY_SIZE: usize = 4096;
pub(crate) const START_ADDRESS: usize = 0x200;

pub struct CpuCore {}

impl CpuCore {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self, cpu: &mut Cpu) {
        cpu.draw_flag = false;

        let opcode = self.fetch_opcode(cpu);

        self.decode_opcode(cpu, opcode);

        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        }

        if cpu.sound_timer > 0 {
            cpu.sound_timer -= 1;
            if cpu.sound_timer == 0 {
                // Placeholder for actual audio.
                println!("beep!");
            }
        }

        cpu.pc += 2;
    }

    fn fetch_opcode(&mut self, cpu: &mut Cpu) -> u16 {
        let Cpu { mem, pc, .. } = cpu;
        let _pc = *pc as usize;

        let hi = mem[_pc] as u16;
        let lo = mem[_pc + 1] as u16;

        hi << 8 | lo
    }

    /// Chip 8 - Instruction set
    pub(super) fn decode_opcode(&mut self, cpu: &mut Cpu, opcode: u16) {
        match opcode {
            // 0x00e0 - CLS Clear display
            0x00e0 => self.op_00e0(cpu, opcode),
            // 0x00ee - RET Return from a subroutine.
            0x00ee => self.op_00ee(cpu, opcode),
            // 0x0nnn - SYS Ignored (old SYS addr)
            0x0000..=0x0FFF => self.op_0nnn(cpu, opcode),
            // 0x1nnn - JP addr
            0x1000..=0x1FFF => self.op_1nnn(cpu, opcode),
            // 0x2nnn - CALL addr
            0x2000..=0x2FFF => self.op_2nnn(cpu, opcode),
            // 0x3xkk - Skip next instruction if Vx = kk.
            0x3000..=0x3FFF => self.op_3xkk(cpu, opcode),
            // 0x4xkk - Skip next instruction if Vx != kk.
            0x4000..=0x4FFF => self.op_4xkk(cpu, opcode),
            // 0x5xy0 - Skip next instruction if Vx = Vy.
            code if code & 0xF00F == 0x5000 => self.op_5xy0(cpu, opcode),
            // 6xkk - Set Vx = kk.
            0x6000..=0x6FFF => self.op_6xkk(cpu, opcode),
            // 7xkk - Set Vx = Vx + kk.
            0x7000..=0x7FFF => self.op_7xkk(cpu, opcode),
            // 0x9xy0 - Skip next instruction if Vx != Vy.
            code if code & 0xF00F == 0x9000 => self.op_9xy0(cpu, opcode),
            // 0xAnnn - The value of register I is set to nnn.
            0xA000..=0xAFFF => self.op_annn(cpu, opcode),
            // 0xBnnn - Jump to location nnn + V0.
            0xB000..=0xBFFF => self.op_bnnn(cpu, opcode),
            // 0xcxkk - Set Vx = random byte AND kk.
            0xC000..=0xCFFF => self.op_cxkk(cpu, opcode),
            // 0xdxyn - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            0xD000..=0xDFFF => self.op_dxyn(cpu, opcode),
            // 0xex9e - Checks the keyboard
            code if code & 0xF0FF == 0xE09E => self.op_ex9e(cpu, opcode),
            // 0xexa1 - Checks the keyboard
            code if code & 0xF0FF == 0xE0A1 => self.op_exa1(cpu, opcode),
            code if code & 0xF0FF == 0xF007 => self.op_fx07(cpu, opcode),
            code if code & 0xF0FF == 0xF00A => self.op_fx0a(cpu, opcode),
            code if code & 0xF0FF == 0xF015 => self.op_fx15(cpu, opcode),
            code if code & 0xF0FF == 0xF018 => self.op_fx18(cpu, opcode),
            code if code & 0xF0FF == 0xF01E => self.op_fx1e(cpu, opcode),
            code if code & 0xF0FF == 0xF029 => self.op_fx29(cpu, opcode),
            code if code & 0xF0FF == 0xF033 => self.op_fx33(cpu, opcode),
            // 0x8xy0-0x8xyE - Arithmetic/logic operations
            code => match code & 0xF00F {
                // 0x8xy0 - Set Vx = Vy.
                0x8000 => self.op_8xy0(cpu, opcode),
                // 0x8xy1 - Set Vx = Vy.
                0x8001 => self.op_8xy1(cpu, opcode),
                // 0x8xy2 - Set Vx = Vy.
                0x8002 => self.op_8xy2(cpu, opcode),
                // 0x8xy3 - Set Vx = Vy.
                0x8003 => self.op_8xy3(cpu, opcode),
                // 0x8xy4 - Set Vx = Vy.
                0x8004 => self.op_8xy4(cpu, opcode),
                // 0x8xy5 - Set Vx = Vy.
                0x8005 => self.op_8xy5(cpu, opcode),
                // 0x8xy6 - Set Vx = Vy.
                0x8006 => self.op_8xy6(cpu, opcode),
                // 0x8xy7 - Set Vx = Vy.
                0x8007 => self.op_8xy7(cpu, opcode),
                // 0x8xye - Set Vx = Vy.
                0x800e => self.op_8xye(cpu, opcode),
                _ => println!("Not matching"),
            },
        }
    }

    #[allow(dead_code)]
    pub fn load_rom(&mut self, cpu: &mut Cpu, filename: &str) -> Result<(), io::Error> {
        let rom_data = fs::read(filename)?;

        for (i, &byte) in rom_data.iter().enumerate() {
            if START_ADDRESS + i >= MEMORY_SIZE {
                eprintln!(
                    "Buffer overflow.\nThe file is overflowing the available memory\nExiting"
                );
                exit(1);
            }

            cpu.mem[START_ADDRESS + i] = byte;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::cpu::Cpu;
    //
    // #[test]
    // fn load_rom_test() {
    //     let mut cpu = Cpu::new();
    //     let mut cpu_engine = CpuCore::new(&mut cpu);
    //
    //     cpu_engine.load_rom("tests/fixtures/test_opcode.ch8")
    //         .expect("Error loading fixture files");
    //
    //     assert!(
    //         cpu_engine.cpu.mem[START_ADDRESS..]
    //             .starts_with(&[0x12, 0x4e, 0xea, 0xac, 0xaa, 0xea, 0xce, 0xaa])
    //     );
    // }
    //
    // #[test]
    // fn new_initializes_state() {
    //     let mut cpu = Cpu::new();
    //     let cpu_engine = CpuCore::new(&mut cpu);
    //
    //     assert_eq!(cpu_engine.cpu.pc, START_ADDRESS as u16);
    //     assert!(cpu_engine.cpu.mem.iter().all(|&b| b == 0));
    //     assert!(cpu_engine.cpu.v.iter().all(|&r| r == 0));
    //     assert_eq!(cpu_engine.cpu.i, 0);
    //     assert_eq!(cpu_engine.cpu.sp, 0);
    //     assert!(cpu_engine.cpu.stack.iter().all(|&s| s == 0));
    //     assert!(cpu_engine.cpu.vram.iter().all(|&p| p == false));
    // }
}
