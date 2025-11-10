use crate::config::W;
use rand::random;
use crate::cpu::Cpu;
use crate::cpu_core::CpuCore;

impl CpuCore {
    // Operations //////////////////////////////////////////////////////////////

    /// Annn - LD I, addr
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    pub(super) fn op_annn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let nnn = opcode & 0x0fff;

        cpu.i = nnn;
    }
    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    pub(super) fn op_bnnn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let nnn = opcode & 0x0fff;

        cpu.pc = (cpu.v[0] as u16) + nnn;
    }
    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value
    /// kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    pub(super) fn op_cxkk(&mut self, cpu: &mut Cpu, opcode: u16) {
        let _nnn = opcode & 0x0fff;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;
        let rnd: u8 = random();

        cpu.v[x] = rnd & kk;
    }
    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes
    /// are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the
    /// existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set
    /// to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen. See instruction 8xy3 for more information
    /// on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    pub(super) fn op_dxyn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = (opcode & 0x0F00 >> 8) as usize;
        let y = (opcode & 0x00F0 >> 4) as usize;
        let _n = (opcode & 0x000F) as usize;

        let base_vram = cpu.v[x] as usize + (cpu.v[y] as usize * W);
        let base_mem = cpu.i as usize;


        let pixels = format!("{:b}", cpu.mem[base_mem]);

        for x in pixels.chars() {
            cpu.vram[base_vram] ^= x != '0'
        }

        // for i in 0..n {
        //     if base_vram + i <= WXH {
        //         let pixels = format!("{:b}", self.memory[base_mem + i]);
        //
        //         for x in pixels.chars() {
        //             self.display[base_vram + i] ^= if x == '0' { 0xff } else { 0x00 }
        //         }
        //
        //         if self.display[base_vram + i] == 0 && self.memory[base_mem + i] == 0xff {
        //             self.v[0xf] = 1;
        //         } else {
        //             self.v[0xf] = 0;
        //         }
        //     }
        // }
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
    /// position, PC is increased by 2.
    pub(super) fn op_ex9e(&mut self, _cpu: &mut Cpu, _opcode: u16) {}

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up
    /// position, PC is increased by 2.
    pub(super) fn op_exa1(&mut self, _cpu: &mut Cpu, _opcode: u16) {}
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use super::*;

    // #[test]
    // fn decode_op_test_annn() {
    //     let mut cpu = Cpu::new();
    //     let mut cpu_core = CpuCore::new(&mut cpu);
    //     cpu_core.cpu.i = 0x444;
    //
    //     cpu_core.decode_opcode(0xA555);
    //
    //     assert_eq!(cpu_core.cpu.i, 0x555);
    // }
    // #[test]
    // fn decode_op_test_bnnn() {
    //     let mut cpu = Cpu::new();
    //     let mut cpu_core = CpuCore::new(&mut cpu);
    //     cpu_core.cpu.pc = 0x400;
    //     cpu_core.cpu.v[0] = 0x10;
    //
    //     cpu_core.decode_opcode(0xB500);
    //
    //     assert_eq!(cpu_core.cpu.pc, 0x510);
    // }
    // #[test]
    // fn decode_op_test_cxkk_and_0() {
    //     let mut cpu = Cpu::new();
    //     let mut cpu_core = CpuCore::new(&mut cpu);
    //     cpu_core.cpu.v[5] = 0x77;
    //
    //     for _ in 0..5 {
    //         cpu_core.decode_opcode(0xC500);
    //         assert_eq!(cpu_core.cpu.v[5], 0x00);
    //     }
    // }
    // #[test]
    // fn decode_op_test_cxkk_rnd() {
    //     let mut cpu = Cpu::new();
    //     let mut cpu_core = CpuCore::new(&mut cpu);
    //     cpu_core.cpu.v[5] = 0x77;
    //
    //     let mut res: Vec<u8> = Vec::new();
    //
    //     for _ in 0..9 {
    //         cpu_core.decode_opcode(0xC5FF);
    //         res.push(cpu_core.cpu.v[5]);
    //     }
    //
    //     assert!(!res.iter().all(|x| *x == res[0]));
    // }
    // /// Dxyn - DRW Vx, Vy, nibble
    // /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    // ///
    // /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes
    // /// are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the
    // /// existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set
    // /// to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
    // /// it wraps around to the opposite side of the screen. See instruction 8xy3 for more information
    // /// on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    // #[test]
    // fn decode_op_test_dxyn() {
    //     assert!(false);
    // }
    // /// Ex9E - SKP Vx
    // /// Skip next instruction if key with the value of Vx is pressed.
    // /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
    // /// position, PC is increased by 2.
    // #[test]
    // fn decode_op_test_ex9e() {
    //     assert!(false);
    // }
    // /// ExA1 - SKNP Vx
    // /// Skip next instruction if key with the value of Vx is not pressed.
    // /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    // #[test]
    // fn decode_op_test_exa1() {
    //     assert!(false);
    // }
}
