use crate::config::{W, WXH};
use crate::cpu::{Cpu, FONT_ADDR, FONT_SIZE};
use crate::cpu_core::CpuCore;
use rand::random;

impl CpuCore {
    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    /// This instruction is only used on the old computers on which Chip-8 was
    /// originally implemented. It is ignored by modern interpreters.
    pub(super) fn op_0nnn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let nnn = opcode & 0x0fff;

        cpu.pc = nnn;

        cpu.wait_for_key = true;
        cpu.draw_flag = true;
    }
    /// 00D4 - RET from a machine language subroutine
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack
    /// then subtracts 1 from the stack pointer.
    pub(super) fn op_00d4(&mut self, cpu: &mut Cpu, _opcode: u16) {
        cpu.pc = cpu.stack[usize::from(cpu.sp)];
        cpu.sp -= 1;
    }

    ///00E0 - CLS
    /// Clear the display.
    pub(super) fn op_00e0(&mut self, cpu: &mut Cpu, _opcode: u16) {
        cpu.vram = [0; WXH];
        cpu.draw_flag = true;
    }

    /// 00EE - RET
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack
    /// then subtracts 1 from the stack pointer.
    pub(super) fn op_00ee(&mut self, cpu: &mut Cpu, _opcode: u16) {
        cpu.pc = cpu.stack[usize::from(cpu.sp)];
        cpu.sp -= 1;
    }

    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    pub(super) fn op_1nnn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let nnn = opcode & 0x0fff;

        cpu.pc = nnn - 2;
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the
    /// top of the stack. The PC is then set to nnn.
    pub(super) fn op_2nnn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let nnn = opcode & 0x0fff;
        cpu.sp += 1;
        cpu.stack[usize::from(cpu.sp)] = cpu.pc;
        cpu.pc = nnn - 2;
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    pub(super) fn op_3xkk(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xf) as usize;
        let kk = (opcode & 0x00ff) as u8;

        if cpu.v[x] == kk {
            cpu.pc += 2
        }
    }

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    pub(super) fn op_4xkk(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xf) as usize;
        let kk = (opcode & 0x00ff) as u8;

        if cpu.v[x] != kk {
            cpu.pc += 2
        }
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal,
    /// increments the program counter by 2.
    pub(super) fn op_5xy0(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xf) as usize;
        let y = ((opcode >> 4) & 0xf) as usize;

        if cpu.v[x] == cpu.v[y] {
            cpu.pc += 2
        }
    }
    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    pub(super) fn op_6xkk(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xf) as usize;
        let kk = (opcode & 0x00ff) as u8;

        cpu.v[x] = kk;
    }

    ///7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    pub(super) fn op_7xkk(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xf) as usize;
        let kk = (opcode & 0x00ff) as u8;

        (cpu.v[x], _) = cpu.v[x].overflowing_add(kk);
    }
    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal,
    /// the program counter is increased by 2.
    pub(super) fn op_9xy0(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xf) as usize;
        let y = ((opcode >> 4) & 0xf) as usize;

        if cpu.v[x] != cpu.v[y] {
            cpu.pc += 2
        }
    }


    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    pub(super) fn op_8xy0(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        cpu.v[x] = cpu.v[y];
    }

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    pub(super) fn op_8xy1(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        cpu.v[x] |= cpu.v[y];
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corresponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    pub(super) fn op_8xy2(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        cpu.v[x] &= cpu.v[y];
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corresponding bits from two values, and if the bits are not
    /// both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    pub(super) fn op_8xy3(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        cpu.v[x] ^= cpu.v[y]
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255)
    /// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    pub(super) fn op_8xy4(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        let sum: usize = cpu.v[x] as usize + cpu.v[y] as usize;

        if sum > 255 {
            cpu.v[0xF] = 1
        } else {
            cpu.v[0xF] = 0
        }

        cpu.v[x] = sum as u8;
    }

    /// 8xy5 - SUB Vx, Vy
    /// Let VX = VX - VY (VF = 00 if VX < VY, VF = 01 if VX >= VY)
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results
    /// stored in Vx.
    pub(super) fn op_8xy5(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        let vx = cpu.v[x] as isize;
        let vy = cpu.v[y] as isize;

        if vx >= vy {
            cpu.v[0xF] = 1
        } else {
            cpu.v[0xF] = 0
        }

        cpu.v[x] = (vx - vy) as u8
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    ///
    /// Actual implementation Vx=Vy=Vy>>1
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    pub(super) fn op_8xy6(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        cpu.v[x] = cpu.v[x] >> 1;
        cpu.v[y] = cpu.v[x];
        cpu.v[0xF] = cpu.v[x] & 0x1;
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy,
    /// and the results stored in Vx.
    pub(super) fn op_8xy7(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        let vx = cpu.v[x] as isize;
        let vy = cpu.v[y] as isize;

        if vy >= vx {
            cpu.v[0xF] = 1
        } else {
            cpu.v[0xF] = 0
        }

        cpu.v[x] = (vy - vx) as u8;
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    ///
    /// Actual implementation Vx=Vy=Vy<<1
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    pub(super) fn op_8xye(&mut self, cpu: &mut Cpu, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        cpu.v[x] = cpu.v[x] << 1;
        cpu.v[y] = cpu.v[x];
        cpu.v[0xF] = (cpu.v[x] & 0x8) >> 3;
    }

    // Helper //////////////////////////////////////////////////////////////////
    #[inline]
    pub(super) fn regs_xy(opcode: u16) -> (usize, usize) {
        // Original form: (opcode & 0x0F00) >> 8; (opcode & 0x00F0) >> 4
        // Equivalent: (opcode >> 8) & 0xF; (opcode >> 4) & 0xF
        let x = ((opcode >> 8) & 0xf) as usize;
        let y = ((opcode >> 4) & 0xf) as usize;
        (x, y)
    }
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
    ///
    /// Show n-byte MI pattern at VX-VY coordinates.
    /// I unchanged. MI pattern is combined with existing display via EXCLUSIVE-OR function.
    /// VF = 01 if a 1 in MI pattern matches 1 in existing display
    pub(super) fn op_dxyn(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;
        let vx = cpu.v[x] as usize;
        let vy = cpu.v[y] as usize;
        let base_mem = cpu.i as usize;

        cpu.v[0xf] = 0;

        // TODO: handle the wrap to the opposite side of the screen
        for j in 0..n {
            let vram_ptr = vx + (vy + j) * W;
            
            for i in 0..8 {
                let mem_bit = (cpu.mem[base_mem + j] >> (7 - i)) & 1 == 1;
                let vram_bit = cpu.vram[vram_ptr + i] != 0x00;

                cpu.vram[vram_ptr + i] = if mem_bit ^ vram_bit { 0xFF } else { 0x00 };

                if mem_bit & vram_bit {
                    cpu.v[0xf] = 1;
                }
            }
        }

        // 10010010 XOR
        // 00011001
        // --------
        // 10001011

        // 10010010 AND
        // 00011001
        // --------
        // 00010000

        // 11010010 AND
        // 00101001
        // --------
        // 00000000

        // XOR 1 1 = 0 -> VF=1
        // XOR 1 0 = 1
        // XOR 0 1 = 1
        // XOR 0 0 = 0

        cpu.draw_flag = true;
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
    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    pub(super) fn op_fx07(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        cpu.v[x] = cpu.delay_timer;
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    pub(super) fn op_fx0a(&mut self, cpu: &mut Cpu, opcode: u16) {
        let _x = usize::from((opcode >> 8) & 0xF);

        cpu.wait_for_key = true;

        // TODO - waiting for the key to press
    }

    ///Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    pub(super) fn op_fx15(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        cpu.delay_timer = cpu.v[x];
    }

    ///Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    pub(super) fn op_fx18(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        cpu.v[x] = cpu.sound_timer;
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in 'I'.
    pub(super) fn op_fx1e(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        cpu.i += cpu.v[x] as u16;
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value
    /// of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    pub(super) fn op_fx29(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        cpu.i = FONT_ADDR + FONT_SIZE * u16::from(cpu.v[x]);
    }

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at location I+2.
    pub(super) fn op_fx33(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        let bcd = format!("{:03}", cpu.v[x]);

        let b = bcd.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
        let c = bcd.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
        let d = bcd.chars().nth(2).unwrap().to_digit(10).unwrap() as u8;


        cpu.mem[usize::from(cpu.i)] = b;
        cpu.mem[usize::from(cpu.i + 1)] = c;
        cpu.mem[usize::from(cpu.i + 2)] = d;
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location 'I'.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at
    /// the address in 'I'.
    pub(super) fn op_fx55(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = ((opcode >> 8) & 0xF) as usize;

        for idx in 0..(x + 1) {
            cpu.mem[cpu.i as usize + idx] = cpu.v[idx];
        }
    }
    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    pub(super) fn op_fx65(&mut self, cpu: &mut Cpu, opcode: u16) {
        let x = usize::from((opcode >> 8) & 0xF);

        for idx in 0..(x + 1) {
            cpu.v[idx] = cpu.mem[cpu.i as usize + idx];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::Cpu;

    #[test]
    fn decode_op_test_0nnn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu_core.decode_opcode(&mut cpu, 0x0234);
        assert_eq!(cpu.pc, 0x0234);
        assert_eq!(cpu.sp, 0);
    }
    #[test]
    fn decode_op_test_00e0() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu_core.decode_opcode(&mut cpu, 0x00e0);
        // TODO TESTS
        // cpu.vram.iter().for_each(|item| {
        //     assert_eq!(*item, false);
        // });
        assert_eq!(cpu.draw_flag, true);
    }
    #[test]
    fn decode_op_test_00ee() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.sp = 1;
        cpu.pc = 0x300;
        cpu.stack[1] = 0x400;
        cpu_core.decode_opcode(&mut cpu, 0x00ee);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 0x400);
    }
    #[test]
    fn decode_op_test_1nnn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu_core.decode_opcode(&mut cpu, 0x1234);
        assert_eq!(cpu.pc, 0x234 - 2);
    }
    #[test]
    fn decode_op_test_2nnn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu_core.decode_opcode(&mut cpu, 0x2345);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[1], 0x300);
        assert_eq!(cpu.pc, 0x345 - 2);
    }
    #[test]
    fn decode_op_test_op_3xkk_x_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x3405);
        assert_eq!(cpu.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_3xkk_x_not_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x3403);
        assert_eq!(cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_4xkk_x_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x4405);
        assert_eq!(cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_4xkk_x_not_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x4403);
        assert_eq!(cpu.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_5xy0_vx_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu.v[0] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x5400);
        assert_eq!(cpu.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_5xy0_vx_not_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu.v[0] = 0x03;
        cpu_core.decode_opcode(&mut cpu, 0x5400);
        assert_eq!(cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_6xkk() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[4] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x6483);
        assert_eq!(cpu.v[4], 0x83);
    }
    #[test]
    fn decode_op_test_op_7xkk() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[4] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x7483);
        assert_eq!(cpu.v[4], 0x88);
    }
    #[test]
    fn decode_op_test_op_9xy0_vx_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu.v[0] = 0x05;
        cpu_core.decode_opcode(&mut cpu, 0x9400);

        assert_eq!(cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_9xy0_vx_not_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x300;
        cpu.v[4] = 0x05;
        cpu.v[0] = 0x03;

        cpu_core.decode_opcode(&mut cpu, 0x9400);

        assert_eq!(cpu.pc, 0x302);
    }

    #[test]
    fn regs_xy_extracts_indices() {
        let (x, y) = CpuCore::regs_xy(0x8AB1);
        assert_eq!(x, 0xA);
        assert_eq!(y, 0xB);
    }

    #[test]
    fn decode_op_test_8xy0() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[3] = 0x08;
        cpu.v[4] = 0x10;
        cpu_core.decode_opcode(&mut cpu, 0x8340);
        assert_eq!(cpu.v[3], 0x10);
        assert_eq!(cpu.v[4], 0x10);
    }

    #[test]
    fn decode_op_test_8xy1() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[2] = 0x40;
        cpu.v[3] = 0xA8;
        cpu_core.decode_opcode(&mut cpu, 0x8231);
        assert_eq!(cpu.v[2], 0xE8);
    }
    #[test]
    fn decode_op_test_8xy2() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[2] = 0xE8;
        cpu.v[3] = 0x44;
        cpu_core.decode_opcode(&mut cpu, 0x8232);
        assert_eq!(cpu.v[2], 0x40);
    }
    #[test]
    fn decode_op_test_8xy3() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0xE8;
        cpu.v[6] = 0x56;
        cpu_core.decode_opcode(&mut cpu, 0x8563);
        assert_eq!(cpu.v[5], 0xBE);
    }
    #[test]
    fn decode_op_test_8xy4_no_carry() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x08;
        cpu.v[6] = 0x56;
        cpu_core.decode_opcode(&mut cpu, 0x8564);
        assert_eq!(cpu.v[5], 0x5E);
        assert_eq!(cpu.v[15], 0x0);
    }

    #[test]
    fn decode_op_test_8xy4_with_carry() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0xFF;
        cpu.v[6] = 0x04;
        cpu_core.decode_opcode(&mut cpu, 0x8564);
        assert_eq!(cpu.v[5], 0x03);
        assert_eq!(cpu.v[15], 0x1);
    }

    #[test]
    fn decode_op_test_8xy5_with_carry() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x5F;
        cpu.v[6] = 0x14;
        cpu_core.decode_opcode(&mut cpu, 0x8565);
        assert_eq!(cpu.v[5], 0x4B);
        assert_eq!(cpu.v[15], 0x1);
    }

    #[test]
    fn decode_op_test_8xy5_no_carry() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x14;
        cpu.v[6] = 0x5F;
        cpu_core.decode_opcode(&mut cpu, 0x8565);
        assert_eq!(cpu.v[5], 0xB5);
        assert_eq!(cpu.v[15], 0x0);
    }

    #[test]
    fn decode_op_test_8xy6_lsb_1() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0xEE;
        cpu.v[6] = 0x5F;
        cpu_core.decode_opcode(&mut cpu, 0x8566);
        assert_eq!(cpu.v[5], 0x77);
        assert_eq!(cpu.v[6], 0x77);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn decode_op_test_8xy6_lsb_0() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0xE0;
        cpu.v[6] = 0x34;
        cpu_core.decode_opcode(&mut cpu, 0x8566);
        assert_eq!(cpu.v[5], 0x70);
        assert_eq!(cpu.v[6], 0x70);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn decode_op_test_8xy7_with_carry() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);
        cpu.v[5] = 0x14;
        cpu.v[6] = 0x5F;
        cpu_core.decode_opcode(&mut cpu, 0x8567);
        assert_eq!(cpu.v[5], 0x4B);
        assert_eq!(cpu.v[15], 0x1);
    }

    #[test]
    fn decode_op_test_8xy7_no_carry() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x14;
        cpu.v[6] = 0x5F;
        cpu_core.decode_opcode(&mut cpu, 0x8567);
        assert_eq!(cpu.v[5], 0x4B);
        assert_eq!(cpu.v[15], 1);
    }

    #[test]
    fn decode_op_test_8xye_lsb_0() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x81;
        cpu.v[6] = 0x5F;
        cpu_core.decode_opcode(&mut cpu, 0x856e);
        assert_eq!(cpu.v[5], 0x02);
        assert_eq!(cpu.v[6], 0x02);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn decode_op_test_8xye_lsb_1() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x77;
        cpu.v[6] = 0x5F;
        cpu_core.decode_opcode(&mut cpu, 0x856e);
        assert_eq!(cpu.v[5], 0xEE);
        assert_eq!(cpu.v[6], 0xEE);
        assert_eq!(cpu.v[0xF], 1);
    }
    #[test]
    fn decode_op_test_annn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.i = 0x444;
        cpu_core.decode_opcode(&mut cpu, 0xA555);
        assert_eq!(cpu.i, 0x555);
    }
    #[test]
    fn decode_op_test_bnnn() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.pc = 0x400;
        cpu.v[0] = 0x10;
        cpu_core.decode_opcode(&mut cpu, 0xB500);
        assert_eq!(cpu.pc, 0x510);
    }
    #[test]
    fn decode_op_test_cxkk_and_0() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

        cpu.v[5] = 0x77;
        for _ in 0..5 {
            cpu_core.decode_opcode(&mut cpu, 0xC500);
            assert_eq!(cpu.v[5], 0x00);
        }
    }
    #[test]
    fn decode_op_test_cxkk_rnd() {
        let mut cpu = Cpu::new();
        let mut cpu_core = CpuCore::new(None);

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
        let mut cpu_core = CpuCore::new(None);

        cpu.i = 0x400;
        cpu.mem[0x400] = 0x01;
        cpu.mem[0x401] = 0x02;
        cpu.mem[0x402] = 0x04;
        cpu.mem[0x403] = 0x08;
        cpu.v[2] = 0x20;
        cpu.v[3] = 0x10;
        cpu_core.decode_opcode(&mut cpu, 0xD234);
        // TODO TESTS
        // assert!(cpu.vram[W * 0x10 + 0x20 + 7]);
        // assert!(cpu.vram[W * 0x11 + 0x20 + 6]);
        // assert!(cpu.vram[W * 0x12 + 0x20 + 5]);
        // assert!(cpu.vram[W * 0x13 + 0x20 + 4]);
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
    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    #[test]
    fn decode_op_test_fx07() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.delay_timer = 123u8;
        core.decode_opcode(&mut cpu, 0xFA07);
        assert_eq!(cpu.v[0xA], cpu.delay_timer);
    }
    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    #[test]
    fn decode_op_test_fx0a() {
        println!("decode_op_test_fx0a to be implemented");
    }
    ///Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    #[test]
    fn decode_op_test_fx15() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.v[0xB] = 34u8;
        core.decode_opcode(&mut cpu, 0xFB15);
        assert_eq!(cpu.delay_timer, cpu.v[0xB]);
    }
    ///Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    #[test]
    fn decode_op_test_fx18() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.sound_timer = 13u8;
        core.decode_opcode(&mut cpu, 0xF218);
        assert_eq!(cpu.v[0x2], cpu.sound_timer);
    }
    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in 'I'.
    #[test]
    fn decode_op_test_fx1e() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.i = 0x402;
        cpu.v[0x3] = 0x2A;
        core.decode_opcode(&mut cpu, 0xF31E);
        assert_eq!(cpu.i, 0x42C);
    }
    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value
    /// of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    #[test]
    fn decode_op_test_fx29() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.i = 0x888;
        cpu.v[0x1] = 0x3;
        core.decode_opcode(&mut cpu, 0xF129);
        assert_eq!(cpu.i, FONT_ADDR + 15);
    }
    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at location I+2.
    #[test]
    fn decode_op_test_fx33() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.v[0x4] = 238; // 0xEE;
        cpu.i = 0x330;
        core.decode_opcode(&mut cpu, 0xF433);
        assert_eq!(cpu.mem[0x330], 2);
        assert_eq!(cpu.mem[0x331], 3);
        assert_eq!(cpu.mem[0x332], 8);

        cpu.v[0x4] = 3; // 0xEE;
        cpu.i = 0x330;
        core.decode_opcode(&mut cpu, 0xF433);
        assert_eq!(cpu.mem[0x330], 0);
        assert_eq!(cpu.mem[0x331], 0);
        assert_eq!(cpu.mem[0x332], 3);
    }
    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location 'I'.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at
    /// the address in 'I'.
    #[test]
    fn decode_op_test_fx55() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.i = 0x502;
        for idx in 0..7 {
            cpu.v[idx] = 0x10u8 + idx as u8;
        }
        core.decode_opcode(&mut cpu, 0xF655);
        for idx in 0..7 {
            assert_eq!(cpu.v[idx], cpu.mem[0x502 + idx]);
        }
        assert_eq!(cpu.v[7], 0x0);
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    #[test]
    fn decode_op_test_fx65() {
        let mut cpu = Cpu::new();
        let mut core = CpuCore::new(None);

        cpu.i = 0x602;
        for idx in 0..9 {
            cpu.v[idx] = 0x20u8 + idx as u8;
        }
        core.decode_opcode(&mut cpu, 0xF865);
        for idx in 0..9 {
            assert_eq!(cpu.mem[0x602 + idx], cpu.v[idx]);
        }
        assert_eq!(cpu.v[8], 0x0);
    }
}
