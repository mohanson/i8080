use super::bit;

// -------------
// | A   Flags |  ---> Program Status Word
// | B       C |  ---> B
// | D       E |  ---> D
// | H       L |  ---> H
// |    SP     |  ---> Stack Pointer
// |    PC     |  ---> Program Counter
// -------------
#[derive(Default)]
pub struct Register {
    pub a: u8,
    pub f: u8, // The F register is indirectly accessible by the programer.
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

// Some instructions, however, allow you to use the registers A,B,C,D,E,H,L as 16-bit registers by pairing them up
// in the following manner: AF,BC,DE,HL.
impl Register {
    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = (v & 0x00d5 | 0x0002) as u8;
    }

    pub fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as u8;
        self.c = (v & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, v: u16) {
        self.d = (v >> 8) as u8;
        self.e = (v & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as u8;
        self.l = (v & 0x00ff) as u8;
    }
}

pub enum Flag {
    S = 7, // Sign Flag
    Z = 6, // Zero Flag
    A = 4, // Also called AC, Auxiliary Carry Flag
    P = 2, // Parity Flag
    C = 0, // Carry Flag
}

impl Register {
    pub fn get_flag(&self, f: Flag) -> bool {
        bit::get(self.f, f as usize)
    }

    pub fn set_flag(&mut self, f: Flag, v: bool) {
        if v {
            self.f = bit::set(self.f, f as usize)
        } else {
            self.f = bit::clr(self.f, f as usize)
        }
    }
}

impl Register {
    pub fn power_up() -> Self {
        Self { f: 0b0000_0010, ..Default::default() }
    }
}
