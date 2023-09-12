use crate::registers::Registers;
use crate::mmu::MMU;

struct Cpu {
    reg: Registers,
    mmu: MMU,
    pc: u8,
    sp: u8
}

impl Cpu {
    fn fetch(index: u8) -> u8 {
        let val = self.mmu.ram[index];
        self.pc += 1;
        val;
    }

    fn exec(opcode: u8) {
        match opcode {
            _ => panic!("opcode not implemented")
        }
    }
}

#[cfg(test)]
mod tests {

}