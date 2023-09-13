use crate::mmu::MMU;

#[derive(Default)]
struct Reg {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

struct CPU {
    reg: Reg,
    pc: u8,
    sp: u8,
    mmu: MMU,
}

impl CPU {
    fn new() -> Self {
        Self {
            reg: Reg::default(),
            mmu: MMU::new(),
            pc: 0,
            sp: 0,
        }
    }

    fn fetch(&mut self, addr: u8) -> u8 {
        let val = self.mmu.get(addr.into());
        self.pc += 1;
        return val;
    }

    fn halt(&self) {}

    fn exec(&mut self, opcode: u8) {
        // https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
        match opcode {
            0x00 => {} // NOOP
            // HALT
            0x76 => self.halt(),
            // LD B, B
            0x40 => self.reg.b = self.reg.b,
            // LD B, C
            0x41 => self.reg.b = self.reg.c,
            // LD B, D
            0x42 => self.reg.b = self.reg.d,
            // LD B, E
            0x43 => self.reg.b = self.reg.e,
            // LD B, H
            0x44 => self.reg.b = self.reg.h,
            // LD B, L
            0x45 => self.reg.b = self.reg.l,
            // LD B, HL
            0x46 => {} // TODO
            // LD B, A
            0x47 => self.reg.b = self.reg.a,
            // LD C, B
            0x48 => self.reg.c = self.reg.b,
            // LD C, C
            0x49 => self.reg.c = self.reg.c,
            // LD C, D
            0x4A => self.reg.c = self.reg.d,
            // LD C, E
            0x4B => self.reg.c = self.reg.e,
            // LD C, H
            0x4C => self.reg.c = self.reg.h,
            // LD C, L
            0x4D => self.reg.c = self.reg.l,
            // LD C, HL
            0x4E => {}
            // LD C,A
            0x4F => self.reg.c = self.reg.a,
            _ => panic!("opcode not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn ld_r1_r2() {
        let mut cpu = CPU::new();
        cpu.reg.a = 10;
        assert_eq!(cpu.reg.a, 10);
        assert_eq!(cpu.reg.b, 0);
        cpu.exec(0x47);
        assert_eq!(cpu.reg.b, 10);
    }
}
