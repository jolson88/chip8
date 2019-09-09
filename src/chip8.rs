use crate::opcode::Opcode;
use std::cmp::max;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
// Following font is pulled from: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.1
#[rustfmt::skip]
const FONT: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
const BASE_FONT_ADDRESS: usize = 0x000;

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
    pc: usize,
    stack: Vec<usize>,
    i_addr: usize,
    delay_timer: u8,
    sound_timer: u8,
    screen: Box<[u8; SCREEN_WIDTH * SCREEN_HEIGHT]>,
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut c8 = Chip8 {
            memory: Box::new([0u8; 4096]),
            reg: [0u8; 16],
            pc: 0x200,
            stack: Vec::new(),
            i_addr: 0,
            delay_timer: 0,
            sound_timer: 0,
            screen: Box::new([0u8; SCREEN_WIDTH * SCREEN_HEIGHT]),
        };

        // Load system font. 16 characters, each 5 bytes long
        for i in 0..16 {
            for j in 0..5 {
                c8.memory[BASE_FONT_ADDRESS + (i * 5) + j] = FONT[(i * 5) + j];
            }
        }
        c8
    }
}

impl Chip8 {
    pub fn load_program(&mut self, data: &[u8]) {
        let dest = &mut self.memory[0x200..0x200 + data.len()];
        dest.copy_from_slice(data);
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.delay_timer = max(self.delay_timer - 1, 0);
        self.sound_timer = max(self.sound_timer - 1, 0);

        // Similar to EAP register in x86, we will increment PC counter after retrieval
        // but before execution. This will help make it more straightforward for branch
        // instructions to "skip next instruction" by incrementing a single two-byte instruction.
        let op = Opcode::from(
            u16::from(self.memory[self.pc]) << 8 | u16::from(self.memory[self.pc + 1]),
        );
        self.pc += 2;
        self.execute_opcode(op)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.screen[y * SCREEN_WIDTH + x]
    }

    // Optimistically execute opcode. For the sake of this emulator, we just let the Vecs panic!
    // in the case of out-of-range indices instead of gracefully handling it. This way, it's
    // "fail fast" and should also help us identify logic errors in our implementation earlier.
    fn execute_opcode(&mut self, op: Opcode) -> Result<(), Box<dyn std::error::Error>> {
        match op {
            Opcode::ClearDisplay => {
                self.screen.iter_mut().for_each(|x| *x = 0);
            }
            Opcode::Noop => {
                // Do nothing
            }
            Opcode::Return => {
                let sp = self
                    .stack
                    .pop()
                    .ok_or_else(|| "Tried to return from empty stack")?;
                self.pc = sp;
            }
            Opcode::Jump(nnn) => {
                self.pc = nnn;
            }
            Opcode::CallSubroutine(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Opcode::SkipIfConstantEqual(vx, kk) => {
                if self.reg[vx as usize] == kk {
                    self.pc += 2;
                }
            }
            Opcode::SkipIfConstantNotEqual(vx, kk) => {
                if self.reg[vx as usize] != kk {
                    self.pc += 2;
                }
            }
            Opcode::SkipIfRegistersEqual(vx, vy) => {
                if self.reg[vx as usize] == self.reg[vy as usize] {
                    self.pc += 2;
                }
            }
            Opcode::LoadConstant(vx, kk) => {
                self.reg[vx as usize] = kk;
            }
            Opcode::AddConstant(vx, kk) => {
                let r = self.reg[vx as usize];
                // Wrap on overflow
                self.reg[vx as usize] = (u16::from(r) + u16::from(kk) % 256) as u8;
            }
            Opcode::LoadRegister(vx, vy) => {
                self.reg[vx as usize] = self.reg[vy as usize];
            }
            Opcode::Or(vx, vy) => {
                self.reg[vx as usize] |= self.reg[vy as usize];
            }
            Opcode::And(vx, vy) => {
                self.reg[vx as usize] &= self.reg[vy as usize];
            }
            Opcode::Xor(vx, vy) => {
                self.reg[vx as usize] ^= self.reg[vy as usize];
            }
            Opcode::AddRegister(vx, vy) => {
                let vx16 = u16::from(self.reg[vx as usize]);
                let vy16 = u16::from(self.reg[vy as usize]);
                let val = vx16 + vy16;
                if val > 255 {
                    self.reg[Register::VF as usize] = 1;
                }
                self.reg[vx as usize] = (val & 0xFF) as u8;
            }
            Opcode::SubtractRightRegister(vx, vy) => {
                let vx_val = self.reg[vx as usize];
                let vy_val = self.reg[vy as usize];
                if vx_val > vy_val {
                    self.reg[Register::VF as usize] = 1;
                }
                self.reg[vx as usize] = vx_val - vy_val;
            }
            Opcode::ShiftRight(vx) => {
                let vx_val = self.reg[vx as usize];
                if vx_val & 0x01 == 1 {
                    // Lease significant bit of 1 was shifted off, signal in VF register
                    self.reg[Register::VF as usize] = 1;
                }
                self.reg[vx as usize] = vx_val >> 1;
            }
            Opcode::SubtractLeftRegister(vx, vy) => {
                let vx_val = self.reg[vx as usize];
                let vy_val = self.reg[vy as usize];
                if vy_val > vx_val {
                    self.reg[Register::VF as usize] = 1;
                }
                self.reg[vx as usize] = vy_val - vx_val;
            }
            Opcode::ShiftLeft(vx) => {
                let vx_val = self.reg[vx as usize];
                if vx_val & 0b1000_0000 == 0b1000_0000 {
                    // Most significant bit of 1 was shifted off, signal in VF register
                    self.reg[Register::VF as usize] = 1;
                }
                self.reg[vx as usize] = vx_val << 1;
            }
            Opcode::SkipIfRegistersNotEqual(vx, vy) => {
                if self.reg[vx as usize] != self.reg[vy as usize] {
                    self.pc += 2;
                }
            }
            Opcode::LoadAddress(nnn) => {
                self.i_addr = nnn;
            }
            Opcode::JumpPlus(nnn) => {
                self.pc = self.reg[Register::V0 as usize] as usize + nnn;
            }
            Opcode::Random(vx, kk) => {
                self.reg[vx as usize] = rand::random::<u8>() & kk;
            }
            Opcode::DisplaySprite(vx, vy, n) => {
                let x = self.reg[vx as usize];
                let y = self.reg[vy as usize];

                let mut collision = false;
                for y_offset in 0..n {
                    // Sprites are N bytes (bit-coded for the 8 pixels across; so a single byte per "line".
                    let sprite_line = self.memory[self.i_addr + (y_offset as usize)];
                    for x_offset in 0..8 {
                        // When drawing, sprites wrap-around in the case of overflow
                        let dest_x = (x + x_offset) % (SCREEN_WIDTH as u8);
                        let dest_y = (y + y_offset) % (SCREEN_HEIGHT as u8);
                        let dest_index = dest_y as usize * SCREEN_WIDTH + dest_x as usize;

                        // most significant bit is the "leftmost" sprite bit
                        let bit = 7 - x_offset;
                        let sprite_pixel = (sprite_line >> bit) & 0x1;
                        if (sprite_pixel == 1) && (self.screen[dest_index] == 1) {
                            collision = true;
                        }
                        self.screen[dest_index] ^= sprite_pixel;
                    }
                }
                if collision {
                    self.reg[Register::VF as usize] = 1;
                }
            }
            Opcode::SkipIfPressed(_vx) => {
                // TODO(jolson): Implement input
            }
            Opcode::SkipIfNotPressed(_vx) => {
                // TODO(jolson): Implement input
            }
            Opcode::LoadDelayTimer(vx) => {
                self.reg[vx as usize] = self.delay_timer;
            }
            Opcode::WaitForPress(_vx) => {
                // TODO(jolson): Implement input
            }
            Opcode::SetDelayTimer(vx) => {
                self.delay_timer = self.reg[vx as usize];
            }
            Opcode::SetSoundTimer(vx) => {
                self.sound_timer = self.reg[vx as usize];
            }
            Opcode::AddAddress(vx) => {
                self.i_addr += self.reg[vx as usize] as usize;
            }
            Opcode::LoadAddressOfSprite(vx) => {
                // Each built-in character is 5-bytes long
                self.i_addr = BASE_FONT_ADDRESS + ((self.reg[vx as usize] * 5) as usize);
            }
            Opcode::LoadDigits(vx) => {
                let val = self.reg[vx as usize];
                self.memory[self.i_addr] = val / 100;
                self.memory[self.i_addr + 1] = val / 10 % 10;
                self.memory[self.i_addr + 2] = val % 10;
            }
            Opcode::StoreRegisters(vx) => {
                for i in 0..(vx as usize) {
                    self.memory[self.i_addr + i] = self.reg[i];
                }
            }
            Opcode::LoadRegisters(vx) => {
                for i in 0..(vx as usize) {
                    self.reg[i] = self.memory[self.i_addr + i];
                }
            }
        }

        Ok(())
    }
}
