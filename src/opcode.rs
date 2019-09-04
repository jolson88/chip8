use std::convert::From;
use num_traits::FromPrimitive;
use crate::cpu::Register;

/// Represents different opcodes that the Chip-8 can execute.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// The address to load from into I
    LoadAddress(u16),
    /// The constant value to load into the Register
    LoadConstant(Register, u8),
}

impl From<u16> for Opcode {
    /// Converts a u16 into an Opcode. Takes a u16 as all Chip-8 instructions are 2-bytes.
    fn from(inst: u16) -> Self {
        // We can use the top 4-bits of the opcode as a switch into the type of opcode for easier parsing
        let opcode_switch = inst >> 12;
        match opcode_switch {
            0x6 => {
                Opcode::LoadConstant(Register::from_u8((inst >> 8) as u8 & 0x0F).unwrap(), (inst & 0xFF) as u8)
            },
            0xA => {
                // The address to load from is the bottom 12-bits
                Opcode::LoadAddress(inst & 0xFFF)
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
    fn parses_load_opcodes() {
        assert_eq!(Opcode::LoadConstant(Register::VA, 0x02), Opcode::from(0x6A02));
        assert_eq!(Opcode::LoadConstant(Register::V0, 0xFF), Opcode::from(0x60FF));
        assert_eq!(Opcode::LoadAddress(0x2EA), Opcode::from(0xA2EA));
    }
}