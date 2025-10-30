use crate::chip8::Chip8;

impl Chip8 {
    // Operations //////////////////////////////////////////////////////////////

    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    /// This instruction is only used on the old computers on which Chip-8 was
    /// originally implemented. It is ignored by modern interpreters.
    pub(super) fn op_0nnn(&mut self, _opcode: u16) {
        println!("0nnn instruction received - ignored");
        // NO-OP Ignored
    }

    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    pub(super) fn op_1nnn(&mut self, opcode: u16) {
        let location = opcode & 0x0fff;

        self.pc = location;
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the
    /// top of the stack. The PC is then set to nnn.
    pub(super) fn op_2nnn(&mut self, opcode: u16) {
        let location = opcode & 0x0fff;

        self.sp += 1;
        self.stack.push(self.pc);
        self.pc = location;
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    pub(super) fn op_3xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        if self.v[x] == kk {
            self.pc += 2
        }
    }

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    pub(super) fn op_4xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        if self.v[x] != kk {
            self.pc += 2
        }
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal,
    /// increments the program counter by 2.
    pub(super) fn op_5xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 8) as usize;

        if self.v[x] == self.v[y] {
            self.pc += 2
        }
    }
    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    pub(super) fn op_6xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        self.v[x] = kk;
    }

    ///7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    pub(super) fn op_7xkk(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;

        self.v[x] += kk;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_op_test_0nnn() {
        let mut chip = Chip8::new();

        let last_pc = chip.pc;
        let last_sp = chip.sp;

        chip.decode_op(0x0234);

        assert_eq!(chip.pc, last_pc);
        assert_eq!(chip.sp, last_sp);
    }
    #[test]
    fn decode_op_test_1nnn() {
        let mut chip = Chip8::new();
        chip.decode_op(0x1234);
        assert_eq!(chip.pc, 0x234);
    }
    #[test]
    fn decode_op_test_2nnn() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;

        chip.decode_op(0x2345);

        assert_eq!(chip.sp, 1);
        assert_eq!(chip.stack[0], 0x300);
        assert_eq!(chip.pc, 0x345);
    }
    #[test]
    fn decode_op_test_op_3xkk_x_equals_kk() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;
        chip.v[4] = 0x05;

        chip.decode_op(0x3405);

        assert_eq!(chip.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_3xkk_x_not_equals_kk() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;
        chip.v[4] = 0x05;

        chip.decode_op(0x3403);

        assert_eq!(chip.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_4xkk_x_equals_kk() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;
        chip.v[4] = 0x05;

        chip.decode_op(0x4405);

        assert_eq!(chip.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_4xkk_x_not_equals_kk() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;
        chip.v[4] = 0x05;

        chip.decode_op(0x4403);

        assert_eq!(chip.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_5xy0_vx_equals_vy() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;
        chip.v[4] = 0x05;
        chip.v[0] = 0x05
        ;
        chip.decode_op(0x5400);

        assert_eq!(chip.pc, 0x302);
    }
    #[test]
    fn decode_op_test_op_5xy0_vx_not_equals_vy() {
        let mut chip = Chip8::new();
        chip.pc = 0x300;
        chip.v[4] = 0x05;
        chip.v[0] = 0x03;

        chip.decode_op(0x5400);

        assert_eq!(chip.pc, 0x300);
    }
    #[test]
    fn decode_op_test_op_6xkk() {
        let mut chip = Chip8::new();
        chip.v[4] = 0x05;

        chip.decode_op(0x6483);

        assert_eq!(chip.v[4], 0x83);
    }
    #[test]
    fn decode_op_test_op_7xkk() {
        let mut chip = Chip8::new();
        chip.v[4] = 0x05;

        chip.decode_op(0x7483);

        assert_eq!(chip.v[4], 0x88);
    }

}
