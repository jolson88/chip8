#[derive(Copy, Clone, Debug, PartialEq, Eq, Primitive)]
pub enum Register {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    VA = 10,
    VB = 11,
    VC = 12,
    VD = 13,
    VE = 14,
    VF = 15
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