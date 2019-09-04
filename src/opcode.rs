use crate::cpu::Register;
use num_traits::FromPrimitive;
use std::convert::From;

/// Represents different opcodes that the Chip-8 can execute.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// *2nnn - CALL addr. Calls subroutine at nnn.
    CallSubroutine(u16),
    /// *Dxyn - DRW Vx, Vy, nibble*. Displays n-byte sprite starting at memory location I at (Vx, Vy).
    DisplaySprite(Register, Register, u8),
    /// *Annn - LD I, addr*. Sets the value of I register to nnn.
    LoadAddress(u16),
    /// *6xkk - LD Vx, byte*. Puts the value kk into register Vx.
    LoadConstant(Register, u8),
}

impl From<u16> for Opcode {
    /// Converts a u16 into an Opcode. Takes a u16 as all Chip-8 instructions are 2-bytes.
    fn from(inst: u16) -> Self {
        // We can use the top 4-bits of the opcode as a switch into the type of opcode for easier parsing
        let opcode_switch = inst >> 12;
        match opcode_switch {
            0x2 => {
                // 2nnn
                Opcode::CallSubroutine(inst & 0xFFF)
            },
            0x6 => {
                // 6xkk
                let r = (inst >> 8) & 0x0F;
                let c = inst & 0xFF;
                Opcode::LoadConstant(Register::from_u8(r as u8).unwrap(), c as u8)
            }
            0xA => {
                // Annn
                // The address to load from is the bottom 12-bits
                Opcode::LoadAddress(inst & 0xFFF)
            }
            0xD => {
                // Dxyn
                let r1 = (inst >> 8) & 0x0F;
                let r2 = (inst >> 4) & 0x0F;
                let n = inst & 0x0F;
                Opcode::DisplaySprite(
                    Register::from_u8(r1 as u8).unwrap(),
                    Register::from_u8(r2 as u8).unwrap(),
                    n as u8,
                )
            }
            _ => {
                unimplemented!("Instruction not recognized: {:X}", inst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_draw_opcodes() {
        assert_eq!(
            Opcode::DisplaySprite(Register::VA, Register::VB, 0x6),
            Opcode::from(0xDAB6)
        );
    }

    #[test]
    fn parses_flow_opcodes() {
        assert_eq!(
            Opcode::CallSubroutine(0x2D4),
            Opcode::from(0x22D4)
        );
    }

    #[test]
    fn parses_load_opcodes() {
        assert_eq!(
            Opcode::LoadConstant(Register::VA, 0x02),
            Opcode::from(0x6A02)
        );
        assert_eq!(
            Opcode::LoadConstant(Register::V0, 0xFF),
            Opcode::from(0x60FF)
        );
        assert_eq!(Opcode::LoadAddress(0x2EA), Opcode::from(0xA2EA));
    }
}
