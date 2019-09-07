use crate::chip8::Register;
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

    pub fn nnn(&self) -> usize {
        (self.0 & 0xFFF) as usize
    }

    pub fn n(&self) -> u8 {
        (self.0 & 0x0F) as u8
    }
}

/// Represents different opcodes that the Chip-8 can execute.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// *00E0 - CLS*. Clear the display.
    ClearDisplay,
    /// *00EE - RET*. Return from a subroutine.
    Return,
    /// *0nnn - SYS addr*. WHile a valid intsruction, this is typically a noop in modern interpreters.
    Noop,
    /// *1nnn - JP addr*. Jump to location nnn.
    Jump(usize),
    /// *2nnn - CALL addr*. Calls subroutine at nnn.
    CallSubroutine(usize),
    /// *3xkk - SE Vx, byte*. Skip next instruction if Vx = kk.
    SkipIfConstantEqual(Register, u8),
    /// *4xkk - SNE Vx, byte*. Skip next instruction if Vx != kk.
    SkipIfConstantNotEqual(Register, u8),
    /// *5xy0 - SE Vx, Vy*. Skip next instruction if Vx = Vy.
    SkipIfRegistersEqual(Register, Register),
    /// *6xkk - LD Vx, byte*. Puts the value kk into register Vx.
    LoadConstant(Register, u8),
    /// *7xkk - ADD Vx, byte*. Adds kk to register Vx.
    AddConstant(Register, u8),
    /// *8xy0 - LD Vx, Vy*. Sets register Vx to value in register Vy.
    LoadRegister(Register, Register),
    /// *8xy1 - OR Vx, Vy*. Performs bitwise OR between Vx and Vy, then stores result in Vx.
    Or(Register, Register),
    /// *8xy2 - AND Vx, Vy*. Performs bitwise AND between Vx and Vy, then stores result in Vx.
    And(Register, Register),
    /// *8xy3 - XOR Vx, Vy*. Performs bitwise XOR between Vx and Vy, then stores result in Vx.
    Xor(Register, Register),
    /// *8xy4 - ADD Vx, Vy*. Adds the values of register Vx and Vy together.
    AddRegister(Register, Register),
    /// *8xy5 - SUB Vx, Vy*. Subtracts the value of register Vy from register Vx, then stores result in Vx.
    SubtractRightRegister(Register, Register),
    /// *8xy6 - SHR Vx*. Shifts the value of register Vx to the right by 1.
    ShiftRight(Register),
    /// *8xy7 - SUBN Vx, Vy*. Substracts the value of register Vx from register Vy, then stores result in Vx.
    SubtractLeftRegister(Register, Register),
    /// *8xyE - SHL Vx*. Shifts the value of register Vx to the left by 1.
    ShiftLeft(Register),
    /// *9xy0 - SNE Vx, Vy*. Skip next instruction if registers Vx and Vy are not equal.
    SkipIfRegistersNotEqual(Register, Register),
    /// *Annn - LD I, addr*. Sets the value of I register to nnn.
    LoadAddress(usize),
    /// *Bnnn - JP V0, addr*. Jump to location nnn + V0.
    JumpPlus(usize),
    /// *Cxkk - RND Vx, byte*. Generates random number betweeen 0 and 255, AND it with the value kk, then stores result in Vx.
    Random(Register, u8),
    /// *Dxyn - DRW Vx, Vy, nibble*. Displays n-byte sprite starting at memory location I at (Vx, Vy).
    DisplaySprite(Register, Register, u8),
    /// *Ex9E - SKP Vx*. Skip next instruction if key with value Vx is pressed.
    SkipIfPressed(Register),
    /// *ExA1 - SKNP Vx*. Skip next instruction if key with value Vx is not pressed.
    SkipIfNotPressed(Register),
    /// *Fx07 - LD Vx, DT*. Placed the value of delay timer into register Vx.
    LoadDelayTimer(Register),
    /// *Fx0A - LD Vx, K*. Wait for a key press, store the value in register Vx.
    WaitForPress(Register),
    /// *Fx15 - LD DT, Vx*. Set delay timer = Vx.
    SetDelayTimer(Register),
    /// *Fx18 - LD ST, Vx*. Set sound timer = Vx.
    SetSoundTimer(Register),
    /// *Fx1E - ADD I, Vx*. The values of I and register Vx are added, then stores result in Vx.
    AddAddress(Register),
    /// *Fx29 - LD F, Vx*. The value of I is set to the location of sprite for digit Vx.
    LoadAddressOfSprite(Register),
    /// *Fx33 - LD B, Vx*. Store BCD representation of Vx in addresses I, I+1, and I+2.
    LoadDigits(Register),
    /// *Fx55 - LD [I], Vx*. Store registers V0 through Vx in memory starting at location I.
    StoreRegisters(Register),
    /// *Fx65 - LD Vx, [I]*. Load registers V0 through Vx from memory starting at location I.
    LoadRegisters(Register),
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
            0x8 => {
                match inst.raw() & 0x0F {
                    0x0 => {
                        // 8xy0
                        Opcode::LoadRegister(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0x1 => {
                        // 8xy1
                        Opcode::Or(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0x2 => {
                        // 8xy2
                        Opcode::And(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0x3 => {
                        // 8xy3
                        Opcode::Xor(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0x4 => {
                        // 8xy4
                        Opcode::AddRegister(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0x5 => {
                        // 8xy5
                        Opcode::SubtractRightRegister(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0x6 => {
                        // 8xy6
                        // TODO: Verify whether it is valid to use Register Y to specify amount to shift by
                        Opcode::ShiftRight(Register::from_u8(inst.x()).unwrap())
                    }
                    0x7 => {
                        // 8xy7
                        Opcode::SubtractLeftRegister(
                            Register::from_u8(inst.x()).unwrap(),
                            Register::from_u8(inst.y()).unwrap(),
                        )
                    }
                    0xE => {
                        // 8xyE
                        // TODO: Verify whether it is valid to use Register Y to specify amount to shift by
                        Opcode::ShiftLeft(Register::from_u8(inst.x()).unwrap())
                    }
                    _ => {
                        panic!("Instruction not recognized: {:X}", inst.raw());
                    }
                }
            }
            0x9 => {
                // 9xy0
                Opcode::SkipIfRegistersNotEqual(
                    Register::from_u8(inst.x()).unwrap(),
                    Register::from_u8(inst.y()).unwrap(),
                )
            }
            0xA => {
                // Annn
                Opcode::LoadAddress(inst.nnn())
            }
            0xB => {
                // Bnnn
                Opcode::JumpPlus(inst.nnn())
            }
            0xC => {
                // Cxkk
                Opcode::Random(Register::from_u8(inst.x()).unwrap(), inst.kk())
            }
            0xD => {
                // Dxyn
                Opcode::DisplaySprite(
                    Register::from_u8(inst.x()).unwrap(),
                    Register::from_u8(inst.y()).unwrap(),
                    inst.n(),
                )
            }
            0xE => {
                match inst.raw() & 0xFF {
                    0x9E => {
                        // Ex9E
                        Opcode::SkipIfPressed(Register::from_u8(inst.x()).unwrap())
                    }
                    0xA1 => {
                        // ExA1
                        Opcode::SkipIfNotPressed(Register::from_u8(inst.x()).unwrap())
                    }
                    _ => {
                        panic!("Instruction not recognized: {:X}", inst.raw());
                    }
                }
            }
            0xF => {
                // lo byte represents the next opcode information
                match inst.raw() & 0xFF {
                    0x07 => {
                        // Fx07
                        Opcode::LoadDelayTimer(Register::from_u8(inst.x()).unwrap())
                    }
                    0x0A => {
                        // Fx0A
                        Opcode::WaitForPress(Register::from_u8(inst.x()).unwrap())
                    }
                    0x15 => {
                        // Fx15
                        Opcode::SetDelayTimer(Register::from_u8(inst.x()).unwrap())
                    }
                    0x18 => {
                        // Fx18
                        Opcode::SetSoundTimer(Register::from_u8(inst.x()).unwrap())
                    }
                    0x1E => {
                        // Fx1E
                        Opcode::AddAddress(Register::from_u8(inst.x()).unwrap())
                    }
                    0x29 => {
                        // Fx29
                        Opcode::LoadAddressOfSprite(Register::from_u8(inst.x()).unwrap())
                    }
                    0x33 => {
                        // Fx33
                        Opcode::LoadDigits(Register::from_u8(inst.x()).unwrap())
                    }
                    0x55 => {
                        // Fx55
                        Opcode::StoreRegisters(Register::from_u8(inst.x()).unwrap())
                    }
                    0x65 => {
                        // Fx65
                        Opcode::LoadRegisters(Register::from_u8(inst.x()).unwrap())
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
        assert_eq!(
            Opcode::LoadAddressOfSprite(Register::V4),
            Opcode::from(0xF429)
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
        assert_eq!(
            Opcode::SkipIfRegistersNotEqual(Register::V1, Register::V4),
            Opcode::from(0x9140)
        );
        assert_eq!(Opcode::JumpPlus(0x17A), Opcode::from(0xB17A));
        assert_eq!(Opcode::SkipIfPressed(Register::V9), Opcode::from(0xE99E));
        assert_eq!(Opcode::SkipIfNotPressed(Register::VE), Opcode::from(0xEEA1));
        assert_eq!(Opcode::WaitForPress(Register::VA), Opcode::from(0xFA0A));
    }

    #[test]
    fn parses_memory_opcodes() {
        assert_eq!(
            Opcode::LoadConstant(Register::VA, 0x02),
            Opcode::from(0x6A02)
        );
        assert_eq!(
            Opcode::LoadConstant(Register::V0, 0xFF),
            Opcode::from(0x60FF)
        );
        assert_eq!(Opcode::LoadAddress(0x2EA), Opcode::from(0xA2EA));
        assert_eq!(
            Opcode::LoadRegister(Register::V1, Register::V2),
            Opcode::from(0x8120)
        );
        assert_eq!(Opcode::LoadDigits(Register::VA), Opcode::from(0xFA33));
        assert_eq!(Opcode::StoreRegisters(Register::V9), Opcode::from(0xF955));
        assert_eq!(Opcode::LoadRegisters(Register::VD), Opcode::from(0xFD65));
    }

    #[test]
    fn parses_math_opcodes() {
        assert_eq!(
            Opcode::AddConstant(Register::V2, 0x3B),
            Opcode::from(0x723B)
        );
        assert_eq!(Opcode::Or(Register::V8, Register::VA), Opcode::from(0x88A1));
        assert_eq!(
            Opcode::And(Register::V1, Register::V3),
            Opcode::from(0x8132)
        );
        assert_eq!(
            Opcode::Xor(Register::V5, Register::VC),
            Opcode::from(0x85C3)
        );
        assert_eq!(
            Opcode::AddRegister(Register::V4, Register::V5),
            Opcode::from(0x8454)
        );
        assert_eq!(
            Opcode::SubtractRightRegister(Register::V2, Register::VA),
            Opcode::from(0x82A5)
        );
        assert_eq!(Opcode::ShiftRight(Register::V7), Opcode::from(0x8716));
        assert_eq!(
            Opcode::SubtractLeftRegister(Register::VA, Register::VC),
            Opcode::from(0x8AC7)
        );
        assert_eq!(Opcode::ShiftLeft(Register::V7), Opcode::from(0x87AE));
        assert_eq!(Opcode::Random(Register::V4, 0x14), Opcode::from(0xC414));
        assert_eq!(Opcode::AddAddress(Register::V8), Opcode::from(0xF81E));
    }

    #[test]
    fn parses_timer_opcodes() {
        assert_eq!(Opcode::SetDelayTimer(Register::V0), Opcode::from(0xF015));
        assert_eq!(Opcode::LoadDelayTimer(Register::V0), Opcode::from(0xF007));
        assert_eq!(Opcode::SetSoundTimer(Register::V3), Opcode::from(0xF318));
    }
}
