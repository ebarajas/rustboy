use crate::mmu::MMU;
use crate::registers::Registers;

struct Cpu {
    reg: Registers,
    mmu: MMU,
    pc: u8,
    sp: u8,
}

impl Cpu {
    fn fetch(index: u8) -> u8 {
        let val = self.mmu.ram[index];
        self.pc += 1;
        val;
    }

    fn halt() {}

    fn exec(&self, opcode: u8) {
        // https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
        match opcode {
            0x00 => {} // NOOP
            // HALT
            0x76 => {
                self.halt()
            }
            // LD B, B
            0x40 => {
                self.reg.set(1, self.reg.b())
            }
            // LD B, C
            0x41 => {
                self.reg.set(1, self.reg.c())
            }
            // LD B, D
            0x42 => {
                self.reg.set(1, self.reg.d())
            }
            // LD B, E
            0x43 => {
                self.reg.set(1, self.reg.e())
            }
            // LD B, H
            0x44 => {
                self.reg.set(1, self.reg.h())
            }
            // LD B, L
            0x45 =>{
                self.reg.set(1, self.reg.l())
            }
            _ => panic!("opcode not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {}
