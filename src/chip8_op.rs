use crate::chip8::Chip8;

impl Chip8 {
    pub fn decode_op(&mut self, opcode: u16) {
        match opcode {
            code if code & 0xf00f == 0x8000 => self.op_8xy0(opcode),
            code if code & 0xf00f == 0x8001 => self.op_8xy1(opcode),
            code if code & 0xf00f == 0x8002 => self.op_8xy2(opcode),
            code if code & 0xf00f == 0x8003 => self.op_8xy3(opcode),
            code if code & 0xf00f == 0x8004 => self.op_8xy4(opcode),
            code if code & 0xf00f == 0x8005 => self.op_8xy5(opcode),
            code if code & 0xf00f == 0x8006 => self.op_8xy6(opcode),
            code if code & 0xf00f == 0x8007 => self.op_8xy7(opcode),
            _ => println!("Not matching"),
        }
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
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corresponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy2(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        self.v[x] &= self.v[y];
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corresponding bits from two values, and if the bits are not
    /// both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn op_8xy3(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        self.v[x] ^= self.v[y]
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255)
    /// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_8xy4(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = sum;
        self.v[0xF] = if carry { 1 } else { 0 }
    }

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results
    /// stored in Vx.
    fn op_8xy5(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        let (diff, carry) = self.v[x].overflowing_sub(self.v[y]);
        if carry {
            self.v[x] = self.v[y] - self.v[x];
            self.v[0xF] = 0;
        } else {
            self.v[x] = diff;
            self.v[0xF] = 1;
        }
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    ///
    /// Actual implementation Vx=Vy=Vy>>1
    fn op_8xy6(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        self.v[x] = self.v[x] >> 1;
        self.v[y] = self.v[x];
        self.v[0xF] = self.v[x] & 0x1;
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy,
    /// and the results stored in Vx.
    fn op_8xy7(&mut self, opcode: u16) {
        let (x, y) = Self::regs_xy(opcode);

        let (diff, carry) = self.v[y].overflowing_sub(self.v[x]);
        if carry {
            self.v[x] = self.v[x] - self.v[y];
            self.v[0xF] = 0;
        } else {
            self.v[x] = diff;
            self.v[0xF] = 1;
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
    #[test]
    fn decode_op_test_8xy3() {
        let mut chip = Chip8::new();
        chip.v[5] = 0xE8;
        chip.v[6] = 0x56;
        chip.decode_op(0x8563);
        assert_eq!(chip.v[5], 0xBE);
    }
    #[test]
    fn decode_op_test_8xy4_no_carry() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x08;
        chip.v[6] = 0x56;
        chip.decode_op(0x8564);
        assert_eq!(chip.v[5], 0x5E);
        assert_eq!(chip.v[15], 0x0);
    }

    #[test]
    fn decode_op_test_8xy4_with_carry() {
        let mut chip = Chip8::new();
        chip.v[5] = 0xFF;
        chip.v[6] = 0x04;
        chip.decode_op(0x8564);
        assert_eq!(chip.v[5], 0x03);
        assert_eq!(chip.v[15], 0x1);
    }

    #[test]
    fn decode_op_test_8xy5_with_carry() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x5F;
        chip.v[6] = 0x14;
        chip.decode_op(0x8565);
        assert_eq!(chip.v[5], 0x4B);
        assert_eq!(chip.v[15], 0x1);
    }

    #[test]
    fn decode_op_test_8xy5_no_carry() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x14;
        chip.v[6] = 0x5F;
        chip.decode_op(0x8565);
        assert_eq!(chip.v[5], 0x4B);
        assert_eq!(chip.v[15], 0x0);
    }

    #[test]
    fn decode_op_test_8xy6_lsb_1() {
        let mut chip = Chip8::new();
        chip.v[5] = 0xEE;
        chip.v[6] = 0x5F;
        chip.decode_op(0x8566);
        assert_eq!(chip.v[5], 0x77);
        assert_eq!(chip.v[6], 0x77);
        assert_eq!(chip.v[0xF], 1);
    }

    #[test]
    fn decode_op_test_8xy6_lsb_0() {
        let mut chip = Chip8::new();
        chip.v[5] = 0xE0;
        chip.v[6] = 0x34;
        chip.decode_op(0x8566);
        assert_eq!(chip.v[5], 0x70);
        assert_eq!(chip.v[6], 0x70);
        assert_eq!(chip.v[0xF], 0);
    }

    #[test]
    fn decode_op_test_8xy7_with_carry() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x14;
        chip.v[6] = 0x5F;
        chip.decode_op(0x8567);
        assert_eq!(chip.v[5], 0x4B);
        assert_eq!(chip.v[15], 0x1);
    }

    #[test]
    fn decode_op_test_8xy7_no_carry() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x5F;
        chip.v[6] = 0x14;
        chip.decode_op(0x8567);
        assert_eq!(chip.v[5], 0x4B);
        assert_eq!(chip.v[15], 0x0);
    }
}

/*
8xyE - SHL Vx {, Vy}
Set Vx = Vx SHL 1.

If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.


 */
