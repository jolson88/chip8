#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF
}

// TODO: This is pretty much a hack until FromPrimitive is brought into stable
// (it's currently experimental at this time of writing).
pub fn register_from_u8(val: u8) -> Register {
    match val {
        0 => Register::V0,
        1 => Register::V1,
        2 => Register::V2,
        3 => Register::V3,
        4 => Register::V4,
        5 => Register::V5,
        6 => Register::V6,
        7 => Register::V7,
        8 => Register::V8,
        9 => Register::V9,
        10 => Register::VA,
        11 => Register::VB,
        12 => Register::VC,
        13 => Register::VD,
        14 => Register::VE,
        15 => Register::VF,
        _ => {
            // Trying to parse a register value for anything over 15 is a programming error.
            // Fail fast!
            panic!("Invalid register value");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_map_to_u8() {
        assert_eq!(0, Register::V0 as u8);
        assert_eq!(10, Register::VA as u8);
        assert_eq!(15, Register::VF as u8);
    }
}