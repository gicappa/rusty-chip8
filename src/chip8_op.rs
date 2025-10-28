use crate::chip8::Chip8;

impl Chip8 {
    pub fn decode_op(&mut self, opcode: u16) {
        match opcode {
            code if code & 0xf00f == 0x8000 => self.op_8xy0(opcode),
            code if code & 0xf00f == 0x8001 => self.op_8xy1(opcode),
            code if code & 0xf00f == 0x8002 => self.op_8xy2(opcode),
            _ => println!("Not matching"),
        }
    }

    // Helper //////////////////////////////////////////////////////////////////
    #[inline]
    fn regs_xy(opcode: u16) -> (usize, usize) {
        // Original form: (opcode & 0x0F00) >> 8; (opcode & 0x00F0) >> 4
        // Equivalent: (opcode >> 8) & 0xF; (opcode >> 4) & 0xF
        let x = ((opcode >> 8) & 0xF) as usize;
        let y = ((opcode >> 4) & 0xF) as usize;
        (x, y)
    }

    // Operations //////////////////////////////////////////////////////////////

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    fn op_8xy0(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        self.v[x] = self.v[y];
    }

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    fn op_8xy1(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        self.v[x] |= self.v[y];
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy2(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        self.v[x] &= self.v[y];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regs_xy_extracts_indices() {
        // opcode 0x8AB1 -> x = A (10), y = B (11)
        let (x, y) = Chip8::regs_xy(0x8AB1);
        assert_eq!(x, 0xA);
        assert_eq!(y, 0xB);
    }

    #[test]
    fn decode_op_test_8xy0() {
        let mut chip = Chip8::new();
        chip.v[3] = 0x08;
        chip.v[4] = 0x10;
        chip.decode_op(0x8340);
        assert_eq!(chip.v[3], 0x10);
        assert_eq!(chip.v[4], 0x10);
    }

    #[test]
    fn decode_op_test_8xy1() {
        let mut chip = Chip8::new();
        chip.v[2] = 0x40;
        chip.v[3] = 0xA8;
        chip.decode_op(0x8231);
        assert_eq!(chip.v[2], 0xE8);
    }
    #[test]
    fn decode_op_test_8xy2() {
        let mut chip = Chip8::new();
        chip.v[2] = 0xE8;
        chip.v[3] = 0x44;
        chip.decode_op(0x8232);
        assert_eq!(chip.v[2], 0x40);
    }
}

/*






8xy3 - XOR Vx, Vy
Set Vx = Vx XOR Vy.

Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.


8xy4 - ADD Vx, Vy
Set Vx = Vx + Vy, set VF = carry.

The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.


8xy5 - SUB Vx, Vy
Set Vx = Vx - Vy, set VF = NOT borrow.

If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.


8xy6 - SHR Vx {, Vy}
Set Vx = Vx SHR 1.

If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.


8xy7 - SUBN Vx, Vy
Set Vx = Vy - Vx, set VF = NOT borrow.

If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.


8xyE - SHL Vx {, Vy}
Set Vx = Vx SHL 1.

If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.


 */
