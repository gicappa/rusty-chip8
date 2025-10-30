use crate::chip8::Chip8;
use rand::random;

impl Chip8 {
    // Operations //////////////////////////////////////////////////////////////

    /// Annn - LD I, addr
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    pub(super) fn op_annn(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;

        self.i = nnn;
    }
    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    pub(super) fn op_bnnn(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;

        self.pc = (self.v[0] as u16) + nnn;
    }
    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value
    /// kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    pub(super) fn op_cxkk(&mut self, opcode: u16) {
        let _nnn = opcode & 0x0fff;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let kk = (opcode & 0x00ff) as u8;
        let rnd: u8 = random();

        self.v[x] = rnd & kk;
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
    pub(super) fn op_dxyn(&mut self, _opcode: u16) {

    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
    /// position, PC is increased by 2.
    pub(super) fn op_ex9e(&mut self, _opcode: u16) {}

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up
    /// position, PC is increased by 2.
    pub(super) fn op_exa1(&mut self, _opcode: u16) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_op_test_annn() {
        let mut chip = Chip8::new();
        chip.i = 0x444;

        chip.decode_op(0xA555);

        assert_eq!(chip.i, 0x555);
    }
    #[test]
    fn decode_op_test_bnnn() {
        let mut chip = Chip8::new();
        chip.pc = 0x400;
        chip.v[0] = 0x10;

        chip.decode_op(0xB500);

        assert_eq!(chip.pc, 0x510);
    }
    #[test]
    fn decode_op_test_cxkk_and_0() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x77;

        for _ in 0..5 {
            chip.decode_op(0xC500);
            assert_eq!(chip.v[5], 0x00);
        }
    }
    #[test]
    fn decode_op_test_cxkk_rnd() {
        let mut chip = Chip8::new();
        chip.v[5] = 0x77;

        let mut res: Vec<u8> = Vec::new();

        for _ in 0..9 {
            chip.decode_op(0xC5FF);
            res.push(chip.v[5]);
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
        assert!(false);
    }
    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
    /// position, PC is increased by 2.
    #[test]
    fn decode_op_test_ex9e() {
        assert!(false);
    }
    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    #[test]
    fn decode_op_test_exa1() {
        assert!(false);
    }

}
