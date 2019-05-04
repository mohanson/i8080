use super::asm;
use super::bit;
use super::memory::Memory;
use super::register::{Flag, Register};
use std::mem;

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
    pub reg: Register,
    pub mem: Box<Memory>,
    halted: bool,
    ei: bool,
}

impl Cpu {
    pub fn power_up(mem: Box<Memory>) -> Self {
        Self {
            reg: Register::power_up(),
            mem,
            halted: false,
            ei: false,
        }
    }

    fn imm_ds(&mut self) -> u8 {
        let v = self.mem.get(self.reg.pc);
        self.reg.pc += 1;
        v
    }

    fn imm_dw(&mut self) -> u16 {
        let v = self.mem.get_word(self.reg.pc);
        self.reg.pc += 2;
        v
    }

    fn get_m(&self) -> u8 {
        let a = self.reg.get_hl();
        self.mem.get(a)
    }

    fn set_m(&mut self, v: u8) {
        let a = self.reg.get_hl();
        self.mem.set(a, v)
    }

    fn stack_add(&mut self, v: u16) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.mem.set_word(self.reg.sp, v);
    }

    fn stack_pop(&mut self) -> u16 {
        let r = self.mem.get_word(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(2);
        r
    }

    fn alu_inr(&mut self, n: u8) -> u8 {
        let r = n.wrapping_add(1);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (n & 0x0f) + 0x01 > 0x0f);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        r
    }

    fn alu_dcr(&mut self, n: u8) -> u8 {
        let r = n.wrapping_sub(1);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (r & 0x0f) != 0x0f);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        r
    }

    // The eight-bit hexadecimal number in the accumulator is.adjusted to form tow four bit binary codecd decimal
    // digits by the following two process
    fn alu_daa(&mut self) {
        let mut r: u8 = self.reg.a;
        // If the least significant four bits of the accumulator represents a number greater than 9, or if the Auxiliary
        // Carry bit is equal to one, the accumulator is incremented by six. Otherwise, no incrementing occurs.
        if ((r & 0x0f) > 9) || self.reg.get_flag(Flag::A) {
            r = r.wrapping_add(0x06);
            self.reg.set_flag(Flag::A, true);
        } else {
            self.reg.set_flag(Flag::A, false);
        }
        // If the most significant four bits of the accumulator now represent a number greater than 9, or if the normal
        // carry bit is equal to one, the most sign ificant four bits of the accumulator are incremented by six.
        // Otherwise, no incrementing occurs.
        if (r > 0x9f) || self.reg.get_flag(Flag::C) {
            r = r.wrapping_add(0x60);
            self.reg.set_flag(Flag::C, true);
        }
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.a = r;
    }

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

    fn alu_adc(&mut self, n: u8) {
        let c = u8::from(self.reg.get_flag(Flag::C));
        let n = n.wrapping_add(c);
        self.alu_add(n);
    }

    fn alu_sub(&mut self, n: u8) {
        let a = self.reg.a;
        let r = a.wrapping_sub(n);
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, (a & 0x0f) + (!n & 0x0f) + 1 > 0x0f);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, u16::from(a) < u16::from(n));
        self.reg.a = r;
    }

    fn alu_sbb(&mut self, n: u8) {
        let c = u8::from(self.reg.get_flag(Flag::C));
        let n = n.wrapping_add(c);
        self.alu_sub(n)
    }

    fn alu_ana(&mut self, n: u8) {
        let r = self.reg.a & n;
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, false);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, false);
        self.reg.a = r;
    }

    fn alu_xra(&mut self, n: u8) {
        let r = self.reg.a ^ n;
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, false);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, false);
        self.reg.a = r;
    }

    fn alu_ora(&mut self, n: u8) {
        let r = self.reg.a | n;
        self.reg.set_flag(Flag::S, bit::get(r, 7));
        self.reg.set_flag(Flag::Z, r == 0x00);
        self.reg.set_flag(Flag::A, false);
        self.reg.set_flag(Flag::P, r.count_ones() & 0x01 == 0x00);
        self.reg.set_flag(Flag::C, false);
        self.reg.a = r;
    }

    fn alu_cmp(&mut self, n: u8) {
        let r = self.reg.a;
        self.alu_sub(n);
        self.reg.a = r;
    }

    fn alu_rlc(&mut self) {
        let c = bit::get(self.reg.a, 7);
        let r = (self.reg.a << 1) | u8::from(c);
        self.reg.set_flag(Flag::C, c);
        self.reg.a = r;
    }

    fn alu_rrc(&mut self) {
        let c = bit::get(self.reg.a, 0);
        let r = if c { 0x80 | (self.reg.a >> 1) } else { self.reg.a >> 1 };
        self.reg.set_flag(Flag::C, c);
        self.reg.a = r;
    }

    fn alu_ral(&mut self) {
        let c = bit::get(self.reg.a, 7);
        let r = (self.reg.a << 1) | u8::from(self.reg.get_flag(Flag::C));
        self.reg.set_flag(Flag::C, c);
        self.reg.a = r;
    }

    fn alu_rar(&mut self) {
        let c = bit::get(self.reg.a, 0);
        let r = if self.reg.get_flag(Flag::C) {
            0x80 | (self.reg.a >> 1)
        } else {
            self.reg.a >> 1
        };
        self.reg.set_flag(Flag::C, c);
        self.reg.a = r;
    }

    fn alu_dad(&mut self, n: u16) {
        let a = self.reg.get_hl();
        let r = a.wrapping_add(n);
        self.reg.set_flag(Flag::C, a > 0xffff - n);
        self.reg.set_hl(r);
    }

    pub fn next(&mut self) -> u32 {
        let opcode = self.imm_ds();
        let opcode = match opcode {
            0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => 0x00,
            0xcb => 0xc3,
            0xd9 => 0xc9,
            0xdd | 0xed | 0xfd => 0xcd,
            _ => opcode,
        };

        log::debug!(
            "{} PC={:04x} SP={:04x} A={:02x} F={:02x} B={:02x} C={:02x} D={:02x} E={:02x} H={:02x} L={:02x}",
            asm::asm(opcode),
            self.reg.pc - 1,
            self.reg.sp,
            self.reg.a,
            self.reg.f,
            self.reg.b,
            self.reg.c,
            self.reg.d,
            self.reg.e,
            self.reg.h,
            self.reg.l
        );

        let mut ecycle = 0;
        match opcode {
            // CARRY BIT INSTRUCTIONS
            0x3f => self.reg.set_flag(Flag::C, !self.reg.get_flag(Flag::C)),
            0x37 => self.reg.set_flag(Flag::C, true),

            // INR Increment Register or Memory
            0x04 => self.reg.b = self.alu_inr(self.reg.b),
            0x0c => self.reg.c = self.alu_inr(self.reg.c),
            0x14 => self.reg.d = self.alu_inr(self.reg.d),
            0x1c => self.reg.e = self.alu_inr(self.reg.e),
            0x24 => self.reg.h = self.alu_inr(self.reg.h),
            0x2c => self.reg.l = self.alu_inr(self.reg.l),
            0x34 => {
                let a = self.get_m();
                let b = self.alu_inr(a);
                self.set_m(b);
            }
            0x3c => self.reg.a = self.alu_inr(self.reg.a),

            // DCR Decrement Register or Memory
            0x05 => self.reg.b = self.alu_dcr(self.reg.b),
            0x0d => self.reg.c = self.alu_dcr(self.reg.c),
            0x15 => self.reg.d = self.alu_dcr(self.reg.d),
            0x1d => self.reg.e = self.alu_dcr(self.reg.e),
            0x25 => self.reg.h = self.alu_dcr(self.reg.h),
            0x2d => self.reg.l = self.alu_dcr(self.reg.l),
            0x35 => {
                let a = self.get_m();
                let b = self.alu_dcr(a);
                self.set_m(b);
            }
            0x3d => self.reg.a = self.alu_dcr(self.reg.a),

            // CMA Complement Accumulator
            0x2f => self.reg.a = !self.reg.a,

            // DAA Decimal Adjust Accumulator
            0x27 => self.alu_daa(),

            // NOP INSTRUCTIONS
            0x00 => {}

            // MOV Instruction
            0x40 => {}
            0x41 => self.reg.b = self.reg.c,
            0x42 => self.reg.b = self.reg.d,
            0x43 => self.reg.b = self.reg.e,
            0x44 => self.reg.b = self.reg.h,
            0x45 => self.reg.b = self.reg.l,
            0x46 => self.reg.b = self.get_m(),
            0x47 => self.reg.b = self.reg.a,
            0x48 => self.reg.c = self.reg.b,
            0x49 => {}
            0x4a => self.reg.c = self.reg.d,
            0x4b => self.reg.c = self.reg.e,
            0x4c => self.reg.c = self.reg.h,
            0x4d => self.reg.c = self.reg.l,
            0x4e => self.reg.c = self.get_m(),
            0x4f => self.reg.c = self.reg.a,
            0x50 => self.reg.d = self.reg.b,
            0x51 => self.reg.d = self.reg.c,
            0x52 => {}
            0x53 => self.reg.d = self.reg.e,
            0x54 => self.reg.d = self.reg.h,
            0x55 => self.reg.d = self.reg.l,
            0x56 => self.reg.d = self.get_m(),
            0x57 => self.reg.d = self.reg.a,
            0x58 => self.reg.e = self.reg.b,
            0x59 => self.reg.e = self.reg.c,
            0x5a => self.reg.e = self.reg.d,
            0x5b => {}
            0x5c => self.reg.e = self.reg.h,
            0x5d => self.reg.e = self.reg.l,
            0x5e => self.reg.e = self.get_m(),
            0x5f => self.reg.e = self.reg.a,
            0x60 => self.reg.h = self.reg.b,
            0x61 => self.reg.h = self.reg.c,
            0x62 => self.reg.h = self.reg.d,
            0x63 => self.reg.h = self.reg.e,
            0x64 => {}
            0x65 => self.reg.h = self.reg.l,
            0x66 => self.reg.h = self.get_m(),
            0x67 => self.reg.h = self.reg.a,
            0x68 => self.reg.l = self.reg.b,
            0x69 => self.reg.l = self.reg.c,
            0x6a => self.reg.l = self.reg.d,
            0x6b => self.reg.l = self.reg.e,
            0x6c => self.reg.l = self.reg.h,
            0x6d => {}
            0x6e => self.reg.l = self.get_m(),
            0x6f => self.reg.l = self.reg.a,
            0x70 => self.set_m(self.reg.b),
            0x71 => self.set_m(self.reg.c),
            0x72 => self.set_m(self.reg.d),
            0x73 => self.set_m(self.reg.e),
            0x74 => self.set_m(self.reg.h),
            0x75 => self.set_m(self.reg.l),
            0x77 => self.set_m(self.reg.a),
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7a => self.reg.a = self.reg.d,
            0x7b => self.reg.a = self.reg.e,
            0x7c => self.reg.a = self.reg.h,
            0x7d => self.reg.a = self.reg.l,
            0x7e => self.reg.a = self.get_m(),
            0x7f => {}

            // STAX Store Accumulator
            0x02 => self.mem.set(self.reg.get_bc(), self.reg.a),
            0x12 => self.mem.set(self.reg.get_de(), self.reg.a),

            // LDAX Load Accumulator
            0x0a => self.reg.a = self.mem.get(self.reg.get_bc()),
            0x1a => self.reg.a = self.mem.get(self.reg.get_de()),

            // ADD ADD Register or Memory To Accumulator
            0x80 => self.alu_add(self.reg.b),
            0x81 => self.alu_add(self.reg.c),
            0x82 => self.alu_add(self.reg.d),
            0x83 => self.alu_add(self.reg.e),
            0x84 => self.alu_add(self.reg.h),
            0x85 => self.alu_add(self.reg.l),
            0x86 => self.alu_add(self.get_m()),
            0x87 => self.alu_add(self.reg.a),

            // ADC ADD Register or Memory To Accumulator With Carry
            0x88 => self.alu_adc(self.reg.b),
            0x89 => self.alu_adc(self.reg.c),
            0x8a => self.alu_adc(self.reg.d),
            0x8b => self.alu_adc(self.reg.e),
            0x8c => self.alu_adc(self.reg.h),
            0x8d => self.alu_adc(self.reg.l),
            0x8e => self.alu_adc(self.get_m()),
            0x8f => self.alu_adc(self.reg.a),

            // SUB Subtract Register or Memory From Accumulator
            0x90 => self.alu_sub(self.reg.b),
            0x91 => self.alu_sub(self.reg.c),
            0x92 => self.alu_sub(self.reg.d),
            0x93 => self.alu_sub(self.reg.e),
            0x94 => self.alu_sub(self.reg.h),
            0x95 => self.alu_sub(self.reg.l),
            0x96 => self.alu_sub(self.get_m()),
            0x97 => self.alu_sub(self.reg.a),

            // SBB Subtract Register or Memory From Accumulator With Borrow
            0x98 => self.alu_sbb(self.reg.b),
            0x99 => self.alu_sbb(self.reg.c),
            0x9a => self.alu_sbb(self.reg.d),
            0x9b => self.alu_sbb(self.reg.e),
            0x9c => self.alu_sbb(self.reg.h),
            0x9d => self.alu_sbb(self.reg.l),
            0x9e => self.alu_sbb(self.get_m()),
            0x9f => self.alu_sbb(self.reg.a),

            // ANA Logical and Register or Memory With Accumulator
            0xa0 => self.alu_ana(self.reg.b),
            0xa1 => self.alu_ana(self.reg.c),
            0xa2 => self.alu_ana(self.reg.d),
            0xa3 => self.alu_ana(self.reg.e),
            0xa4 => self.alu_ana(self.reg.h),
            0xa5 => self.alu_ana(self.reg.l),
            0xa6 => self.alu_ana(self.get_m()),
            0xa7 => self.alu_ana(self.reg.a),

            // XRA Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
            0xa8 => self.alu_xra(self.reg.b),
            0xa9 => self.alu_xra(self.reg.c),
            0xaa => self.alu_xra(self.reg.d),
            0xab => self.alu_xra(self.reg.e),
            0xac => self.alu_xra(self.reg.h),
            0xad => self.alu_xra(self.reg.l),
            0xae => self.alu_xra(self.get_m()),
            0xaf => self.alu_xra(self.reg.a),

            // ORA Logical or Register or Memory With Accumulator
            0xb0 => self.alu_ora(self.reg.b),
            0xb1 => self.alu_ora(self.reg.c),
            0xb2 => self.alu_ora(self.reg.d),
            0xb3 => self.alu_ora(self.reg.e),
            0xb4 => self.alu_ora(self.reg.h),
            0xb5 => self.alu_ora(self.reg.l),
            0xb6 => self.alu_ora(self.get_m()),
            0xb7 => self.alu_ora(self.reg.a),

            // CMP Compare Register or Memory With Accumulator
            0xb8 => self.alu_cmp(self.reg.b),
            0xb9 => self.alu_cmp(self.reg.c),
            0xba => self.alu_cmp(self.reg.d),
            0xbb => self.alu_cmp(self.reg.e),
            0xbc => self.alu_cmp(self.reg.h),
            0xbd => self.alu_cmp(self.reg.l),
            0xbe => self.alu_cmp(self.get_m()),
            0xbf => self.alu_cmp(self.reg.a),

            // RLC Rotate Accumulator Left
            0x07 => self.alu_rlc(),

            // RRC Rotate Accumulator Right
            0x0f => self.alu_rrc(),

            // RAL Rotate Accumulator Left Through Carry
            0x17 => self.alu_ral(),

            // RAR Rotate Accumulator Right Through Carry
            0x1f => self.alu_rar(),

            // PUSH Push Data Onto Stack
            0xc5 => self.stack_add(self.reg.get_bc()),
            0xd5 => self.stack_add(self.reg.get_de()),
            0xe5 => self.stack_add(self.reg.get_hl()),
            0xf5 => self.stack_add(self.reg.get_af()),

            // POP Pop Data Off Stack
            0xc1 => {
                let a = self.stack_pop();
                self.reg.set_bc(a);
            }
            0xd1 => {
                let a = self.stack_pop();
                self.reg.set_de(a);
            }
            0xe1 => {
                let a = self.stack_pop();
                self.reg.set_hl(a);
            }
            0xf1 => {
                let a = self.stack_pop();
                self.reg.set_af(a);
            }

            // DAD Double Add
            0x09 => self.alu_dad(self.reg.get_bc()),
            0x19 => self.alu_dad(self.reg.get_de()),
            0x29 => self.alu_dad(self.reg.get_hl()),
            0x39 => self.alu_dad(self.reg.sp),

            // INX Increment Register Pair
            0x03 => self.reg.set_bc(self.reg.get_bc().wrapping_add(1)),
            0x13 => self.reg.set_de(self.reg.get_de().wrapping_add(1)),
            0x23 => self.reg.set_hl(self.reg.get_hl().wrapping_add(1)),
            0x33 => self.reg.sp = self.reg.sp.wrapping_add(1),

            // DCX Decrement Register Pair
            0x0b => self.reg.set_bc(self.reg.get_bc().wrapping_sub(1)),
            0x1b => self.reg.set_de(self.reg.get_de().wrapping_sub(1)),
            0x2b => self.reg.set_hl(self.reg.get_hl().wrapping_sub(1)),
            0x3b => self.reg.sp = self.reg.sp.wrapping_sub(1),

            // XCHG Exchange Registers
            0xeb => {
                mem::swap(&mut self.reg.h, &mut self.reg.d);
                mem::swap(&mut self.reg.l, &mut self.reg.e);
            }

            // XTHL Exchange Stack
            0xe3 => {
                let a = self.mem.get_word(self.reg.sp);
                let b = self.reg.get_hl();
                self.reg.set_hl(a);
                self.mem.set_word(self.reg.sp, b)
            }

            // SPHL Load SP From H And L
            0xf9 => self.reg.sp = self.reg.get_hl(),

            // LXI Load Immediate Data
            0x01 => {
                let a = self.imm_dw();
                self.reg.set_bc(a);
            }
            0x11 => {
                let a = self.imm_dw();
                self.reg.set_de(a);
            }
            0x21 => {
                let a = self.imm_dw();
                self.reg.set_hl(a);
            }
            0x31 => {
                let a = self.imm_dw();
                self.reg.sp = a;
            }

            // MVI Move Immediate Data
            0x06 => self.reg.b = self.imm_ds(),
            0x0e => self.reg.c = self.imm_ds(),
            0x16 => self.reg.d = self.imm_ds(),
            0x1e => self.reg.e = self.imm_ds(),
            0x26 => self.reg.h = self.imm_ds(),
            0x2e => self.reg.l = self.imm_ds(),
            0x36 => {
                let a = self.imm_ds();
                self.set_m(a);
            }
            0x3e => self.reg.a = self.imm_ds(),

            // ADI Add Immediate To Accumulator
            0xc6 => {
                let a = self.imm_ds();
                self.alu_add(a);
            }

            // ACI Add Immediate To Accumulator With Carry
            0xce => {
                let a = self.imm_ds();
                self.alu_adc(a);
            }

            // SUI Subtract Immediate From Accumulator
            0xd6 => {
                let a = self.imm_ds();
                self.alu_sub(a);
            }

            // SBI Subtract Immediate from Accumulator With Borrow
            0xde => {
                let v = self.imm_ds();
                self.alu_sbb(v);
            }

            // ANI And Immediate With AccumulatorLabel
            0xe6 => {
                let a = self.imm_ds();
                self.alu_ana(a);
            }

            // XRI Exclusive-Or Immediate With Accumulator
            0xee => {
                let a = self.imm_ds();
                self.alu_xra(a);
            }

            // ORI Or Immediate With Accumulator
            0xf6 => {
                let a = self.imm_ds();
                self.alu_ora(a);
            }

            // CPI Compare Immediate With Accumulator
            0xfe => {
                let a = self.imm_ds();
                self.alu_cmp(a);
            }

            // STA Store Accumulator Direct
            0x32 => {
                let a = self.imm_dw();
                self.mem.set(a, self.reg.a);
            }

            // LDA Load Accumulator Direct
            0x3a => {
                let a = self.imm_dw();
                let b = self.mem.get(a);
                self.reg.a = b;
            }

            // SHLD Store Hand L Direct
            0x22 => {
                let a = self.imm_dw();
                self.mem.set_word(a, self.reg.get_hl());
            }

            // LHLD Load HAnd L Direct
            0x2a => {
                let a = self.imm_dw();
                let b = self.mem.get_word(a);
                self.reg.set_hl(b);
            }

            // PCHL Load Program Counter
            0xe9 => self.reg.pc = self.reg.get_hl(),

            // JUMP INSTRUCTIONS
            0xc3 | 0xda | 0xd2 | 0xca | 0xc2 | 0xfa | 0xf2 | 0xea | 0xe2 => {
                let a = self.imm_dw();
                let cond = match opcode {
                    // JMP JUMP
                    0xc3 => true,
                    // JC Jump If Carry
                    0xda => self.reg.get_flag(Flag::C),
                    // JNC Jump If No Carry
                    0xd2 => !self.reg.get_flag(Flag::C),
                    // JZ Jump If Zero
                    0xca => self.reg.get_flag(Flag::Z),
                    // JNZ Jump If Not Zero
                    0xc2 => !self.reg.get_flag(Flag::Z),
                    // JM Jump If Minus
                    0xfa => self.reg.get_flag(Flag::S),
                    // JP Jump If Positive
                    0xf2 => !self.reg.get_flag(Flag::S),
                    // JPE Jump If Parity Even
                    0xea => self.reg.get_flag(Flag::P),
                    // JPO Jump If Parity Odd
                    0xe2 => !self.reg.get_flag(Flag::P),
                    _ => unimplemented!(),
                };
                if cond {
                    self.reg.pc = a;
                }
            }

            // CALL SUBROUTINE INSTRUCTIONS
            0xcd | 0xdc | 0xd4 | 0xcc | 0xc4 | 0xfc | 0xf4 | 0xec | 0xe4 => {
                let a = self.imm_dw();
                let cond = match opcode {
                    // CALL Call
                    0xcd => true,
                    // CC Call If Carry
                    0xdc => self.reg.get_flag(Flag::C),
                    // CNC Call If No Carry
                    0xd4 => !self.reg.get_flag(Flag::C),
                    // CZ Call If Zero
                    0xcc => self.reg.get_flag(Flag::Z),
                    // CNZ Call If Not Zero
                    0xc4 => !self.reg.get_flag(Flag::Z),
                    // CM Call If Minus
                    0xfc => self.reg.get_flag(Flag::S),
                    // CP Call If Plus
                    0xf4 => !self.reg.get_flag(Flag::S),
                    // CPE Call If Parity Even
                    0xec => self.reg.get_flag(Flag::P),
                    // CPO Call If Parity Odd
                    0xe4 => !self.reg.get_flag(Flag::P),
                    _ => unimplemented!(),
                };
                if cond {
                    ecycle = 6;
                    self.stack_add(self.reg.pc);
                    self.reg.pc = a;
                }
            }

            // RETURN FROM SUBROUTINE INSTRUCTIONS
            0xc9 | 0xd8 | 0xd0 | 0xc8 | 0xc0 | 0xf8 | 0xf0 | 0xe8 | 0xe0 => {
                let cond = match opcode {
                    // RET Return
                    0xc9 => true,
                    // RC Return If Carry
                    0xd8 => self.reg.get_flag(Flag::C),
                    // RNC Return If No Carry
                    0xd0 => !self.reg.get_flag(Flag::C),
                    // RZ Return If Zero
                    0xc8 => self.reg.get_flag(Flag::Z),
                    // RNZ Return If Not Zero
                    0xc0 => !self.reg.get_flag(Flag::Z),
                    // RM Return If Minus
                    0xf8 => self.reg.get_flag(Flag::S),
                    // RP Return If Plus
                    0xf0 => !self.reg.get_flag(Flag::S),
                    // RPE Return If Parity Even
                    0xe8 => self.reg.get_flag(Flag::P),
                    // RPO Return If Parity Odd
                    0xe0 => !self.reg.get_flag(Flag::P),
                    _ => unimplemented!(),
                };
                if cond {
                    self.reg.pc = self.stack_pop()
                }
            }

            // RST INSTRUCTION
            0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
                self.stack_add(self.reg.pc);
                self.reg.pc = u16::from(opcode & 0x38);
            }

            // INTERRUPT FLIP-FLOP INSTRUCTIONS
            0xfb => self.ei = true,
            0xf3 => self.ei = false,

            // INPUT/OUTPUT INSTRUCTIONS
            0xdb => {
                println!("0xdb input");
            }
            0xd3 => {
                let a = self.imm_ds();
                println!("out => port={} data={}", a, self.reg.a);
            }

            // HLT HALT INSTRUCTION
            0x76 => self.halted = true,
            _ => {}
        };

        OP_CYCLES[opcode as usize] + ecycle
    }
}
