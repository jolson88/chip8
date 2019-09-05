use crate::cpu::Register;
use num_traits::FromPrimitive;
use std::convert::From;

struct Instruction(u16);

impl Instruction {
    pub fn raw(&self) -> u16 {
        self.0
    }

    /// We can use the top 4-bits of the opcode as a switch into the type of opcode for easier parsing
    pub fn op(&self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub fn x(&self) -> u8 {
        ((self.0 >> 8) & 0x0F) as u8
    }

    pub fn y(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    pub fn kk(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn nnn(&self) -> u16 {
        self.0 & 0xFFF
    }

    pub fn n(&self) -> u8 {
        (self.0 & 0x0F) as u8
    }
}

/// Represents different opcodes that the Chip-8 can execute.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// *00E0 - CLS. Clear the display.
    ClearDisplay,
    /// *00EE - RET. Return from a subroutine.
    Return,
    /// *0nnn - SYS addr. WHile a valid intsruction, this is typically a noop in modern interpreters.
    Noop,
    /// *1nnn - JP addr. Jump to location nnn.
    Jump(u16),
    /// *2nnn - CALL addr. Calls subroutine at nnn.
    CallSubroutine(u16),
    /// *3xkk - SE Vx, byte. Skip next instruction if Vx = kk.
    SkipIfConstantEqual(Register, u8),
    /// *4xkk - SNE Vx, byte. Skip next instruction if Vx != kk.
    SkipIfConstantNotEqual(Register, u8),
    /// *5xy0 - SE Vx, Vy. Skip next instruction if Vx = Vy.
    SkipIfRegistersEqual(Register, Register),
    /// *6xkk - LD Vx, byte*. Puts the value kk into register Vx.
    LoadConstant(Register, u8),
    /// *7xkk - ADD Vx, byte. Adds kk to register Vx.
    AddConstant(Register, u8),
    /// *Annn - LD I, addr*. Sets the value of I register to nnn.
    LoadAddress(u16),
    /// *Dxyn - DRW Vx, Vy, nibble*. Displays n-byte sprite starting at memory location I at (Vx, Vy).
    DisplaySprite(Register, Register, u8),
    /// *Fx07 - LD Vx, DT*. Placed the value of delay timer into register Vx.
    LoadDelayTimer(Register),
    /// *Fx15 - LD DT, Vx*. Set delay timer = Vx.
    SetDelayTimer(Register),
}

impl From<u16> for Opcode {
    /// Converts a u16 into an Opcode. Takes a u16 as all Chip-8 instructions are 2-bytes.
    fn from(val: u16) -> Self {
        let inst = Instruction(val);
        match inst.op() {
            0x0 => {
                match inst.raw() & 0xFF {
                    0xE0 => Opcode::ClearDisplay,
                    0xEE => Opcode::Return,
                    _ => {
                        // Other commands that are now noops like 0nnn (SYS addr).
                        Opcode::Noop
                    }
                }
            }
            0x1 => {
                // 1nnn
                Opcode::Jump(inst.nnn())
            }
            0x2 => {
                // 2nnn
                Opcode::CallSubroutine(inst.nnn())
            }
            0x3 => {
                // 3xkk
                Opcode::SkipIfConstantEqual(Register::from_u8(inst.x()).unwrap(), inst.kk())
            }
            0x4 => {
                // 4xkk
                Opcode::SkipIfConstantNotEqual(Register::from_u8(inst.x()).unwrap(), inst.kk())
            }
            0x5 => {
                // 5xy0
                Opcode::SkipIfRegistersEqual(
                    Register::from_u8(inst.x()).unwrap(),
                    Register::from_u8(inst.y()).unwrap(),
                )
            }
            0x6 => {
                // 6xkk
                Opcode::LoadConstant(Register::from_u8(inst.x()).unwrap(), inst.kk())
            }
            0x7 => {
                // 7xkk
                Opcode::AddConstant(Register::from_u8(inst.x()).unwrap(), inst.kk())
            }
            0xA => {
                // Annn
                Opcode::LoadAddress(inst.nnn())
            }
            0xD => {
                // Dxyn
                Opcode::DisplaySprite(
                    Register::from_u8(inst.x()).unwrap(),
                    Register::from_u8(inst.y()).unwrap(),
                    inst.n(),
                )
            }
            0xF => {
                // lo byte represents the next opcode information
                match inst.raw() & 0xFF {
                    0x07 => {
                        // Fx07
                        Opcode::LoadDelayTimer(Register::from_u8(inst.x()).unwrap())
                    }
                    0x15 => {
                        // Fx15
                        Opcode::SetDelayTimer(Register::from_u8(inst.x()).unwrap())
                    }
                    _ => {
                        panic!("Instruction not recognized: {:X}", inst.raw());
                    }
                }
            }
            _ => {
                panic!("Instruction not recognized: {:X}", inst.raw());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_draw_opcodes() {
        assert_eq!(Opcode::ClearDisplay, Opcode::from(0x00E0));
        assert_eq!(
            Opcode::DisplaySprite(Register::VA, Register::VB, 0x6),
            Opcode::from(0xDAB6)
        );
    }

    #[test]
    fn parses_flow_opcodes() {
        assert_eq!(Opcode::CallSubroutine(0x2D4), Opcode::from(0x22D4));
        assert_eq!(Opcode::Jump(0x53A), Opcode::from(0x153A));
        assert_eq!(Opcode::Noop, Opcode::from(0x0123));
        assert_eq!(Opcode::Return, Opcode::from(0x00EE));
        assert_eq!(
            Opcode::SkipIfConstantEqual(Register::V7, 0x14),
            Opcode::from(0x3714)
        );
        assert_eq!(
            Opcode::SkipIfConstantNotEqual(Register::VA, 0xAE),
            Opcode::from(0x4AAE)
        );
        assert_eq!(
            Opcode::SkipIfRegistersEqual(Register::VA, Register::VD),
            Opcode::from(0x5AD0)
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
    fn parses_math_opcodes() {
        assert_eq!(
            Opcode::AddConstant(Register::V2, 0x3B),
            Opcode::from(0x723B)
        );
    }

    #[test]
    fn parses_timer_opcodes() {
        assert_eq!(Opcode::SetDelayTimer(Register::V0), Opcode::from(0xF015));
        assert_eq!(Opcode::LoadDelayTimer(Register::V0), Opcode::from(0xF007));
    }
}
