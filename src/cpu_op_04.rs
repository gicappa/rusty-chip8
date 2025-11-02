use crate::cpu::CPU;

impl CPU {
    // Operations //////////////////////////////////////////////////////////////

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    pub(super) fn op_fx07(&mut self, _opcode: u16) {}

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    pub(super) fn op_fx0a(&mut self, _opcode: u16) {}

    ///Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    pub(super) fn op_fx15(&mut self, _opcode: u16) {}

    ///Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    pub(super) fn op_fx18(&mut self, _opcode: u16) {}

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    pub(super) fn op_fx1e(&mut self, _opcode: u16) {}

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value
    /// of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    pub(super) fn op_fx29(&mut self, _opcode: u16) {}

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at location I+2.
    pub(super) fn op_fx33(&mut self, _opcode: u16) {}
}

#[cfg(test)]
mod tests {
    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    #[test]
    fn decode_op_test_fx07() {
        assert!(false);
    }
    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    #[test]
    fn decode_op_test_fx0a() {
        assert!(false);
    }
    ///Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    #[test]
    fn decode_op_test_fx15() {
        assert!(false);
    }
    ///Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    #[test]
    fn decode_op_test_fx18() {
        assert!(false);
    }
    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    #[test]
    fn decode_op_test_fx1e() {
        assert!(false);
    }
    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value
    /// of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    #[test]
    fn decode_op_test_fx29() {
        assert!(false);
    }
    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at location I+2.
    #[test]
    fn decode_op_test_fx33() {
        assert!(false);
    }
}
/*

Fx55 - LD [I], Vx
Store registers V0 through Vx in memory starting at location I.

The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.


Fx65 - LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.

The interpreter reads values from memory starting at location I into registers V0 through Vx.
 */
