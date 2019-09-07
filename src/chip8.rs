use crate::opcode::Opcode;

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
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            memory: Box::new([0u8; 4096]),
            reg: [0u8; 16],
            pc: 0x200,
            stack: Vec::new(),
            i_addr: 0,
        }
    }
}

impl Chip8 {
    pub fn load_program(&mut self, data: &[u8]) {
        let dest = &mut self.memory[0x200..0x200 + data.len()];
        dest.copy_from_slice(data);
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Similar to EAP register in x86, we will increment PC counter after retrieval
        // but before execution. This will help make it more straightforward for branch
        // instructions to "skip next instruction" by incrementing a single two-byte instruction.
        let op = Opcode::from(
            u16::from(self.memory[self.pc]) << 8 | u16::from(self.memory[self.pc + 1]),
        );
        self.pc += 2;
        self.execute_opcode(op)
    }

    // Optimistically execute opcode. For the sake of this emulator, we just let the Vecs panic!
    // in the case of out-of-range indices instead of gracefully handling it. This way, it's
    // "fail fast" and should also help us identify logic errors in our implementation earlier.
    fn execute_opcode(&mut self, op: Opcode) -> Result<(), Box<dyn std::error::Error>> {
        match op {
            Opcode::ClearDisplay => {
                // TODO(jolson): Zero out frame buffer
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
            Opcode::Jump(_nnn) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::CallSubroutine(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Opcode::SkipIfConstantEqual(_vx, _kk) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SkipIfConstantNotEqual(_vx, _kk) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SkipIfRegistersEqual(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::LoadConstant(vx, kk) => {
                self.reg[vx as usize] = kk;
            }
            Opcode::AddConstant(_vx, _kk) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::LoadRegister(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::Or(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::And(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::Xor(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::AddRegister(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SubtractRightRegister(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::ShiftRight(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SubtractLeftRegister(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::ShiftLeft(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SkipIfRegistersNotEqual(_vx, _vy) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::LoadAddress(nnn) => {
                self.i_addr = nnn;
            }
            Opcode::JumpPlus(_nnn) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::Random(_vx, _kk) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::DisplaySprite(_vx, _vy, _n) => {
                // TODO(jolson): Implement sprite drawing
            }
            Opcode::SkipIfPressed(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SkipIfNotPressed(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::LoadDelayTimer(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::WaitForPress(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SetDelayTimer(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::SetSoundTimer(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::AddAddress(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
            }
            Opcode::LoadAddressOfSprite(_vx) => {
                unimplemented!("Opcode not implemented yet: {:?}", op);
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
