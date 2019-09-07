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
    VF = 15,
}

pub struct Chip8 {
    memory: Box<[u8; 4096]>,
    reg: [u8; 16],
    pc: u16,
    stack: Vec<u16>,
    i: u16,
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            memory: Box::new([0u8; 4096]),
            reg: [0u8; 16],
            pc: 0,
            stack: Vec::new(),
            i: 0,
        }
    }
}

impl Chip8 {
    pub fn load_program(&mut self, data: &[u8]) {
        let dest = &mut self.memory[0x200..0x200 + data.len()];
        dest.copy_from_slice(data);
    }

    pub fn tick(&self) {

    }
}