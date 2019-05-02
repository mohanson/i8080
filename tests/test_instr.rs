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
