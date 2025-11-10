use crate::config::W;
use crate::cpu::Cpu;
use crate::cpu_core::CpuCore;
use rand::random;

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
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;
        let vx = cpu.v[x] as usize;
        let vy = cpu.v[y] as usize;
        let base_mem = cpu.i as usize;

        // TODO: handle the wrap to the opposite side of the screen
        // TODO: handle collision using VF register
        for j in 0..n {
            let vram_ptr = vx + (vy + j) * W;

            let pixels = format!("{:08b}", cpu.mem[base_mem + j]);

            for (idx, bit) in pixels.char_indices() {
                cpu.vram[vram_ptr + idx] ^= bit != '0';
            }
        }
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
    /// position, PC is increased by 2.
    pub(super) fn op_ex9e(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xF) as usize;
        if cpu.keypad[cpu.v[x] as usize] {
            cpu.pc += 2;
        }
    }

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up
    /// position, PC is increased by 2.
    pub(super) fn op_exa1(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xF) as usize;
        if !cpu.keypad[cpu.v[x] as usize] {
            cpu.pc += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::Cpu;
    #[test]
    fn decode_op_test_annn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new();
        cpu.i = 0x444;

        cpu_core.decode_opcode(&mut cpu, 0xA555);

        assert_eq!(cpu.i, 0x555);
    }
    #[test]
    fn decode_op_test_bnnn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new();
        cpu.pc = 0x400;
        cpu.v[0] = 0x10;

        cpu_core.decode_opcode(&mut cpu, 0xB500);

        assert_eq!(cpu.pc, 0x510);
    }
    #[test]
    fn decode_op_test_cxkk_and_0() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new();
        cpu.v[5] = 0x77;

        for _ in 0..5 {
            cpu_core.decode_opcode(&mut cpu, 0xC500);
            assert_eq!(cpu.v[5], 0x00);
        }
    }
    #[test]
    fn decode_op_test_cxkk_rnd() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new();
        cpu.v[5] = 0x77;

        let mut res: Vec<u8> = Vec::new();

        for _ in 0..9 {
            cpu_core.decode_opcode(&mut cpu, 0xC5FF);
            res.push(cpu.v[5]);
        }

        assert!(!res.iter().all(|x| *x == res[0]));
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
    #[test]
    fn decode_op_test_dxyn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new();
        cpu.i = 0x400;

        cpu.mem[0x400] = 0x01;
        cpu.mem[0x401] = 0x02;
        cpu.mem[0x402] = 0x04;
        cpu.mem[0x403] = 0x08;

        cpu.v[2] = 0x20;
        cpu.v[3] = 0x10;

        cpu_core.decode_opcode(&mut cpu, 0xD234);
        assert!(cpu.vram[W * 0x10 + 0x20 + 7]);
        assert!(cpu.vram[W * 0x11 + 0x20 + 6]);
        assert!(cpu.vram[W * 0x12 + 0x20 + 5]);
        assert!(cpu.vram[W * 0x13 + 0x20 + 4]);
    }
    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
    /// position, PC is increased by 2.
    #[test]
    fn decode_op_test_ex9e() {
        // assert!(false);
    }
    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    #[test]
    fn decode_op_test_exa1() {
        // assert!(false);
    }
}
