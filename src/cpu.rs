use super::bit;
use super::memory::Memory;
use super::register::{Flag, Register};

//  0   1   2   3   4   5   6   7   8   9   a   b   c   d   e   f
const OP_CYCLES: [u32; 256] = [
    04, 10, 07, 05, 05, 05, 07, 04, 04, 10, 07, 05, 05, 05, 07, 04, // 0
    04, 10, 07, 05, 05, 05, 07, 04, 04, 10, 07, 05, 05, 05, 07, 04, // 1
    04, 10, 16, 05, 05, 05, 07, 04, 04, 10, 16, 05, 05, 05, 07, 04, // 2
    04, 10, 13, 05, 10, 10, 10, 04, 04, 10, 13, 05, 05, 05, 07, 04, // 3
    05, 05, 05, 05, 05, 05, 07, 05, 05, 05, 05, 05, 05, 05, 07, 05, // 4
    05, 05, 05, 05, 05, 05, 07, 05, 05, 05, 05, 05, 05, 05, 07, 05, // 5
    05, 05, 05, 05, 05, 05, 07, 05, 05, 05, 05, 05, 05, 05, 07, 05, // 6
    07, 07, 07, 07, 07, 07, 07, 07, 05, 05, 05, 05, 05, 05, 07, 05, // 7
    04, 04, 04, 04, 04, 04, 07, 04, 04, 04, 04, 04, 04, 04, 07, 04, // 8
    04, 04, 04, 04, 04, 04, 07, 04, 04, 04, 04, 04, 04, 04, 07, 04, // 9
    04, 04, 04, 04, 04, 04, 07, 04, 04, 04, 04, 04, 04, 04, 07, 04, // a
    04, 04, 04, 04, 04, 04, 07, 04, 04, 04, 04, 04, 04, 04, 07, 04, // b
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, // c
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, // d
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, // e
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, // f
];

pub struct Cpu {
    reg: Register,
    halted: bool,
}

impl Cpu {
    pub fn power_up() -> Self {
        Self {
            reg: Register::power_up(),
            halted: false,
        }
    }

    fn imm_db(&mut self, mem: &mut Memory) -> u8 {
        let v = mem.get(self.reg.pc);
        self.reg.pc += 1;
        v
    }

    fn imm_dw(&mut self, mem: &mut Memory) -> u16 {
        let v = mem.get_word(self.reg.pc);
        self.reg.pc += 2;
        v
    }

    // Increment register n.
    // n = A,B,C,D,E,H,L,(HL)
    fn alu_inr(&mut self, n: u8) -> u8 {
        let r = n.wrapping_add(1);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (n & 0x0f) + 0x01 > 0x0f);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        r
    }

    // Decrement register n.
    // n = A,B,C,D,E,H,L,(HL)
    fn alu_dcr(&mut self, n: u8) -> u8 {
        let r = n.wrapping_sub(1);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, n.trailing_zeros() >= 4);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        r
    }

    // Rotate A left. Old bit 7 to Carry flag.
    fn alu_rlc(&mut self, n: u8) -> u8 {
        let c = bit::get(n, 7);
        let r = (n << 1) | u8::from(c);
        self.reg.set_flag(Flag::C, c);
        r
    }

    // Rotate A left through Carry flag.
    fn alu_ral(&mut self, n: u8) -> u8 {
        let c = bit::get(n, 7);
        let r = (n << 1) | u8::from(self.reg.get_flag(Flag::C));
        self.reg.set_flag(Flag::C, c);
        r
    }

    // Decimal adjust register A. This instruction adjusts register A so that the correct representation of Binary
    // Coded Decimal (BCD) is obtained.
    fn alu_daa(&mut self) {
        if ((self.reg.a & 0x0f) > 9) || self.reg.get_flag(Flag::A) {
            self.reg.a += 0x06;
            self.reg.set_flag(Flag::A, true);
        } else {
            self.reg.set_flag(Flag::A, false);
        }
        if (self.reg.a > 0x9f) || self.reg.get_flag(Flag::C) {
            self.reg.a += 0x60;
            self.reg.set_flag(Flag::C, true);
        } else {
            self.reg.set_flag(Flag::C, false);
        }
        self.reg.set_flag(Flag::S, bit::get(self.reg.a, 7));
        self.reg.set_flag(Flag::Z, self.reg.a == 0x00);
    }

    // Add n to HL
    // n = BC,DE,HL,SP
    fn alu_dad(&mut self, n: u16) {
        let a = self.reg.get_hl();
        let r = a.wrapping_add(n);
        self.reg.set_flag(Flag::C, a > 0xffff - n);
        self.reg.set_hl(r);
    }

    // Rotate A right. Old bit 0 to Carry flag.
    fn alu_rrc(&mut self, n: u8) -> u8 {
        let c = bit::get(n, 0);
        let r = if c { 0x80 | (n >> 1) } else { n >> 1 };
        self.reg.set_flag(Flag::C, c);
        r
    }

    // Rotate A right through Carry flag.
    fn alu_rar(&mut self, n: u8) -> u8 {
        let c = bit::get(n, 0);
        let r = if self.reg.get_flag(Flag::C) {
            0x80 | (n >> 1)
        } else {
            n >> 1
        };
        self.reg.set_flag(Flag::C, c);
        r
    }

    // Add n to A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_add(&mut self, n: u8) {
        let a = self.reg.a;
        let r = a.wrapping_add(n);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (a & 0x0f) + (n & 0x0f) > 0x0f);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, u16::from(a) + u16::from(n) > 0xff);
        self.reg.a = r;
    }

    // Add n + Carry flag to A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_adc(&mut self, n: u8) {
        let a = self.reg.a;
        let c = u8::from(self.reg.get_flag(Flag::C));
        let r = a.wrapping_add(n).wrapping_add(c);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (a & 0x0f) + (n & 0x0f) + (c & 0x0f) > 0x0f);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg
            .set_flag(Flag::C, u16::from(a) + u16::from(n) + u16::from(c) > 0xff);
        self.reg.a = r;
    }

    // Subtract n from A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_sub(&mut self, n: u8) {
        let a = self.reg.a;
        let r = a.wrapping_sub(n);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (a & 0x0f) < (n & 0x0f));
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, u16::from(a) < u16::from(n));
        self.reg.a = r;
    }

    // Subtract n + Carry flag from A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_sbb(&mut self, n: u8) {
        let a = self.reg.a;
        let c = u8::from(self.reg.get_flag(Flag::C));
        let r = a.wrapping_sub(n).wrapping_sub(c);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (a & 0x0f) < (n & 0x0f) + c);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, u16::from(a) < u16::from(n) + u16::from(c));
        self.reg.a = r;
    }

    // Logically AND n with A, result in A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_ana(&mut self, n: u8) {
        let r = self.reg.a & n;
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, false);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, false);
        self.reg.a = r;
    }

    // Logical exclusive OR n with register A, result in A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_xra(&mut self, n: u8) {
        let r = self.reg.a ^ n;
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, false);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, false);
        self.reg.a = r;
    }

    // Logical OR n with register A, result in A.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_ora(&mut self, n: u8) {
        let r = self.reg.a | n;
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, false);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, false);
        self.reg.a = r;
    }

    // Compare A with n. This is basically an A - n subtraction instruction but the results are thrown away.
    // n = A,B,C,D,E,H,L,(HL),#
    fn alu_cmp(&mut self, n: u8) {
        let r = self.reg.a;
        self.alu_sub(n);
        self.reg.a = r;
    }

    pub fn next(&mut self, mem: &mut Memory) -> u32 {
        let opcode = self.imm_db(mem);
        match opcode {
            0x00 => {}
            0x01 => {
                let a = self.imm_dw(mem);
                self.reg.set_bc(a);
            }
            0x02 => mem.set(self.reg.get_bc(), self.reg.a),
            0x03 => {
                let a = self.reg.get_bc().wrapping_add(1);
                self.reg.set_bc(a);
            }
            0x04 => self.reg.b = self.alu_inr(self.reg.b),
            0x05 => self.reg.b = self.alu_dcr(self.reg.b),
            0x06 => self.reg.b = self.imm_db(mem),
            0x07 => self.reg.a = self.alu_rlc(self.reg.a),
            0x08 => {}
            0x09 => self.alu_dad(self.reg.get_bc()),
            0x0a => self.reg.a = mem.get(self.reg.get_bc()),
            0x0b => {
                let a = self.reg.get_bc().wrapping_sub(1);
                self.reg.set_bc(a);
            }
            0x0c => self.reg.c = self.alu_inr(self.reg.c),
            0x0d => self.reg.c = self.alu_dcr(self.reg.c),
            0x0e => self.reg.c = self.imm_db(mem),
            0x0f => self.reg.a = self.alu_rrc(self.reg.a),
            0x10 => {}
            0x11 => {
                let a = self.imm_dw(mem);
                self.reg.set_de(a);
            }
            0x12 => mem.set(self.reg.get_de(), self.reg.a),
            0x13 => {
                let a = self.reg.get_de().wrapping_add(1);
                self.reg.set_de(a);
            }
            0x14 => self.reg.d = self.alu_inr(self.reg.d),
            0x15 => self.reg.d = self.alu_dcr(self.reg.d),
            0x16 => self.reg.d = self.imm_db(mem),
            0x17 => self.reg.a = self.alu_ral(self.reg.a),
            0x18 => {}
            0x19 => self.alu_dad(self.reg.get_de()),
            0x1a => self.reg.a = mem.get(self.reg.get_de()),
            0x1b => {
                let a = self.reg.get_de().wrapping_sub(1);
                self.reg.set_de(a);
            }
            0x1c => self.reg.e = self.alu_inr(self.reg.e),
            0x1d => self.reg.e = self.alu_dcr(self.reg.e),
            0x1e => self.reg.e = self.imm_db(mem),
            0x1f => self.reg.a = self.alu_rar(self.reg.a),
            0x20 => {}
            0x21 => {
                let a = self.imm_dw(mem);
                self.reg.set_hl(a);
            }
            0x22 => {
                let a = self.imm_dw(mem);
                mem.set_word(a, self.reg.get_hl());
            }
            0x23 => {
                let v = self.reg.get_hl().wrapping_add(1);
                self.reg.set_hl(v);
            }
            0x24 => self.reg.h = self.alu_inr(self.reg.h),
            0x25 => self.reg.h = self.alu_dcr(self.reg.h),
            0x26 => self.reg.h = self.imm_db(mem),
            0x27 => self.alu_daa(),
            0x28 => {}
            0x29 => self.alu_dad(self.reg.get_hl()),
            0x2a => {
                let a = self.imm_dw(mem);
                let b = mem.get_word(a);
                self.reg.set_hl(b);
            }
            0x2b => {
                let a = self.reg.get_hl().wrapping_sub(1);
                self.reg.set_hl(a);
            }
            0x2c => self.reg.l = self.alu_inr(self.reg.l),
            0x2d => self.reg.l = self.alu_dcr(self.reg.l),
            0x2e => self.reg.l = self.imm_db(mem),
            0x2f => self.reg.a = !self.reg.a,
            0x30 => {}
            0x31 => {
                let a = self.imm_dw(mem);
                self.reg.sp = a;
            }
            0x32 => {
                let a = self.imm_dw(mem);
                mem.set(a, self.reg.a);
            }
            0x33 => {
                let a = self.reg.sp.wrapping_add(1);
                self.reg.sp = a;
            }
            0x34 => {
                let a = self.reg.get_hl();
                let b = mem.get(a);
                mem.set(a, self.alu_inr(b));
            }
            0x35 => {
                let a = self.reg.get_hl();
                let b = mem.get(a);
                mem.set(a, self.alu_dcr(b));
            }
            0x36 => {
                let a = self.reg.get_hl();
                let b = self.imm_db(mem);
                mem.set(a, b);
            }
            0x37 => self.reg.set_flag(Flag::C, true),
            0x38 => {}
            0x39 => self.alu_dad(self.reg.sp),
            0x3a => {
                let a = self.imm_dw(mem);
                let b = mem.get(a);
                self.reg.a = b;
            }
            0x3b => {
                let a = self.reg.sp.wrapping_sub(1);
                self.reg.sp = a
            }
            0x3c => self.reg.a = self.alu_inr(self.reg.a),
            0x3d => self.reg.a = self.alu_dcr(self.reg.a),
            0x3e => self.reg.a = self.imm_db(mem),
            0x3f => self.reg.set_flag(Flag::C, !self.reg.get_flag(Flag::C)),
            0x40 => {}
            0x41 => self.reg.b = self.reg.c,
            0x42 => self.reg.b = self.reg.d,
            0x43 => self.reg.b = self.reg.e,
            0x44 => self.reg.b = self.reg.h,
            0x45 => self.reg.b = self.reg.l,
            0x46 => self.reg.b = mem.get(self.reg.get_hl()),
            0x47 => self.reg.b = self.reg.a,
            0x48 => self.reg.c = self.reg.b,
            0x49 => {}
            0x4a => self.reg.c = self.reg.d,
            0x4b => self.reg.c = self.reg.e,
            0x4c => self.reg.c = self.reg.h,
            0x4d => self.reg.c = self.reg.l,
            0x4e => self.reg.c = mem.get(self.reg.get_hl()),
            0x4f => self.reg.c = self.reg.a,
            0x50 => self.reg.d = self.reg.b,
            0x51 => self.reg.d = self.reg.c,
            0x52 => {}
            0x53 => self.reg.d = self.reg.e,
            0x54 => self.reg.d = self.reg.h,
            0x55 => self.reg.d = self.reg.l,
            0x56 => self.reg.d = mem.get(self.reg.get_hl()),
            0x57 => self.reg.d = self.reg.a,
            0x58 => self.reg.e = self.reg.b,
            0x59 => self.reg.e = self.reg.c,
            0x5a => self.reg.e = self.reg.d,
            0x5b => {}
            0x5c => self.reg.e = self.reg.h,
            0x5d => self.reg.e = self.reg.l,
            0x5e => self.reg.e = mem.get(self.reg.get_hl()),
            0x5f => self.reg.e = self.reg.a,
            0x60 => self.reg.h = self.reg.b,
            0x61 => self.reg.h = self.reg.c,
            0x62 => self.reg.h = self.reg.d,
            0x63 => self.reg.h = self.reg.e,
            0x64 => {}
            0x65 => self.reg.h = self.reg.l,
            0x66 => self.reg.h = mem.get(self.reg.get_hl()),
            0x67 => self.reg.h = self.reg.a,
            0x68 => self.reg.l = self.reg.b,
            0x69 => self.reg.l = self.reg.c,
            0x6a => self.reg.l = self.reg.d,
            0x6b => self.reg.l = self.reg.e,
            0x6c => self.reg.l = self.reg.h,
            0x6d => {}
            0x6e => self.reg.l = mem.get(self.reg.get_hl()),
            0x6f => self.reg.l = self.reg.a,
            0x70 => mem.set(self.reg.get_hl(), self.reg.b),
            0x71 => mem.set(self.reg.get_hl(), self.reg.c),
            0x72 => mem.set(self.reg.get_hl(), self.reg.d),
            0x73 => mem.set(self.reg.get_hl(), self.reg.e),
            0x74 => mem.set(self.reg.get_hl(), self.reg.h),
            0x75 => mem.set(self.reg.get_hl(), self.reg.l),
            0x76 => self.halted = true,
            0x77 => mem.set(self.reg.get_hl(), self.reg.a),
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7a => self.reg.a = self.reg.d,
            0x7b => self.reg.a = self.reg.e,
            0x7c => self.reg.a = self.reg.h,
            0x7d => self.reg.a = self.reg.l,
            0x7e => self.reg.a = mem.get(self.reg.get_hl()),
            0x7f => {}
            0x80 => self.alu_add(self.reg.b),
            0x81 => self.alu_add(self.reg.c),
            0x82 => self.alu_add(self.reg.d),
            0x83 => self.alu_add(self.reg.e),
            0x84 => self.alu_add(self.reg.h),
            0x85 => self.alu_add(self.reg.l),
            0x86 => self.alu_add(mem.get(self.reg.get_hl())),
            0x87 => self.alu_add(self.reg.a),
            0x88 => self.alu_adc(self.reg.b),
            0x89 => self.alu_adc(self.reg.c),
            0x8a => self.alu_adc(self.reg.d),
            0x8b => self.alu_adc(self.reg.e),
            0x8c => self.alu_adc(self.reg.h),
            0x8d => self.alu_adc(self.reg.l),
            0x8e => self.alu_adc(mem.get(self.reg.get_hl())),
            0x8f => self.alu_adc(self.reg.a),
            0x90 => self.alu_sub(self.reg.b),
            0x91 => self.alu_sub(self.reg.c),
            0x92 => self.alu_sub(self.reg.d),
            0x93 => self.alu_sub(self.reg.e),
            0x94 => self.alu_sub(self.reg.h),
            0x95 => self.alu_sub(self.reg.l),
            0x96 => self.alu_sub(mem.get(self.reg.get_hl())),
            0x97 => self.alu_sub(self.reg.a),
            0x98 => self.alu_sbb(self.reg.b),
            0x99 => self.alu_sbb(self.reg.c),
            0x9a => self.alu_sbb(self.reg.d),
            0x9b => self.alu_sbb(self.reg.e),
            0x9c => self.alu_sbb(self.reg.h),
            0x9d => self.alu_sbb(self.reg.l),
            0x9e => self.alu_sbb(mem.get(self.reg.get_hl())),
            0x9f => self.alu_sbb(self.reg.a),
            0xa0 => self.alu_ana(self.reg.b),
            0xa1 => self.alu_ana(self.reg.c),
            0xa2 => self.alu_ana(self.reg.d),
            0xa3 => self.alu_ana(self.reg.e),
            0xa4 => self.alu_ana(self.reg.h),
            0xa5 => self.alu_ana(self.reg.l),
            0xa6 => self.alu_ana(mem.get(self.reg.get_hl())),
            0xa7 => self.alu_ana(self.reg.a),
            0xa8 => self.alu_xra(self.reg.b),
            0xa9 => self.alu_xra(self.reg.c),
            0xaa => self.alu_xra(self.reg.d),
            0xab => self.alu_xra(self.reg.e),
            0xac => self.alu_xra(self.reg.h),
            0xad => self.alu_xra(self.reg.l),
            0xae => self.alu_xra(mem.get(self.reg.get_hl())),
            0xaf => self.alu_xra(self.reg.a),
            0xb0 => self.alu_ora(self.reg.b),
            0xb1 => self.alu_ora(self.reg.c),
            0xb2 => self.alu_ora(self.reg.d),
            0xb3 => self.alu_ora(self.reg.e),
            0xb4 => self.alu_ora(self.reg.h),
            0xb5 => self.alu_ora(self.reg.l),
            0xb6 => self.alu_ora(mem.get(self.reg.get_hl())),
            0xb7 => self.alu_ora(self.reg.a),
            0xb8 => self.alu_cmp(self.reg.b),
            0xb9 => self.alu_cmp(self.reg.c),
            0xba => self.alu_cmp(self.reg.d),
            0xbb => self.alu_cmp(self.reg.e),
            0xbc => self.alu_cmp(self.reg.h),
            0xbd => self.alu_cmp(self.reg.l),
            0xbe => self.alu_cmp(mem.get(self.reg.get_hl())),
            0xbf => self.alu_cmp(self.reg.a),
            0xc0 => unimplemented!(),
            0xc1 => unimplemented!(),
            0xc2 => unimplemented!(),
            0xc3 => unimplemented!(),
            0xc4 => unimplemented!(),
            0xc5 => unimplemented!(),
            0xc6 => unimplemented!(),
            0xc7 => unimplemented!(),
            0xc8 => unimplemented!(),
            0xc9 => unimplemented!(),
            0xca => unimplemented!(),
            0xcb => unimplemented!(),
            0xcc => unimplemented!(),
            0xcd => unimplemented!(),
            0xce => unimplemented!(),
            0xcf => unimplemented!(),
            0xd0 => unimplemented!(),
            0xd1 => unimplemented!(),
            0xd2 => unimplemented!(),
            0xd3 => unimplemented!(),
            0xd4 => unimplemented!(),
            0xd5 => unimplemented!(),
            0xd6 => unimplemented!(),
            0xd7 => unimplemented!(),
            0xd8 => unimplemented!(),
            0xd9 => unimplemented!(),
            0xda => unimplemented!(),
            0xdb => unimplemented!(),
            0xdc => unimplemented!(),
            0xdd => unimplemented!(),
            0xde => unimplemented!(),
            0xdf => unimplemented!(),
            0xe0 => unimplemented!(),
            0xe1 => unimplemented!(),
            0xe2 => unimplemented!(),
            0xe3 => unimplemented!(),
            0xe4 => unimplemented!(),
            0xe5 => unimplemented!(),
            0xe6 => unimplemented!(),
            0xe7 => unimplemented!(),
            0xe8 => unimplemented!(),
            0xe9 => unimplemented!(),
            0xea => unimplemented!(),
            0xeb => unimplemented!(),
            0xec => unimplemented!(),
            0xed => unimplemented!(),
            0xee => unimplemented!(),
            0xef => unimplemented!(),
            0xf0 => unimplemented!(),
            0xf1 => unimplemented!(),
            0xf2 => unimplemented!(),
            0xf3 => unimplemented!(),
            0xf4 => unimplemented!(),
            0xf5 => unimplemented!(),
            0xf6 => unimplemented!(),
            0xf7 => unimplemented!(),
            0xf8 => unimplemented!(),
            0xf9 => unimplemented!(),
            0xfa => unimplemented!(),
            0xfb => unimplemented!(),
            0xfc => unimplemented!(),
            0xfd => unimplemented!(),
            0xfe => unimplemented!(),
            0xff => unimplemented!(),
        };
        OP_CYCLES[opcode as usize]
    }
}
