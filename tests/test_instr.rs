use i8080::{Flag, Linear, Memory};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_inr() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.c = 0x99;
    mem.borrow_mut().set(0x0000, 0x0c);
    cpu.next();
    assert_eq!(cpu.reg.c, 0x9a);
}

#[test]
fn test_dcr() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0x3a;
    cpu.reg.l = 0x7c;
    mem.borrow_mut().set(0x3a7c, 0x40);
    mem.borrow_mut().set(0x0000, 0x35);
    cpu.next();
    assert_eq!(mem.borrow().get(0x3a7c), 0x3f);
}

#[test]
fn test_cma() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x51;
    mem.borrow_mut().set(0x0000, 0x2f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xae);
}

#[test]
fn test_daa() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x9b;
    mem.borrow_mut().set(0x0000, 0x27);
    cpu.next();
    assert_eq!(cpu.reg.a, 1);
    assert!(cpu.reg.get_flag(Flag::A));
    assert!(cpu.reg.get_flag(Flag::C));
}

#[test]
fn test_mov() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xff;
    cpu.reg.h = 0x2b;
    cpu.reg.l = 0xe9;
    mem.borrow_mut().set(0x0000, 0x77);
    cpu.next();
    assert_eq!(mem.borrow().get(0x2be9), 0xff);
}

#[test]
fn test_stax() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xff;
    cpu.reg.b = 0x3f;
    cpu.reg.c = 0x16;
    mem.borrow_mut().set(0x0000, 0x02);
    cpu.next();
    assert_eq!(mem.borrow().get(0x3f16), 0xff);
}

#[test]
fn test_ldax() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.d = 0x93;
    cpu.reg.e = 0x8b;
    mem.borrow_mut().set(0x938b, 0xff);
    mem.borrow_mut().set(0x0000, 0x1a);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xff);
}

#[test]
fn test_add_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.d = 0x2e;
    cpu.reg.a = 0x6c;
    mem.borrow_mut().set(0x0000, 0x82);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x9a);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_add_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x01;
    mem.borrow_mut().set(0x0000, 0x87);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x02);
}

#[test]
fn test_adc_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x42;
    cpu.reg.c = 0x3d;
    mem.borrow_mut().set(0x0000, 0x89);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x7f);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_adc_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x42;
    cpu.reg.c = 0x3d;
    cpu.reg.set_flag(Flag::C, true);
    mem.borrow_mut().set(0x0000, 0x89);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x80);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_adc_3() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x3f;
    cpu.reg.f = 0xd3;
    mem.borrow_mut().set(0x0000, 0x8f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x7f);
    assert_eq!(cpu.reg.f, 0x12);
}

#[test]
fn test_sub() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x3e;
    mem.borrow_mut().set(0x0000, 0x97);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x00);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_sbb() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.l = 0x02;
    cpu.reg.a = 0x04;
    cpu.reg.set_flag(Flag::C, true);
    mem.borrow_mut().set(0x0000, 0x9d);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_ana() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xfc;
    cpu.reg.c = 0x0f;
    mem.borrow_mut().set(0x0000, 0xa1);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x0c);
}

#[test]
fn test_xra_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x0a;
    cpu.reg.b = 0x0b;
    cpu.reg.c = 0x0c;
    mem.borrow_mut().set(0x0000, 0xaf);
    mem.borrow_mut().set(0x0001, 0x47);
    mem.borrow_mut().set(0x0002, 0x4f);
    cpu.next();
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0x00);
    assert_eq!(cpu.reg.b, 0x00);
    assert_eq!(cpu.reg.c, 0x00);
}

#[test]
fn test_xra_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xff;
    cpu.reg.b = 0b1010_1010;
    mem.borrow_mut().set(0x0000, 0xa8);
    cpu.next();
    assert_eq!(cpu.reg.a, 0b0101_0101);
}

#[test]
fn test_xra_3() {}

#[test]
fn test_ora() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x33;
    cpu.reg.c = 0x0f;
    mem.borrow_mut().set(0x0000, 0xb1);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x3f);
}

#[test]
fn test_cmp_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x0a;
    cpu.reg.e = 0x05;
    mem.borrow_mut().set(0x0000, 0xbb);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x0a);
    assert_eq!(cpu.reg.e, 0x05);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_cmp_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x02;
    cpu.reg.e = 0x05;
    mem.borrow_mut().set(0x0000, 0xbb);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x02);
    assert_eq!(cpu.reg.e, 0x05);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_cmp_3() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xe5;
    cpu.reg.e = 0x05;
    mem.borrow_mut().set(0x0000, 0xbb);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xe5);
    assert_eq!(cpu.reg.e, 0x05);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_rlc() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xf2;
    mem.borrow_mut().set(0x0000, 0x07);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xe5);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_rrc() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xf2;
    mem.borrow_mut().set(0x0000, 0x0f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x79);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_ral() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xb5;
    mem.borrow_mut().set(0x0000, 0x17);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x6a);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_rar() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x6a;
    cpu.reg.set_flag(Flag::C, true);
    mem.borrow_mut().set(0x0000, 0x1f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xb5);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_stack_push_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.d = 0x8f;
    cpu.reg.e = 0x9d;
    cpu.reg.sp = 0x3a2c;
    mem.borrow_mut().set(0x0000, 0xd5);
    cpu.next();
    assert_eq!(mem.borrow().get(0x3a2b), 0x8f);
    assert_eq!(mem.borrow().get(0x3a2a), 0x9d);
    assert_eq!(cpu.reg.sp, 0x3a2a);
}

#[test]
fn test_stack_push_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x1f;
    cpu.reg.sp = 0x502a;
    cpu.reg.set_flag(Flag::C, true);
    cpu.reg.set_flag(Flag::Z, true);
    cpu.reg.set_flag(Flag::P, true);
    mem.borrow_mut().set(0x0000, 0xf5);
    cpu.next();
    assert_eq!(mem.borrow().get(0x5029), 0x1f);
    assert_eq!(mem.borrow().get(0x5028), 0x47);
    assert_eq!(cpu.reg.sp, 0x5028);
}

#[test]
fn test_stack_pop_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x1239, 0x3d);
    mem.borrow_mut().set(0x123a, 0x93);
    cpu.reg.sp = 0x1239;
    mem.borrow_mut().set(0x0000, 0xe1);
    cpu.next();
    assert_eq!(cpu.reg.l, 0x3d);
    assert_eq!(cpu.reg.h, 0x93);
    assert_eq!(cpu.reg.sp, 0x123b);
}

#[test]
fn test_stack_pop_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x2c00, 0xc3);
    mem.borrow_mut().set(0x2c01, 0xff);
    cpu.reg.sp = 0x2c00;
    mem.borrow_mut().set(0x0000, 0xf1);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xff);
    assert_eq!(cpu.reg.f, 0xc3);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_dad_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.b = 0x33;
    cpu.reg.c = 0x9f;
    cpu.reg.h = 0xa1;
    cpu.reg.l = 0x7b;
    mem.borrow_mut().set(0x0000, 0x09);
    cpu.next();
    assert_eq!(cpu.reg.h, 0xd5);
    assert_eq!(cpu.reg.l, 0x1a);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_dad_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0xa1;
    cpu.reg.l = 0x7b;
    mem.borrow_mut().set(0x0000, 0x29);
    cpu.next();
    assert_eq!(cpu.reg.get_hl(), 0xa17b << 1);
}

#[test]
fn test_inx_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.d = 0x38;
    cpu.reg.e = 0xff;
    mem.borrow_mut().set(0x0000, 0x13);
    cpu.next();
    assert_eq!(cpu.reg.d, 0x39);
    assert_eq!(cpu.reg.e, 0x00);
}

#[test]
fn test_inx_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.sp = 0xffff;
    mem.borrow_mut().set(0x0000, 0x33);
    cpu.next();
    assert_eq!(cpu.reg.sp, 0x0000);
}

#[test]
fn test_dcx() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0x98;
    cpu.reg.l = 0x00;
    mem.borrow_mut().set(0x0000, 0x2b);
    cpu.next();
    assert_eq!(cpu.reg.h, 0x97);
    assert_eq!(cpu.reg.l, 0xff);
}

#[test]
fn test_xchg() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0x00;
    cpu.reg.l = 0xff;
    cpu.reg.d = 0x33;
    cpu.reg.e = 0x55;
    mem.borrow_mut().set(0x0000, 0xeb);
    cpu.next();
    assert_eq!(cpu.reg.h, 0x33);
    assert_eq!(cpu.reg.l, 0x55);
    assert_eq!(cpu.reg.d, 0x00);
    assert_eq!(cpu.reg.e, 0xff);
}

#[test]
fn test_xthl() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.sp = 0x10ad;
    cpu.reg.h = 0x0b;
    cpu.reg.l = 0x3c;
    mem.borrow_mut().set(0x10ad, 0xf0);
    mem.borrow_mut().set(0x10ae, 0x0d);
    mem.borrow_mut().set(0x0000, 0xe3);
    cpu.next();
    assert_eq!(cpu.reg.h, 0x0d);
    assert_eq!(cpu.reg.l, 0xf0);
    assert_eq!(mem.borrow().get(0x10ad), 0x3c);
    assert_eq!(mem.borrow().get(0x10ae), 0x0b);
}

#[test]
fn test_sphl() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0x50;
    cpu.reg.l = 0x6c;
    mem.borrow_mut().set(0x0000, 0xf9);
    cpu.next();
    assert_eq!(cpu.reg.sp, 0x506c);
}

#[test]
fn test_mvi() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0000, 0x26);
    mem.borrow_mut().set(0x0001, 0x3c);
    mem.borrow_mut().set(0x0002, 0x2e);
    mem.borrow_mut().set(0x0003, 0xf4);
    mem.borrow_mut().set(0x0004, 0x36);
    mem.borrow_mut().set(0x0005, 0xff);
    cpu.next();
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.h, 0x3c);
    assert_eq!(cpu.reg.l, 0xf4);
    assert_eq!(mem.borrow().get(0x3cf4), 0xff);
}

#[test]
fn test_adi() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0000, 0x3e);
    mem.borrow_mut().set(0x0001, 0x14);
    mem.borrow_mut().set(0x0002, 0xc6);
    mem.borrow_mut().set(0x0003, 0x42);
    mem.borrow_mut().set(0x0004, 0xc6);
    mem.borrow_mut().set(0x0005, 0xbe);
    cpu.next();
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0x14);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_aci() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0000, 0x3e);
    mem.borrow_mut().set(0x0001, 0x56);
    mem.borrow_mut().set(0x0002, 0xce);
    mem.borrow_mut().set(0x0003, 0xbe);
    mem.borrow_mut().set(0x0004, 0xce);
    mem.borrow_mut().set(0x0005, 0x42);
    cpu.next();
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0x57);
}

#[test]
fn test_sui() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0000, 0x3e);
    mem.borrow_mut().set(0x0001, 0x00);
    mem.borrow_mut().set(0x0002, 0xd6);
    mem.borrow_mut().set(0x0003, 0x01);
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0xff);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_sbi_1() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0000, 0xde);
    mem.borrow_mut().set(0x0001, 0x01);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xff);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_sbi_2() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.set_flag(Flag::C, true);
    mem.borrow_mut().set(0x0000, 0xde);
    mem.borrow_mut().set(0x0001, 0x01);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xfe);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_ani() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.c = 0x3a;
    mem.borrow_mut().set(0x0000, 0x79);
    mem.borrow_mut().set(0x0001, 0xe6);
    mem.borrow_mut().set(0x0002, 0x0f);
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0x0a);
}

#[test]
fn test_xri() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0x3b;
    mem.borrow_mut().set(0x0000, 0xee);
    mem.borrow_mut().set(0x0001, 0x81);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xba);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_ori() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.c = 0xb5;
    mem.borrow_mut().set(0x0000, 0x79);
    mem.borrow_mut().set(0x0001, 0xf6);
    mem.borrow_mut().set(0x0002, 0x0f);
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0xbf);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_cpi() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0000, 0x3e);
    mem.borrow_mut().set(0x0001, 0x4a);
    mem.borrow_mut().set(0x0002, 0xfe);
    mem.borrow_mut().set(0x0003, 0x40);
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0x4a);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_sta() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.a = 0xff;
    mem.borrow_mut().set(0x0000, 0x32);
    mem.borrow_mut().set(0x0001, 0xb3);
    mem.borrow_mut().set(0x0002, 0x05);
    cpu.next();
    assert_eq!(mem.borrow().get(0x05b3), 0xff);
}

#[test]
fn test_lda() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x0300, 0xff);
    mem.borrow_mut().set(0x0000, 0x3a);
    mem.borrow_mut().set(0x0001, 0x00);
    mem.borrow_mut().set(0x0002, 0x03);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xff);
}

#[test]
fn test_shld() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0xae;
    cpu.reg.l = 0x29;
    mem.borrow_mut().set(0x0000, 0x22);
    mem.borrow_mut().set(0x0001, 0x0a);
    mem.borrow_mut().set(0x0002, 0x01);
    cpu.next();
    assert_eq!(mem.borrow().get(0x010a), 0x29);
    assert_eq!(mem.borrow().get(0x010b), 0xae);
}

#[test]
fn test_lhld() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    mem.borrow_mut().set(0x025b, 0xff);
    mem.borrow_mut().set(0x025c, 0x03);
    mem.borrow_mut().set(0x0000, 0x2a);
    mem.borrow_mut().set(0x0001, 0x5b);
    mem.borrow_mut().set(0x0002, 0x02);
    cpu.next();
    assert_eq!(cpu.reg.l, 0xff);
    assert_eq!(cpu.reg.h, 0x03);
}

#[test]
fn test_pchl() {
    let mem = Rc::new(RefCell::new(Linear::new()));
    let mut cpu = i8080::Cpu::power_up(mem.clone());
    cpu.reg.h = 0x41;
    cpu.reg.l = 0x3e;
    mem.borrow_mut().set(0x0000, 0xe9);
    cpu.next();
    assert_eq!(cpu.reg.pc, 0x413e);
}
