use crate::config::{H, W};
use crate::cpu_engine::CpuEngine;

impl<'a> CpuEngine<'a> {
    // Operations //////////////////////////////////////////////////////////////

    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    /// This instruction is only used on the old computers on which Chip-8 was
    /// originally implemented. It is ignored by modern interpreters.
    pub(super) fn op_0nnn(&mut self, _opcode: u16) {
        print!(".")
        // NO-OP Ignored
    }

    ///00E0 - CLS
    /// Clear the display.
    pub(super) fn op_00e0(&mut self, _opcode: u16) {
        self.cpu.vram = [false; H * W];
        self.draw_flag = true;
    }

    /// 00EE - RET
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack
    /// then subtracts 1 from the stack pointer.
    pub(super) fn op_00ee(&mut self, _opcode: u16) {
        self.cpu.sp -= 1;
        self.cpu.pc = self.cpu.stack.pop().unwrap();
    }

    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    pub(super) fn op_1nnn(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;

        self.cpu.pc = nnn;
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the
    /// top of the stack. The PC is then set to nnn.
    pub(super) fn op_2nnn(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;
        self.cpu.sp += 1;
        self.cpu.stack.push(self.cpu.pc);

        self.cpu.pc = nnn;
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    pub(super) fn op_3xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        if self.cpu.v[x] == kk {
            self.cpu.pc += 2
        }
    }

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    pub(super) fn op_4xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        if self.cpu.v[x] != kk {
            self.cpu.pc += 2
        }
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal,
    /// increments the program counter by 2.
    pub(super) fn op_5xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 8) as usize;

        if self.cpu.v[x] == self.cpu.v[y] {
            self.cpu.pc += 2
        }
    }
    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    pub(super) fn op_6xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        self.cpu.v[x] = kk;
    }

    ///7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    pub(super) fn op_7xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        self.cpu.v[x] += kk;
    }
    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal,
    /// the program counter is increased by 2.
    pub(super) fn op_9xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 8) as usize;

        if self.cpu.v[x] != self.cpu.v[y] {
            self.cpu.pc += 2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use super::*;

    #[test]
    fn decode_op_test_0nnn() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        let last_pc = cpu_engine.cpu.pc;
        let last_sp = cpu_engine.cpu.sp;

        cpu_engine.decode_opcode(0x0234);

        assert_eq!(cpu_engine.cpu.pc, last_pc);
        assert_eq!(cpu_engine.cpu.sp, last_sp);
    }
    #[test]
    fn decode_op_test_00e0() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.decode_opcode(0x00e0);
        cpu_engine.cpu.vram.iter().for_each(|item| {
            assert_eq!(*item, false);
        });

        assert_eq!(cpu_engine.cpu.draw_flag, true);
    }
    #[test]
    fn decode_op_test_00ee() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.sp = 1;
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.stack.push(0x400);

        cpu_engine.decode_opcode(0x00ee);
        assert_eq!(cpu_engine.cpu.sp, 0);
        assert_eq!(cpu_engine.cpu.stack.len(), 0);
        assert_eq!(cpu_engine.cpu.pc, 0x400);
    }
    #[test]
    fn decode_op_test_1nnn() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.decode_opcode(0x1234);
        assert_eq!(cpu_engine.cpu.pc, 0x234);
    }
    #[test]
    fn decode_op_test_2nnn() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;

        cpu_engine.decode_opcode(0x2345);

        assert_eq!(cpu_engine.cpu.sp, 1);
        assert_eq!(cpu_engine.cpu.stack[0], 0x300);
        assert_eq!(cpu_engine.cpu.pc, 0x345);
    }
    #[test]
    fn decode_op_test_op_3xkk_x_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;

        cpu_engine.decode_opcode(0x3405);

        assert_eq!(cpu_engine.cpu.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_3xkk_x_not_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;

        cpu_engine.decode_opcode(0x3403);

        assert_eq!(cpu_engine.cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_4xkk_x_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;

        cpu_engine.decode_opcode(0x4405);

        assert_eq!(cpu_engine.cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_4xkk_x_not_equals_kk() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;

        cpu_engine.decode_opcode(0x4403);

        assert_eq!(cpu_engine.cpu.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_5xy0_vx_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;
        cpu_engine.cpu.v[0] = 0x05;
        cpu_engine.decode_opcode(0x5400);

        assert_eq!(cpu_engine.cpu.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_5xy0_vx_not_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;
        cpu_engine.cpu.v[0] = 0x03;

        cpu_engine.decode_opcode(0x5400);

        assert_eq!(cpu_engine.cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_6xkk() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.v[4] = 0x05;
        cpu_engine.decode_opcode(0x6483);

        assert_eq!(cpu_engine.cpu.v[4], 0x83);
    }
    #[test]
    fn decode_op_test_op_7xkk() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.v[4] = 0x05;

        cpu_engine.decode_opcode(0x7483);

        assert_eq!(cpu_engine.cpu.v[4], 0x88);
    }
    #[test]
    fn decode_op_test_op_9xy0_vx_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;
        cpu_engine.cpu.v[0] = 0x05;
        cpu_engine.decode_opcode(0x9400);

        assert_eq!(cpu_engine.cpu.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_9xy0_vx_not_equals_vy() {
        let mut cpu = Cpu::new();
        let mut cpu_engine = CpuEngine::new(&mut cpu);
        cpu_engine.cpu.pc = 0x300;
        cpu_engine.cpu.v[4] = 0x05;
        cpu_engine.cpu.v[0] = 0x03;

        cpu_engine.decode_opcode(0x9400);

        assert_eq!(cpu_engine.cpu.pc, 0x302);
    }
}
