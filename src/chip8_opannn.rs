use crate::chip8::Chip8;

impl Chip8 {
    // Operations //////////////////////////////////////////////////////////////

    /// Annn - LD I, addr
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    pub(super) fn op_annn(&mut self, opcode: u16) {
        let nnn = opcode & 0x0fff;

        self.i = nnn;
    }
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
}
