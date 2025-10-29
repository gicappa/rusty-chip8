use crate::chip8::Chip8;

impl Chip8 {
    // Operations //////////////////////////////////////////////////////////////

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
