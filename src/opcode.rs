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
    /// *Fx07 - LD Vx, DT*. Placed the value of delay timer into register Vx.
    LoadDelayTimer(Register),
    /// *Fx15 - LD DT, Vx*. Set delay timer  = Vx.
    SetDelayTimer(Register),
}

impl From<u16> for Opcode {
    /// Converts a u16 into an Opcode. Takes a u16 as all Chip-8 instructions are 2-bytes.
    fn from(inst: u16) -> Self {
        // We can use the top 4-bits of the opcode as a switch into the type of opcode for easier parsing
        let opcode_switch = inst >> 12;
        match opcode_switch {
            0x2 => {
                // 2nnn
                let nnn = inst & 0xFFF;
                Opcode::CallSubroutine(nnn)
            }
            0x6 => {
                // 6xkk
                let x = (inst >> 8) & 0x0F;
                let kk = inst & 0xFF;
                Opcode::LoadConstant(Register::from_u8(x as u8).unwrap(), kk as u8)
            }
            0xA => {
                // Annn
                let nnn = inst & 0xFFF;
                Opcode::LoadAddress(nnn)
            }
            0xD => {
                // Dxyn
                let x = (inst >> 8) & 0x0F;
                let y = (inst >> 4) & 0x0F;
                let n = inst & 0x0F;
                Opcode::DisplaySprite(
                    Register::from_u8(x as u8).unwrap(),
                    Register::from_u8(y as u8).unwrap(),
                    n as u8,
                )
            }
            0xF => {
                // lo byte represents the next opcode information
                match inst & 0xFF {
                    0x07 => {
                        // Fx07
                        let x = (inst >> 8) & 0x0F;
                        Opcode::LoadDelayTimer(Register::from_u8(x as u8).unwrap())
                    }
                    0x15 => {
                        // Fx15
                        let x = (inst >> 8) & 0x0F;
                        Opcode::SetDelayTimer(Register::from_u8(x as u8).unwrap())
                    }
                    _ => {
                        panic!("Instruction not recognized: {:X}", inst);
                    }
                }
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

    #[test]
    fn parses_timer_opcodes() {
        assert_eq!(
            Opcode::SetDelayTimer(Register::V0),
            Opcode::from(0xF015)
        );
        assert_eq!(
            Opcode::LoadDelayTimer(Register::V0),
            Opcode::from(0xF007)
        );
    }
}
