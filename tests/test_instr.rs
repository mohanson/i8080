use i8080::Flag;

mod help;

#[test]
fn test_inr() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.c = 0x99;
    cpu.mem.set(0x0000, 0x0c);
    cpu.next();
    assert_eq!(cpu.reg.c, 0x9a);
}

#[test]
fn test_dcr() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.h = 0x3a;
    cpu.reg.l = 0x7c;
    cpu.mem.set(0x3a7c, 0x40);
    cpu.mem.set(0x0000, 0x35);
    cpu.next();
    assert_eq!(cpu.mem.get(0x3a7c), 0x3f);
}

#[test]
fn test_cma() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x51;
    cpu.mem.set(0x0000, 0x2f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xae);
}

#[test]
fn test_daa() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x9b;
    cpu.mem.set(0x0000, 0x27);
    cpu.next();
    assert_eq!(cpu.reg.a, 1);
    assert!(cpu.reg.get_flag(Flag::A));
    assert!(cpu.reg.get_flag(Flag::C));
}

#[test]
fn test_mov() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xff;
    cpu.reg.h = 0x2b;
    cpu.reg.l = 0xe9;
    cpu.mem.set(0x0000, 0x77);
    cpu.next();
    assert_eq!(cpu.mem.get(0x2be9), 0xff);
}

#[test]
fn test_stax() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xff;
    cpu.reg.b = 0x3f;
    cpu.reg.c = 0x16;
    cpu.mem.set(0x0000, 0x02);
    cpu.next();
    assert_eq!(cpu.mem.get(0x3f16), 0xff);
}

#[test]
fn test_ldax() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.d = 0x93;
    cpu.reg.e = 0x8b;
    cpu.mem.set(0x938b, 0xff);
    cpu.mem.set(0x0000, 0x1a);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xff);
}

#[test]
fn test_add_0() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.d = 0x2e;
    cpu.reg.a = 0x6c;
    cpu.mem.set(0x0000, 0x82);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x9a);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
}

#[test]
fn test_add_1() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x01;
    cpu.mem.set(0x0000, 0x87);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x02);
}

#[test]
fn test_adc_0() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x42;
    cpu.reg.c = 0x3d;
    cpu.mem.set(0x0000, 0x89);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x7f);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
}

#[test]
fn test_adc_1() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x42;
    cpu.reg.c = 0x3d;
    cpu.reg.set_flag(Flag::C, true);
    cpu.mem.set(0x0000, 0x89);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x80);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::S), true);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
}

#[test]
fn test_sub() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x3e;
    cpu.mem.set(0x0000, 0x97);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x00);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), true);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
}

#[test]
fn test_sbb() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.l = 0x02;
    cpu.reg.a = 0x04;
    cpu.reg.set_flag(Flag::C, true);
    cpu.mem.set(0x0000, 0x9d);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x01);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), true);
    assert_eq!(cpu.reg.get_flag(Flag::P), false);
    assert_eq!(cpu.reg.get_flag(Flag::S), false);
}

#[test]
fn test_ana() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xfc;
    cpu.reg.c = 0x0f;
    cpu.mem.set(0x0000, 0xa1);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x0c);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::A), false);
}

#[test]
fn test_xra_0() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x0a;
    cpu.reg.b = 0x0b;
    cpu.reg.c = 0x0c;
    cpu.mem.set(0x0000, 0xaf);
    cpu.mem.set(0x0001, 0x47);
    cpu.mem.set(0x0002, 0x4f);
    cpu.next();
    cpu.next();
    cpu.next();
    assert_eq!(cpu.reg.a, 0x00);
    assert_eq!(cpu.reg.b, 0x00);
    assert_eq!(cpu.reg.c, 0x00);
}

#[test]
fn test_xra_1() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xff;
    cpu.reg.b = 0b1010_1010;
    cpu.mem.set(0x0000, 0xa8);
    cpu.next();
    assert_eq!(cpu.reg.a, 0b0101_0101);
}

#[test]
fn test_xra_2() {}

#[test]
fn test_ora() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x33;
    cpu.reg.c = 0x0f;
    cpu.mem.set(0x0000, 0xb1);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x3f);
}

#[test]
fn test_cmp_0() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x0a;
    cpu.reg.e = 0x05;
    cpu.mem.set(0x0000, 0xbb);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x0a);
    assert_eq!(cpu.reg.e, 0x05);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
}

#[test]
fn test_cmp_1() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x02;
    cpu.reg.e = 0x05;
    cpu.mem.set(0x0000, 0xbb);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x02);
    assert_eq!(cpu.reg.e, 0x05);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
}

#[test]
fn test_cmp_2() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xe5;
    cpu.reg.e = 0x05;
    cpu.mem.set(0x0000, 0xbb);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xe5);
    assert_eq!(cpu.reg.e, 0x05);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
}

#[test]
fn test_rlc() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xf2;
    cpu.mem.set(0x0000, 0x07);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xe5);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_rrc() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xf2;
    cpu.mem.set(0x0000, 0x0f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x79);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn test_ral() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0xb5;
    cpu.mem.set(0x0000, 0x17);
    cpu.next();
    assert_eq!(cpu.reg.a, 0x6a);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn test_rar() {
    let mem = Box::new(help::Memory::new());
    let mut cpu = i8080::Cpu::power_up(mem);
    cpu.reg.a = 0x6a;
    cpu.reg.set_flag(Flag::C, true);
    cpu.mem.set(0x0000, 0x1f);
    cpu.next();
    assert_eq!(cpu.reg.a, 0xb5);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}
