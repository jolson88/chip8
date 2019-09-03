use crate::cpu::{register_from_u8, Register};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    LoadConstant(Register, u8),
}

pub fn opcode_from_binary(hi_byte: u8, lo_byte: u8) -> Opcode {
    // We can use the top 4-bits of the opcode as a switch into the type of opcode for easier parsing
    let opcode_switch = hi_byte >> 4;
    match opcode_switch {
        6 => {
            Opcode::LoadConstant(register_from_u8(hi_byte & 0x0F), lo_byte)
        },
        _ => {
            unimplemented!("Opcode not recognized: 0x{:X}{:X}", hi_byte, lo_byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_opcodes() {
        assert_eq!(Opcode::LoadConstant(Register::VA, 0x02), opcode_from_binary(0x6A, 0x02));
        assert_eq!(Opcode::LoadConstant(Register::V0, 0xFF), opcode_from_binary(0x60, 0xFF));
    }
}