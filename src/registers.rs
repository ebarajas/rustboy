pub struct Registers {
    gen: [u8; 8],
    sp: u16,
    pc: u16,
}

impl Registers {
    fn get(&self, index: usize) -> u8 {
        self.gen[index]
    }

    fn set(&mut self, index: usize, val: u8) {
        self.gen[index] = val
    }

    pub fn a(&self) -> u8 {
        self.get(0)
    }

    pub fn b(&self) -> u8 {
        self.get(1)
    }

    pub fn c(&self) -> u8 {
        self.get(2)
    }

    pub fn d(&self) -> u8 {
        self.get(3)
    }

    pub fn e(&self) -> u8 {
        self.get(4)
    }

    pub fn f(&self) -> u8 {
        self.get(5)
    }

    pub fn h(&self) -> u8 {
        self.get(6)
    }

    pub fn l(&self) -> u8 {
        self.get(7)
    }

    pub fn af(&self) -> u16 {
        (self.a() as u16) << 8 | (self.f() as u16)
    }

    pub fn bc(&self) -> u16 {
        (self.b() as u16) << 8 | (self.c() as u16)
    }

    pub fn de(&self) -> u16 {
        (self.d() as u16) << 8 | (self.e() as u16)
    }

    pub fn hl(&self) -> u16 {
        (self.h() as u16) << 8 | (self.l() as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    #[test]
    fn combined_registers() {
        let registers = Registers {
            gen: [
                0b00000000, // A
                0b00110011, // B
                0b00010001, // C
                0b00001111, // D
                0b11110000, // E
                0b00000000, // F
                0b01010101, // H
                0b10101010, // L
            ],
            sp: 0x00,
            pc: 0x00,
        };

        assert_eq!(registers.bc(), 0b0011001100010001);
        assert_eq!(registers.de(), 0b0000111111110000);
        assert_eq!(registers.hl(), 0b0101010110101010);
    }
}
